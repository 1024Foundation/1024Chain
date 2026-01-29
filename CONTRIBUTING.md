# Contributing to 1024Chain

Thank you for your interest in contributing to 1024Chain!

1024Chain is a Solana-derived blockchain optimized for high-frequency financial workloads.
We welcome contributions that align with our design principles:
**execution over generality, low latency first, high parallelism, and systems-first engineering**.

---

## Getting Started

### 1. Fork and Clone

```bash
git clone https://github.com/YOUR_USERNAME/1024Chain.git
cd 1024Chain
```

### 2. Set Up Development Environment

```bash
# Install Rust
curl https://sh.rustup.rs -sSf | sh
source $HOME/.cargo/env
rustup component add rustfmt clippy

# Install dependencies (Ubuntu)
sudo apt-get install libssl-dev libudev-dev pkg-config zlib1g-dev \
    llvm clang cmake make libprotobuf-dev protobuf-compiler libclang-dev

# Build
./cargo build
```

### 3. Run Tests

```bash
./cargo test
```

---

## Pull Request Guidelines

### Before Opening a PR

1. **Search existing issues and PRs** to avoid duplicates
2. **Open an issue first** for significant changes to discuss the approach
3. **Keep PRs small and focused** - one logical change per PR
4. **Write tests** for all new functionality
5. **Run the full test suite** locally before submitting

### PR Requirements

- [ ] Code compiles without warnings
- [ ] All tests pass (`./cargo test`)
- [ ] Code is formatted (`./cargo fmt`)
- [ ] Clippy passes (`./cargo clippy`)
- [ ] Commit messages are clear and descriptive
- [ ] PR description explains the "why" not just the "what"

### PR Size Guidelines

| Type | Max Lines | Notes |
|------|-----------|-------|
| Bug fix | ~500 | Include regression test |
| New feature | ~1,000 | Include tests and docs |
| Refactoring | No limit | If no functional changes |

### Commit Message Format

```
<type>: <short description>

<optional longer description>

<optional issue reference>
```

**Types:**
- `feat:` New feature
- `fix:` Bug fix
- `refactor:` Code refactoring
- `docs:` Documentation changes
- `test:` Test changes
- `chore:` Build/tooling changes

**Example:**
```
feat: Add zero-inflation genesis configuration

- Set inflation parameters to 0%
- Update token economics documentation
- Add validation for fixed supply

Closes #123
```

---

## Code Style

### Rust Conventions

- **Format:** All code must pass `rustfmt`
- **Lint:** All code must pass `clippy` without warnings
- **Naming:** Use descriptive names, avoid abbreviations
  - Variables: `snake_case`
  - Functions: `verb_subject` (e.g., `validate_transaction`)
  - Types: `PascalCase`
- **Comments:** Explain "why", not "what"
- **Error handling:** Prefer `?` operator, use `expect()` over `unwrap()`

### Performance Considerations

1024Chain prioritizes performance. When contributing:

- **Benchmark your changes** - Include before/after metrics
- **Avoid allocations in hot paths** - Use stack allocation when possible
- **Leverage parallelism** - Use Rayon for parallel iteration
- **Profile first** - Don't guess, measure

---

## Testing

### Test Categories

| Type | Command | Purpose |
|------|---------|---------|
| Unit tests | `./cargo test` | Test individual functions |
| Integration tests | `./cargo test --test '*'` | Test component interactions |
| Benchmarks | `cargo +nightly bench` | Performance measurement |

### Writing Good Tests

```rust
#[test]
fn test_descriptive_name_of_what_is_being_tested() {
    // Arrange
    let input = ...;
    
    // Act
    let result = function_under_test(input);
    
    // Assert
    assert_eq!(result, expected);
}
```

---

## Review Process

1. **Automated checks** - CI must pass (tests, format, clippy)
2. **Code review** - At least one maintainer approval required
3. **Performance review** - For performance-critical changes
4. **Merge** - Squash merge to maintain clean history

### Response Time

- Initial review: Within 7 days
- Follow-up reviews: Within 3 days
- Stale PRs (no activity for 14 days) may be closed

---

## Areas for Contribution

### High Priority

- Performance optimizations
- Test coverage improvements
- Documentation enhancements
- Bug fixes

### Good First Issues

Look for issues labeled `good first issue` for beginner-friendly tasks.

### Not Accepting

- Changes that break SVM compatibility
- Features that don't align with financial execution focus
- Cosmetic-only changes without functional benefit

---

## SVM Compatibility Guidelines

1024Chain maintains **100% SVM compatibility**. We only optimize "how to be faster", not "what it is".

### What You CAN Modify

| Layer | Examples |
|-------|----------|
| Network | Gossip, turbine, QUIC tuning |
| Scheduler | Transaction ordering, priority |
| Execution | Parallelism, thread pools |
| Storage | Ledger, accounts DB, caching |
| Consensus Parameters | Tick rate, slot timing |

### What You MUST NOT Modify

| Layer | Examples |
|-------|----------|
| Transaction Format | Message structure, versioned transactions |
| Address Lookup Table | ALT behavior and semantics |
| Syscalls | All BPF syscalls behavior |
| Loader | BPF loader, upgradeable loader |
| ABI / Compute | Compute units, stack size, heap |
| Account Model | Ownership, rent, lamports semantics |
| Runtime Semantics | Instruction processing, CPI |

### Compatibility Checklist

Before submitting performance-related PRs:

- [ ] Does not change transaction/message format
- [ ] Does not alter syscall behavior
- [ ] Does not modify account ownership rules
- [ ] Does not change compute unit semantics
- [ ] Existing Solana programs work unchanged
- [ ] Solana CLI/SDK tools work unchanged

> **Rule:** If a change would cause any existing Solana program to behave differently, it breaks compatibility and will not be accepted.

---

## Communication

- **GitHub Issues:** Bug reports and feature requests
- **Discord:** Real-time discussion and questions
- **Pull Requests:** Code contributions

---

## License

By contributing to 1024Chain, you agree that your contributions will be licensed
under the Apache License 2.0.

---

## Code of Conduct

We are committed to providing a welcoming and inclusive environment.
Please be respectful and constructive in all interactions.

---

Thank you for contributing to 1024Chain!
