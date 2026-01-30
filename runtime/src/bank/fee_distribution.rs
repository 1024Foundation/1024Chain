use {
    super::Bank,
    crate::bank::CollectorFeeDetails,
    log::debug,
    solana_account::{ReadableAccount, WritableAccount},
    solana_fee::FeeFeatures,
    solana_fee_structure::FeeBudgetLimits,
    solana_pubkey::Pubkey,
    solana_reward_info::{RewardInfo, RewardType},
    solana_runtime_transaction::transaction_with_meta::TransactionWithMeta,
    solana_svm::rent_calculator::{get_account_rent_state, transition_allowed},
    solana_system_interface::program as system_program,
    std::{result::Result, sync::atomic::Ordering::Relaxed},
    thiserror::Error,
};

#[derive(Error, Debug, PartialEq)]
enum DepositFeeError {
    #[error("fee account became rent paying")]
    InvalidRentPayingAccount,
    #[error("lamport overflow")]
    LamportOverflow,
    #[error("invalid fee account owner")]
    InvalidAccountOwner,
}

#[derive(Default)]
pub struct FeeDistribution {
    deposit: u64,
    burn: u64,
}

impl FeeDistribution {
    pub fn get_deposit(&self) -> u64 {
        self.deposit
    }
}

impl Bank {
    // Distribute collected transaction fees for this slot to collector_id (= current leader).
    //
    // Each validator is incentivized to process more transactions to earn more transaction fees.
    // Transaction fees are rewarded for the computing resource utilization cost, directly
    // proportional to their actual processing power.
    //
    // collector_id is rotated according to stake-weighted leader schedule. So the opportunity of
    // earning transaction fees are fairly distributed by stake. And missing the opportunity
    // (not producing a block as a leader) earns nothing. So, being online is incentivized as a
    // form of transaction fees as well.
    pub(super) fn distribute_transaction_fee_details(&self) {
        let fee_details = self.collector_fee_details.read().unwrap();
        if fee_details.total_transaction_fee() == 0 {
            // nothing to distribute, exit early
            return;
        }

        let FeeDistribution { deposit, burn } =
            self.calculate_reward_and_burn_fee_details(&fee_details);

        let total_burn = self.deposit_or_burn_fee(deposit).saturating_add(burn);
        self.capitalization.fetch_sub(total_burn, Relaxed);
    }

    pub fn calculate_reward_for_transaction(
        &self,
        transaction: &impl TransactionWithMeta,
        fee_budget_limits: &FeeBudgetLimits,
    ) -> u64 {
        let (_last_hash, last_lamports_per_signature) =
            self.last_blockhash_and_lamports_per_signature();
        let fee_details = solana_fee::calculate_fee_details(
            transaction,
            last_lamports_per_signature == 0,
            self.fee_structure().lamports_per_signature,
            fee_budget_limits.prioritization_fee,
            FeeFeatures::from(self.feature_set.as_ref()),
        );
        let FeeDistribution {
            deposit: reward,
            burn: _,
        } = self.calculate_reward_and_burn_fee_details(&CollectorFeeDetails::from(fee_details));
        reward
    }

    pub fn calculate_reward_and_burn_fee_details(
        &self,
        fee_details: &CollectorFeeDetails,
    ) -> FeeDistribution {
        if fee_details.transaction_fee == 0 {
            return FeeDistribution::default();
        }

        let burn = fee_details.transaction_fee * self.burn_percent() / 100;
        let deposit = fee_details
            .priority_fee
            .saturating_add(fee_details.transaction_fee.saturating_sub(burn));
        FeeDistribution { deposit, burn }
    }

