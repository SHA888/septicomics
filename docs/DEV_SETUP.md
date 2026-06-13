# Development Setup

## Prerequisites

- Rust 1.96+ (see `Cargo.toml` workspace `rust-version`)
- Python 3.7+ (for pre-commit hooks)
- `cargo-semver-checks` (for SemVer validation on contract crates)

## Installation

### 1. Clone and build

```bash
git clone https://github.com/septicomics/septicomics.git
cd septicomics
cargo build
```

### 2. Install pre-commit hooks

Pre-commit hooks enforce the project's engineering disciplines before each commit:

```bash
./scripts/setup-hooks.sh
```

Or manually:

```bash
pip install pre-commit
pre-commit install
pre-commit install --hook-type commit-msg
```

### 3. Install semver-checks

The `fed-protocol` and `cdm` crates are SemVer-critical contracts; the CI gate requires semver compliance:

```bash
cargo install cargo-semver-checks
```

## Workflow

### Running tests locally

```bash
# All tests
cargo test

# Specific crate
cargo test -p septicomics-cdm
cargo test -p septicomics-fedproto

# With output
cargo test -- --nocapture
```

### Pre-commit checks

All checks run automatically before `git commit`. To run manually:

```bash
# Run all hooks on all files
pre-commit run --all-files

# Run specific hook
pre-commit run cargo-fmt --all-files
pre-commit run cargo-clippy --all-files

# Update hook versions
pre-commit autoupdate
```

### Formatting and linting

```bash
# Format
cargo fmt

# Lint (warnings as errors)
cargo clippy --all-targets --all-features -- -D warnings

# SemVer checks on contract crates
cargo-semver-checks -p septicomics-cdm
cargo-semver-checks -p septicomics-fedproto
```

## Pre-commit hooks

The framework enforces **five load-bearing disciplines**:

### 1. Code quality (Rust)

| Hook | Command | Purpose |
|------|---------|---------|
| `cargo-fmt` | `cargo fmt` | Format all code to project standard |
| `cargo-clippy` | `cargo clippy -D warnings` | Catch common mistakes and style issues |
| `cargo-test` | `cargo test --all` | All tests pass before commit |

### 2. Contract stability

| Hook | Command | Purpose |
|------|---------|---------|
| `cargo-semver-checks-cdm` | `cargo-semver-checks -p septicomics-cdm` | CDM is SemVer-critical; breaks detected at commit time |
| `cargo-semver-checks-fedproto` | `cargo-semver-checks -p septicomics-fedproto` | Federation protocol is SemVer-critical |

### 3. Load-bearing invariants

| Hook | Check | Enforces |
|------|-------|----------|
| `no-co-authored-by` | Blocks `Co-Authored-By:` trailers | Per CLAUDE.md: attribution stays with human author |
| `enforce-english-plans` | Blocks Japanese in `Plans.md` | Plans must use English status markers (`cc:done` not `cc:完了`) |

### 4. File quality

| Hook | Check | Purpose |
|------|-------|---------|
| `trailing-whitespace` | Trim end-of-line spaces | Clean diffs |
| `end-of-file-fixer` | Enforce final newline | Consistent formatting |
| `check-yaml` | Validate YAML syntax | Catch config errors |
| `check-toml` | Validate TOML syntax | Catch manifest errors |
| `check-merge-conflict` | Detect unresolved conflicts | Prevent accidental commits |
| `detect-private-key` | Block committed secrets | Security gate |

## Troubleshooting

### Pre-commit hook fails on commit

If a hook fails, fix the issue and re-stage. Example:

```bash
# cargo-fmt fails
cargo fmt
git add -A
git commit -m "..."
```

### Skipping hooks (rarely needed)

```bash
# Skip all hooks (not recommended)
git commit --no-verify

# Skip specific hook stages
git commit --no-verify --hook-type commit  # Skip commit hooks
git commit --no-verify --hook-type commit-msg  # Skip commit message hooks
```

### Slow pre-commit runs

The first run of `cargo test` and semver-checks can be slow. Subsequent runs are cached:

```bash
# Rebuild incrementally (faster on repeat)
cargo test --all

# Skip cargo-test locally (not in CI)
pre-commit run --hook-stage=commit -k 'not cargo-test'
```

## CI/CD Pipeline

The same checks run in CI (GitHub Actions) with the same strictness:

1. **Format gate**: `cargo fmt --check` (exact formatting)
2. **Lint gate**: `cargo clippy -D warnings` (no warnings allowed)
3. **Test gate**: `cargo test --all` (all tests pass)
4. **SemVer gate** (contract crates): `cargo-semver-checks` (no breaking changes)

Local pre-commit hooks mirror this, so CI failures are rare.

## Adding new checks

To add a new pre-commit hook:

1. Edit `.pre-commit-hooks.yaml` to define the hook
2. Edit `.pre-commit-config.yaml` to enable it
3. Run `pre-commit run --all-files` to validate
4. Commit both files

Example: Add a license header check

```yaml
- id: check-license-headers
  name: Check license headers
  entry: bash scripts/check-license-headers.sh
  language: system
  files: '\.(rs|py|ts)$'
  stages: [commit]
```

## References

- [pre-commit.com](https://pre-commit.com/) — Framework documentation
- [CLAUDE.md](../CLAUDE.md) — Project conventions and constraints
- [TODO.md](../TODO.md) — CI gates per phase
