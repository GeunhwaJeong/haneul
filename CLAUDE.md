# CLAUDE.md

## Crate-specific CLAUDE.md files
When a sub-crate's CLAUDE.md conflicts with this file, the sub-crate's instructions win.

## Individual Preferences
Individual preferences supersede and extend project preferences:
- @CLAUDE.local.md if present.

## Essential Development Commands

### License comments

Source files that originate from Mysten Labs must start with the following license in comments at the top of the file:

    Copyright (c) Mysten Labs, Inc.
    Modifications Copyright (c) 2026 Geunhwa Jeong
    SPDX-License-Identifier: Apache-2.0

New source files created in this repository must instead start with:

    Copyright (c) 2026 Geunhwa Jeong
    SPDX-License-Identifier: Apache-2.0

The year is the year the file was first modified or created here; it stays fixed afterwards. `cargo xlint` enforces these headers for rs, move, sh, py, proto, js and ts files.

### Building and Installation

```bash
# Build a specific crate. Generally don't need to do release build.
cargo build -p haneul-core

# Check code without code generation or linking (preferred)
cargo check
```

### Testing

```bash
# Run e2e tests. simtests must be run with `cargo simtest` to avoid false negatives
cargo simtest -p haneul-e2e-tests

# Run Rust unittests. skip simulation tests as they may cause false negatives with `cargo nextest`
HANEUL_SKIP_SIMTESTS=1 cargo nextest run -p <crate-name>
```

**Important Notes for Testing:**
- When compiling or running tests in this repository, set timeout limits to at least 10 minutes due to the large codebase size
- For faster iteration, use -p to select only the most relevant packages for testing. Use multiple `-p` flags if necessary, e.g. `cargo nextest run -p haneul-types -p haneul-core`
- Use `cargo nextest run --lib` to run only library tests and skip integration tests for faster feedback
- Use a scoped `cargo insta test` for the relevant package when snapshots are affected. Inspect the generated snapshot diffs. If they match the intended changes, update them with `cargo insta accept`. Do not accept unrelated snapshot changes.
- Consult crate-specific CLAUDE.md files for instructions on which tests to run, when changing files in those crates

### Linting and Formatting

```bash
# Formats & lints all Rust & Move (can be slow).
./scripts/lint.sh

# For formatting:
cargo fmt --all

# Lint a single crate in `crates/`, `consensus/`, `haneul-execution/`:
cargo xclippy -p <crate-name>

# Linting all crates in `external-crates/`: cd into the crate directory and run:
cargo move-clippy
```

## High-Level Architecture

### Core Components Structure

```
haneul/
├── crates/                             # Main Rust crates
│   ├── haneul-core/                       # Core blockchain logic
│   ├── haneul-node/                       # Validator node implementation
│   ├── haneul-framework/                  # Move system packages & stdlib
│   ├── haneul-types/                      # Core type definitions
│   ├── haneul-json-rpc/                   # JSON-RPC API server
│   ├── haneul-indexer-alt-graphql/        # GraphQL API server
│   └── haneul-indexer-alt/                # Blockchain data indexer
├── consensus/                          # Consensus mechanism (Mysticeti)
├── haneul-execution/                      # Move execution layer with versions
├── dapps/                              # Frontend applications
└── external-crates/                    # Move compiler and VM
```

### Key Architectural Patterns

1. **Authority System**: Haneul uses a set of validators (authorities) that process transactions in parallel. Each authority maintains its own state and participates in Mysticeti consensus.

2. **Data Model**: Haneul supports an object data model where each object has a unique ID and version. Accounts can also own balances.

3. **Transaction Flow**:
   - User → Fullnode → Validators
   - All user transactions require consensus voting and commit before execution.
   - Pre and post-consensus fastpath executions have been removed. Surviving mentions of "fastpath" refer to consensus transaction-voting logic, owned object logic, or should be reworded or removed. There is no longer a separate execution path called fastpath.

4. **Storage Layer**:
   - Uses RocksDB or Tidehunter for persistent storage on Haneul nodes.
   - Separate stores for permanent, per-epoch, checkpoint, consensus and indexing data

5. **Execution Pipeline**:
   - Consensus output → Execution → Effects commitment
   - Move VM executes smart contracts with gas metering
   - Parallel execution for non-conflicting transactions

## Development Notes

### Build flags

Haneul binaries like haneul-node built with `release` profile have `panic=abort` enabled.

### Test-Only Code

Use `#[cfg(test)]` for test-only code used within the same crate. Use `#[cfg(feature = "testing")]` for test-only code that must be callable cross-crate. For the `testing` feature: define `testing = []` in the crate's `Cargo.toml`, and callers must propagate it via `features = ["testing"]` in their dependency declaration.

Use `#[tokio::test]` for async tests, not `#[test]`.

### Protocol Config Changes:

When modifying `crates/haneul-protocol-config/src/lib.rs`, always invoke `/protocol-config` to verify changes are safe. Incorrect changes can break network consensus.

### Raising a PR:

When opening or updating a PR in this repo, always invoke the `/send-pr` skill.

### Comment Writing Guidelines

**Do NOT comment the obvious** - comments should not simply repeat what the code does.
**When to comment**:
- Non-obvious algorithms or business logic
- Temporary exclusions, timeouts, or thresholds and their reasoning
- Complex calculations where the "why" isn't immediately clear
- Subtle race conditions or threading considerations
- Assumptions about external state or preconditions

**When NOT to comment**:
- Simple variable assignments
- Standard library usage
- Self-descriptive function calls
- Basic control flow (if/for/while)