    /// Returns the fee burn percentage based on active Feature Gates.
    /// 
    /// 1024Chain Economic Model:
    /// - Default: 0% burn (100% to Leader)
    /// - Can be adjusted via Feature Gate activation
    /// 
    /// Priority order (highest active wins):
    /// 100% > 75% > 50% > 25% > 10% > 0%
    fn burn_percent(&self) -> u64 {
        use agave_feature_set::{
            fee_burn_10_percent, fee_burn_25_percent,
            fee_burn_50_percent, fee_burn_75_percent, fee_burn_100_percent,
        };

        // Check from highest to lowest, first active wins
        // Note: fee_burn_0_percent is not checked as it's the default
        if self.feature_set.is_active(&fee_burn_100_percent::id()) {
            100
        } else if self.feature_set.is_active(&fee_burn_75_percent::id()) {
            75
        } else if self.feature_set.is_active(&fee_burn_50_percent::id()) {
            50
        } else if self.feature_set.is_active(&fee_burn_25_percent::id()) {
            25
        } else if self.feature_set.is_active(&fee_burn_10_percent::id()) {
            10
        } else {
            // 1024Chain default: 0% burn (100% to Leader)
            // This differs from Solana's default of 50%
            0
        }
    }

    /// Attempts to deposit the given `deposit` amount into the fee collector account.
    ///
    /// Returns the original `deposit` amount if the deposit failed and must be burned, otherwise 0.
    fn deposit_or_burn_fee(&self, deposit: u64) -> u64 {
        if deposit == 0 {
            return 0;
        }

        match self.deposit_fees(&self.collector_id, deposit) {
            Ok(post_balance) => {
                self.rewards.write().unwrap().push((
                    self.collector_id,
                    RewardInfo {
                        reward_type: RewardType::Fee,
                        lamports: deposit as i64,
                        post_balance,
                        commission: None,
                    },
                ));
                0
            }
            Err(err) => {
                debug!(
                    "Burned {} lamport tx fee instead of sending to {} due to {}",
                    deposit, self.collector_id, err
                );
                datapoint_warn!(
                    "bank-burned_fee",
                    ("slot", self.slot(), i64),
                    ("num_lamports", deposit, i64),
                    ("error", err.to_string(), String),
                );
                deposit
            }
        }
    }

    // Deposits fees into a specified account and if successful, returns the new balance of that account
    fn deposit_fees(&self, pubkey: &Pubkey, fees: u64) -> Result<u64, DepositFeeError> {
        let mut account = self
            .get_account_with_fixed_root_no_cache(pubkey)
            .unwrap_or_default();

        if !system_program::check_id(account.owner()) {
            return Err(DepositFeeError::InvalidAccountOwner);
        }

        let recipient_pre_rent_state = get_account_rent_state(
            &self.rent_collector().rent,
            account.lamports(),
            account.data().len(),
        );
        let distribution = account.checked_add_lamports(fees);
        if distribution.is_err() {
            return Err(DepositFeeError::LamportOverflow);
        }

        let recipient_post_rent_state = get_account_rent_state(
            &self.rent_collector().rent,
            account.lamports(),
            account.data().len(),
        );
        let rent_state_transition_allowed =
            transition_allowed(&recipient_pre_rent_state, &recipient_post_rent_state);
        if !rent_state_transition_allowed {
            return Err(DepositFeeError::InvalidRentPayingAccount);
        }

        self.store_account(pubkey, &account);
        Ok(account.lamports())
    }
}

#[cfg(test)]
pub mod tests {
    use {
        super::*,
        crate::genesis_utils::{create_genesis_config, create_genesis_config_with_leader},
        solana_account::AccountSharedData,
        solana_pubkey as pubkey,
        solana_rent::Rent,
        solana_signer::Signer,
        std::sync::RwLock,
    };

    #[test]
    fn test_deposit_or_burn_zero_fee() {
        let genesis = create_genesis_config(0);
        let bank = Bank::new_for_tests(&genesis.genesis_config);
        assert_eq!(bank.deposit_or_burn_fee(0), 0);
    }

