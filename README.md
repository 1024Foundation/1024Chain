<p align="center">
  <img alt="1024Chain" src="./docs/static/img/logo.png" width="120" />
</p>

<h1 align="center">1024Chain</h1>

<p align="center">
  <strong>A high-performance, Rust-native Layer-1 blockchain forked from the Solana validator codebase,<br/>
  optimized from the ground up for high-frequency financial workloads.</strong>
</p>

<p align="center">
  <a href="./README_CN.md">中文</a> •
  <a href="https://testnet-scan.1024chain.com">Explorer</a> •
  <a href="https://docs.1024chain.com">Documentation</a> •
  <a href="https://discord.gg/1024chain">Discord</a>
</p>

---

## Overview

1024Chain is an independent Layer-1 blockchain derived from the Solana validator codebase.
It is designed to be **lighter, faster, and more execution-focused** than the Solana mainnet,
with architectural and parameter choices tailored specifically for
**high-throughput trading, low-latency matching, and rapid settlement**.

> **Built for Finance from Layer-1:**  
> Unlike general-purpose blockchains that retrofit financial applications,
> 1024Chain is engineered from the protocol layer to support
> **sub-second order book matching** and **sub-second settlement**.

## Motivation

Solana pioneered a high-performance, Rust-based blockchain architecture.
However, as the Solana mainnet evolved into a general-purpose, large-scale ecosystem,
its validator design necessarily optimized for **broad compatibility, long-term stability,
and heterogeneous workloads**.

1024Chain takes a different set of trade-offs.

Instead of serving all use cases, we focus on a narrow but demanding domain:

> **High-frequency financial execution**

This includes on-chain order books, derivatives, arbitrage, market making,
and latency-sensitive financial infrastructure.

---

## Key Advantages

### Performance Engineered for Finance

| Capability | Description |
|------------|-------------|
| **Sub-second Matching** | Order book matching engine achieves sub-second confirmation |
| **Sub-second Settlement** | Trade settlement completes in under 1 second end-to-end |
| **High Throughput** | Optimized for sustained high-TPS under trading workloads |
| **Deterministic Execution** | Predictable transaction ordering for fair trading |

### Protocol-Level Optimizations

- **Faster Block Production**  
  Higher tick rate delivers higher TPS and shorter confirmation times.

- **Optimized Scheduler**  
  Transaction scheduling prioritized for trading patterns and account access locality.

- **Reduced Confirmation Latency**  
  Faster finality for time-sensitive financial operations.

- **Parallel Execution**  
  Aggressive parallelization leveraging account isolation for maximum throughput.

### Developer Experience

- **100% SVM Compatible** — All Solana programs, tools, and SDKs work out of the box
- **Familiar Tooling** — Use Anchor, Solana CLI, web3.js, and existing Solana development workflows
- **Battle-tested Runtime** — Built on the proven Agave/Solana validator codebase

### Operational Benefits

- **Lighter Validator Footprint** — Reduced complexity for easier deployment and maintenance
- **Lower Infrastructure Costs** — Optimized resource usage for cost-effective node operation
- **Simplified Architecture** — Focused feature set means fewer moving parts and failure modes

---

## Ecosystem

1024Chain powers a suite of high-performance financial applications,
all benefiting from protocol-level optimizations for speed and reliability.

### 1024EX — Professional Trading Exchange

A next-generation decentralized exchange built natively on 1024Chain,
offering CEX-like performance with full on-chain transparency.

- **Perpetual Contracts** — High-leverage perpetual futures trading
- **Spot Trading** — Instant spot execution with deep liquidity
- **Options** — On-chain options trading with real-time pricing and settlement
- **Millisecond Matching** — Ultra-fast order book matching engine
- **Sub-second Settlement** — Settlement finality < 1 second
- **Professional APIs** — Built for market makers and algorithmic traders

### Prediction Market

Decentralized prediction markets powered by 1024Chain's low-latency infrastructure.

- **Real-time Odds Updates** — Prices reflect new information within milliseconds
- **Instant Settlement** — Outcomes settled on-chain as soon as events resolve
- **High-frequency Trading Support** — Market makers can provide liquidity efficiently
- **Transparent & Verifiable** — All market operations recorded on-chain

---

## Why Build on 1024Chain?

| If You Need... | 1024Chain Delivers |
|----------------|---------------------|
| Fast order matching | Millisecond matching engine |
| Quick trade settlement | Sub-second on-chain finality |
| High transaction throughput | Optimized for trading workloads |
| Solana compatibility | 100% SVM compatible |
| Professional trading infra | Built for institutional use cases |
| Lower operational costs | Lighter, more efficient validators |

## Key Design Principles

