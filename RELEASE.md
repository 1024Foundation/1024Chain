# 1024Chain Release Process

## Version Scheme

1024Chain follows [Semantic Versioning](https://semver.org/):

```
MAJOR.MINOR.PATCH

Examples:
- 4.0.0 → Major release (breaking changes)
- 4.1.0 → Minor release (new features, backward compatible)
- 4.1.1 → Patch release (bug fixes)
```

---

## Branches and Tags

```
========================= master branch (development) ======================>
         \                      \                     \
          \___v4.0.0 tag         \                     \
           \                      \         v4.2.0 tag__\
            \          v4.1.0 tag__\                     \
 v4.0.1 tag__\                      \                 v4.2 branch (beta)
              \___v4.0.2 tag         \___v4.1.1 tag
               \                      \
                \                      \
           v4.0 branch          v4.1 branch (stable)
```

### Branches

| Branch | Purpose | Stability |
|--------|---------|-----------|
| `master` | Active development | Edge |
| `v4.2` | Latest stabilization | Beta |
| `v4.1` | Current stable | Stable |
| `v4.0` | Previous stable | Maintenance |

---

## Release Channels

| Channel | Branch | Stability | Update Frequency |
|---------|--------|-----------|------------------|
| **edge** | `master` | Least stable | Daily |
| **beta** | Latest `vX.Y` | More stable | Weekly |
| **stable** | Second latest `vX.Y` | Most stable | As needed |

---

## Creating a Release

### 1. Prepare the Release Branch

For a new minor version (e.g., v4.2.0):

```bash
# Checkout latest master
git fetch --all
git checkout origin/master

# Create release branch
git checkout -b v4.2
git push -u origin v4.2

# Bump version on master
git checkout master
./scripts/increment-cargo-version.sh minor
git commit -am "Bump version to 4.3.0"
git push
```

### 2. Create the Release Tag

```bash
# On the release branch
git checkout v4.2
git tag v4.2.0
git push origin v4.2.0
```

### 3. Build Release Artifacts

Release artifacts are built automatically by CI when a tag is pushed.

Artifacts include:
- `1024chain-validator-linux-x86_64.tar.gz`
- `1024chain-validator-macos-x86_64.tar.gz`
- `1024chain-validator-macos-aarch64.tar.gz`

### 4. Publish Release Notes

Go to GitHub Releases and create a new release:

1. Select the tag (e.g., `v4.2.0`)
2. Write release notes (see template below)
3. Attach build artifacts
4. Publish

---

## Release Notes Template

```markdown
# 1024Chain v4.2.0

## Highlights

- Brief summary of major changes

## What's New

### Features
- Feature 1 description
- Feature 2 description

### Improvements
- Improvement 1
- Improvement 2

### Bug Fixes
- Fix 1
- Fix 2

## Breaking Changes

- List any breaking changes

## Upgrade Notes

Instructions for upgrading from previous version.

## Compatibility

- Solana SDK: Compatible
- SVM Programs: Compatible
- RPC API: Compatible

## Contributors

Thanks to all contributors!
```

---

## Hotfix Process

For critical bug fixes:

```bash
# Create hotfix branch from stable
git checkout v4.1
git checkout -b hotfix/critical-bug

# Make fix
git commit -am "fix: Critical bug description"

# Merge to stable
git checkout v4.1
git merge hotfix/critical-bug

# Tag patch release
git tag v4.1.2
git push origin v4.1.2

# Cherry-pick to master
git checkout master
git cherry-pick <commit-hash>
git push
```

---

## Validator Upgrade Process

### For Node Operators

1. **Review release notes** for breaking changes
2. **Backup data** before upgrading
3. **Download new version**
   ```bash
   # Using Docker
   docker pull 1024chain/validator:v4.2.0
   
   # Or build from source
   git checkout v4.2.0
   ./cargo build --release
   ```
4. **Stop validator gracefully**
   ```bash
   kill -SIGINT <validator-pid>
   ```
5. **Start new version**
6. **Verify operation**
   ```bash
   curl http://localhost:8899 -d '{"jsonrpc":"2.0","id":1,"method":"getHealth"}'
   ```

### Rollback

If issues occur:

```bash
# Stop current version
kill -SIGINT <validator-pid>

# Switch to previous version
docker pull 1024chain/validator:v4.1.1
# or
git checkout v4.1.1
./cargo build --release

# Restart
```

---

## Network Upgrades

For changes that require coordinated network upgrades:

1. **Announce** upgrade timeline (minimum 7 days notice)
2. **Publish** upgrade instructions
3. **Monitor** validator upgrade progress
4. **Activate** feature gate at designated epoch

---

## Support Policy

| Version | Support Status |
|---------|---------------|
| Current stable (v4.1) | Full support |
| Previous stable (v4.0) | Security fixes only |
| Older versions | No support |

---

## Contact

- **Discord:** #validators channel
- **GitHub:** Open an issue
- **Email:** validators@1024chain.com