    #[test]
    fn test_deposit_or_burn_fee() {
        #[derive(PartialEq)]
        enum Scenario {
            Normal,
            InvalidOwner,
            RentPaying,
        }

        struct TestCase {
            scenario: Scenario,
        }

        impl TestCase {
            fn new(scenario: Scenario) -> Self {
                Self { scenario }
            }
        }

        for test_case in [
            TestCase::new(Scenario::Normal),
            TestCase::new(Scenario::InvalidOwner),
            TestCase::new(Scenario::RentPaying),
        ] {
            let mut genesis = create_genesis_config(0);
            let rent = Rent::default();
            let min_rent_exempt_balance = rent.minimum_balance(0);
            genesis.genesis_config.rent = rent; // Ensure rent is non-zero, as genesis_utils sets Rent::free by default
            let bank = Bank::new_for_tests(&genesis.genesis_config);

            let deposit = 100;
            let mut burn = 100;

            if test_case.scenario == Scenario::RentPaying {
                // ensure that account balance + collected fees will make it rent-paying
                let initial_balance = 100;
                let account = AccountSharedData::new(initial_balance, 0, &system_program::id());
                bank.store_account(bank.collector_id(), &account);
                assert!(initial_balance + deposit < min_rent_exempt_balance);
            } else if test_case.scenario == Scenario::InvalidOwner {
                // ensure that account owner is invalid and fee distribution will fail
                let account =
                    AccountSharedData::new(min_rent_exempt_balance, 0, &Pubkey::new_unique());
                bank.store_account(bank.collector_id(), &account);
            } else {
                let account =
                    AccountSharedData::new(min_rent_exempt_balance, 0, &system_program::id());
                bank.store_account(bank.collector_id(), &account);
            }

            let initial_burn = burn;
            let initial_collector_id_balance = bank.get_balance(bank.collector_id());
            burn += bank.deposit_or_burn_fee(deposit);
            let new_collector_id_balance = bank.get_balance(bank.collector_id());

            if test_case.scenario != Scenario::Normal {
                assert_eq!(initial_collector_id_balance, new_collector_id_balance);
                assert_eq!(initial_burn + deposit, burn);
                let locked_rewards = bank.rewards.read().unwrap();
                assert!(
                    locked_rewards.is_empty(),
                    "There should be no rewards distributed"
                );
            } else {
                assert_eq!(
                    initial_collector_id_balance + deposit,
                    new_collector_id_balance
                );

                assert_eq!(initial_burn, burn);

                let locked_rewards = bank.rewards.read().unwrap();
                assert_eq!(
                    locked_rewards.len(),
                    1,
                    "There should be one reward distributed"
                );

                let reward_info = &locked_rewards[0];
                assert_eq!(
                    reward_info.1.lamports, deposit as i64,
                    "The reward amount should match the expected deposit"
                );
                assert_eq!(
                    reward_info.1.reward_type,
                    RewardType::Fee,
                    "The reward type should be Fee"
                );
            }
        }
    }

    #[test]
    fn test_deposit_fees() {
        let initial_balance = 1_000_000_000;
        let genesis = create_genesis_config(initial_balance);
        let bank = Bank::new_for_tests(&genesis.genesis_config);
        let pubkey = genesis.mint_keypair.pubkey();
        let deposit_amount = 500;

        assert_eq!(
            bank.deposit_fees(&pubkey, deposit_amount),
            Ok(initial_balance + deposit_amount),
            "New balance should be the sum of the initial balance and deposit amount"
        );
    }

    #[test]
    fn test_deposit_fees_with_overflow() {
        let initial_balance = u64::MAX;
        let genesis = create_genesis_config(initial_balance);
        let bank = Bank::new_for_tests(&genesis.genesis_config);
        let pubkey = genesis.mint_keypair.pubkey();
        let deposit_amount = 500;

        assert_eq!(
            bank.deposit_fees(&pubkey, deposit_amount),
            Err(DepositFeeError::LamportOverflow),
            "Expected an error due to lamport overflow"
        );
    }

    #[test]
    fn test_deposit_fees_invalid_account_owner() {
        let initial_balance = 1000;
        let genesis = create_genesis_config_with_leader(0, &pubkey::new_rand(), initial_balance);
        let bank = Bank::new_for_tests(&genesis.genesis_config);
        let pubkey = genesis.voting_keypair.pubkey();
        let deposit_amount = 500;

        assert_eq!(
            bank.deposit_fees(&pubkey, deposit_amount),
            Err(DepositFeeError::InvalidAccountOwner),
            "Expected an error due to invalid account owner"
        );
    }