- **Execution over generality** — Prioritize deterministic execution and throughput over maximal flexibility
- **Low latency first** — Optimize block production, scheduling, and confirmation paths for fast feedback loops
- **High parallelism** — Aggressively leverage parallel execution and account isolation to maximize throughput
- **Rust-native, systems-first** — Treat the validator as a real-time distributed system, not an application framework

## What 1024Chain Is (and Is Not)

**1024Chain is:**
- A Solana-derived Rust blockchain optimized from Layer-1 for finance
- A high-performance financial execution layer
- A system designed for professional trading infrastructure
- 100% SVM (Solana Virtual Machine) compatible
- The foundation for 1024EX and other financial applications

**1024Chain is not:**
- A general-purpose smart contract platform for all applications
- An official Solana network or upgrade
- A replacement for the Solana mainnet

---

## Network Information

### Testnet

```yaml
RPC Endpoint:    https://testnet-rpc.1024chain.com/rpc/
WebSocket:       wss://testnet-rpc.1024chain.com/ws/
Block Explorer:  https://testnet-scan.1024chain.com/
```

---

## Quick Start

### Install Rust

```bash
curl https://sh.rustup.rs -sSf | sh
source $HOME/.cargo/env
rustup component add rustfmt
```

### Install Dependencies

**Ubuntu/Debian:**
```bash
sudo apt-get update
sudo apt-get install libssl-dev libudev-dev pkg-config zlib1g-dev \
    llvm clang cmake make libprotobuf-dev protobuf-compiler libclang-dev
```

**Fedora:**
```bash
sudo dnf install openssl-devel systemd-devel pkg-config zlib-devel \
    llvm clang cmake make protobuf-devel protobuf-compiler perl-core libclang-dev
```

### Clone and Build

```bash
git clone https://github.com/1024-chain/1024Chain.git
cd 1024Chain
./cargo build --release
```

### Run a Validator

```bash
git clone https://github.com/1024-chain/1024chain-config.git
cd 1024chain-config
make bootstrap  # Start genesis validator
```

See [1024chain-config](https://github.com/1024-chain/1024chain-config) for more deployment options.

### Testing

```bash
./cargo test                    # Run the test suite
cargo +nightly bench            # Run benchmarks
```

---

## Project Structure

```
1024Chain/
├── core/               # Validator core logic
├── runtime/            # Transaction execution runtime
├── svm/                # Solana Virtual Machine
├── programs/           # Native programs (System, Stake, Vote, etc.)
├── program-binaries/   # Pre-compiled SPL programs
├── rpc/                # JSON-RPC server
├── gossip/             # Cluster communication
├── ledger/             # Block storage
├── validator/          # Validator binary
└── cli/                # Command-line tools
```

---

## Documentation & Resources

- **Developer Docs:** https://docs.1024chain.com
- **API Reference:** https://docs.1024chain.com/api
- **Validator Guide:** https://docs.1024chain.com/validators
- **1024EX Docs:** https://docs.1024ex.com

## Contributing & Open Source

We welcome contributions! Please read [CONTRIBUTING.md](./CONTRIBUTING.md) before submitting PRs.

### SVM 100% Compatibility Commitment

1024Chain pursues higher performance while strictly maintaining full SVM compatibility.

**What we optimize:**
- Network layer, scheduler, execution parallelism
- Storage layer, consensus parameters

**What we never touch:**
- Transaction/message format, Address Lookup Table behavior
- Syscalls / Loader / ABI / Compute semantics
- Account ownership / Rent semantics

---

## Vision

> Solana proved that high-performance blockchains are possible.
> 1024Chain is optimized from the protocol layer for **high-speed financial scenarios**.

We focus on **high-frequency financial execution** — on-chain order books, derivatives,
arbitrage, market making, and all latency-sensitive financial infrastructure.

The future of finance is on-chain, and 1024Chain is the infrastructure built for it.

---

## Links

**1024Chain:**
- **Website:** https://1024chain.com
- **Documentation:** https://docs.1024chain.com
- **Explorer:** https://testnet-scan.1024chain.com
- **Discord:** https://discord.gg/1024chain
- **Twitter:** https://twitter.com/1024chain

**1024EX:**
- **Website:** https://1024ex.com
- **Documentation:** https://docs.1024ex.com
- **Discord:** https://discord.gg/1024ex
- **Twitter:** https://x.com/1024EX

---

## License

1024Chain is released under the **Apache License 2.0**. See [LICENSE](./LICENSE) for details.

Solana and the Solana logo are trademarks of Solana Foundation.
1024Chain is an independent project and is **not affiliated with Solana Foundation or its affiliates**.

---

**Maintained by:** 1024 Foundation