    #[test]
    fn test_deposit_fees_invalid_rent_paying() {
        let initial_balance = 0;
        let genesis = create_genesis_config(initial_balance);
        let pubkey = genesis.mint_keypair.pubkey();
        let mut genesis_config = genesis.genesis_config;
        genesis_config.rent = Rent::default(); // Ensure rent is non-zero, as genesis_utils sets Rent::free by default
        let bank = Bank::new_for_tests(&genesis_config);
        let min_rent_exempt_balance = genesis_config.rent.minimum_balance(0);

        let deposit_amount = 500;
        assert!(initial_balance + deposit_amount < min_rent_exempt_balance);

        assert_eq!(
            bank.deposit_fees(&pubkey, deposit_amount),
            Err(DepositFeeError::InvalidRentPayingAccount),
            "Expected an error due to invalid rent paying account"
        );
    }

    #[test]
    fn test_distribute_transaction_fee_details_normal() {
        use agave_feature_set::{
            fee_burn_10_percent, fee_burn_25_percent,
            fee_burn_50_percent, fee_burn_75_percent, fee_burn_100_percent,
        };

        let genesis = create_genesis_config(0);
        let mut bank = Bank::new_for_tests(&genesis.genesis_config);
        
        // Disable all fee burn features to test 1024Chain default (0% burn)
        let mut feature_set = (*bank.feature_set).clone();
        feature_set.deactivate(&fee_burn_10_percent::id());
        feature_set.deactivate(&fee_burn_25_percent::id());
        feature_set.deactivate(&fee_burn_50_percent::id());
        feature_set.deactivate(&fee_burn_75_percent::id());
        feature_set.deactivate(&fee_burn_100_percent::id());
        bank.feature_set = std::sync::Arc::new(feature_set);

        let transaction_fee = 100;
        let priority_fee = 200;
        bank.collector_fee_details = RwLock::new(CollectorFeeDetails {
            transaction_fee,
            priority_fee,
        });
        // 1024Chain default: 0% burn (all fees go to leader)
        let expected_burn = 0;
        let expected_rewards = transaction_fee - expected_burn + priority_fee;

        let initial_capitalization = bank.capitalization();
        let initial_collector_id_balance = bank.get_balance(bank.collector_id());
        bank.distribute_transaction_fee_details();
        let new_collector_id_balance = bank.get_balance(bank.collector_id());

        assert_eq!(
            initial_collector_id_balance + expected_rewards,
            new_collector_id_balance
        );
        assert_eq!(
            initial_capitalization - expected_burn,
            bank.capitalization()
        );
        let locked_rewards = bank.rewards.read().unwrap();
        assert_eq!(
            locked_rewards.len(),
            1,
            "There should be one reward distributed"
        );

        let reward_info = &locked_rewards[0];
        assert_eq!(
            reward_info.1.lamports, expected_rewards as i64,
            "The reward amount should match the expected deposit"
        );
        assert_eq!(
            reward_info.1.reward_type,
            RewardType::Fee,
            "The reward type should be Fee"
        );
    }

    #[test]
    fn test_distribute_transaction_fee_details_zero() {
        let genesis = create_genesis_config(0);
        let bank = Bank::new_for_tests(&genesis.genesis_config);
        assert_eq!(
            *bank.collector_fee_details.read().unwrap(),
            CollectorFeeDetails::default()
        );

        let initial_capitalization = bank.capitalization();
        let initial_collector_id_balance = bank.get_balance(bank.collector_id());
        bank.distribute_transaction_fee_details();
        let new_collector_id_balance = bank.get_balance(bank.collector_id());

        assert_eq!(initial_collector_id_balance, new_collector_id_balance);
        assert_eq!(initial_capitalization, bank.capitalization());
        let locked_rewards = bank.rewards.read().unwrap();
        assert!(
            locked_rewards.is_empty(),
            "There should be no rewards distributed"
        );
    }

    #[test]
    fn test_distribute_transaction_fee_details_overflow_failure() {
        let genesis = create_genesis_config(0);
        let mut bank = Bank::new_for_tests(&genesis.genesis_config);
        let transaction_fee = 100;
        let priority_fee = 200;
        bank.collector_fee_details = RwLock::new(CollectorFeeDetails {
            transaction_fee,
            priority_fee,
        });

        // ensure that account balance will overflow and fee distribution will fail
        let account = AccountSharedData::new(u64::MAX, 0, &system_program::id());
        bank.store_account(bank.collector_id(), &account);

        let initial_capitalization = bank.capitalization();
        let initial_collector_id_balance = bank.get_balance(bank.collector_id());
        bank.distribute_transaction_fee_details();
        let new_collector_id_balance = bank.get_balance(bank.collector_id());

        assert_eq!(initial_collector_id_balance, new_collector_id_balance);
        assert_eq!(
            initial_capitalization - transaction_fee - priority_fee,
            bank.capitalization()
        );
        let locked_rewards = bank.rewards.read().unwrap();
        assert!(
            locked_rewards.is_empty(),
            "There should be no rewards distributed"
        );
    }

    #[test]
    fn test_fee_burn_feature_gate() {
        use agave_feature_set::{
            fee_burn_10_percent, fee_burn_25_percent,
            fee_burn_50_percent, fee_burn_75_percent, fee_burn_100_percent,
        };

        let genesis = create_genesis_config(0);
        let fee_details = CollectorFeeDetails {
            transaction_fee: 1000,
            priority_fee: 500,
        };

        // Test default (0% burn for 1024Chain) - need to disable all fee burn features
        let mut bank = Bank::new_for_tests(&genesis.genesis_config);
        let mut feature_set = (*bank.feature_set).clone();
        feature_set.deactivate(&fee_burn_10_percent::id());
        feature_set.deactivate(&fee_burn_25_percent::id());
        feature_set.deactivate(&fee_burn_50_percent::id());
        feature_set.deactivate(&fee_burn_75_percent::id());
        feature_set.deactivate(&fee_burn_100_percent::id());
        bank.feature_set = std::sync::Arc::new(feature_set);

        let distribution = bank.calculate_reward_and_burn_fee_details(&fee_details);
        // Default: 0% burn, so all transaction fee goes to leader
        assert_eq!(distribution.burn, 0);
        assert_eq!(distribution.deposit, 1500); // 1000 + 500

        // Test with only 50% burn feature activated
        let mut bank_50 = Bank::new_for_tests(&genesis.genesis_config);
        let mut feature_set_50 = (*bank_50.feature_set).clone();
        feature_set_50.deactivate(&fee_burn_10_percent::id());
        feature_set_50.deactivate(&fee_burn_25_percent::id());
        // Keep 50% active
        feature_set_50.deactivate(&fee_burn_75_percent::id());
        feature_set_50.deactivate(&fee_burn_100_percent::id());
        bank_50.feature_set = std::sync::Arc::new(feature_set_50);

        let distribution_50 = bank_50.calculate_reward_and_burn_fee_details(&fee_details);
        assert_eq!(distribution_50.burn, 500); // 50% of 1000
        assert_eq!(distribution_50.deposit, 1000); // 500 remaining + 500 priority

        // Test priority: higher burn percentage takes precedence (75% vs 25%)
        let mut bank_priority = Bank::new_for_tests(&genesis.genesis_config);
        let mut feature_set_priority = (*bank_priority.feature_set).clone();
        feature_set_priority.deactivate(&fee_burn_10_percent::id());
        // Keep 25% active
        feature_set_priority.deactivate(&fee_burn_50_percent::id());
        // Keep 75% active - this should win
        feature_set_priority.deactivate(&fee_burn_100_percent::id());
        bank_priority.feature_set = std::sync::Arc::new(feature_set_priority);

        let distribution_priority = bank_priority.calculate_reward_and_burn_fee_details(&fee_details);
        assert_eq!(distribution_priority.burn, 750); // 75% of 1000
        assert_eq!(distribution_priority.deposit, 750); // 250 remaining + 500 priority
    }
}
