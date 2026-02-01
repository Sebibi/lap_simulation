# Repository Guidelines

## Project Structure & Module Organization
- `src/`: main Rust source. Entry points live in `src/bin/`, core logic in submodules like `models/`, `simulation/`, `tracks/`, and `plotting/`.
- `tests/`: integration tests, with shared helpers under `tests/common/`.
- `results/`: generated outputs (treat as artifacts, not source).
- `.github/workflows/`: CI configuration.
- `ci/`: container build assets for CI.

## Build, Test, and Development Commands
- `cargo build --verbose`: build the project with detailed output.
- `cargo test --verbose`: run unit and integration tests.
- `cargo run --bin lap_simulation`: run the main simulation binary.
- `cargo test --features ffmpeg`: run tests that require `ffmpeg`.

## Coding Style & Naming Conventions
- Indentation: 4 spaces (Rust standard).
- Prefer idiomatic Rust formatting via `cargo fmt` (run before PR if formatting changes).
- Module/file names follow `snake_case` (e.g., `point_mass.rs`).
- Types in `PascalCase`, functions/variables in `snake_case`.

## Testing Guidelines
- Framework: Rust’s built-in test harness (`#[test]`) plus integration tests in `tests/`.
- Integration tests live under `tests/` and should have descriptive file names (e.g., `simulation/open_loop.rs`).
- If adding tests that require external tools (like `ffmpeg`), gate them behind feature flags and document the usage.

## Commit & Pull Request Guidelines
- Commit messages are short and imperative (e.g., “Add duration parameter”, “Refactor rust conventions”). Optional prefixes like `ci:` appear in history.
- PRs should include a clear summary of changes and testing performed (commands run).
- If a change affects outputs or artifacts (e.g., generated plots), mention the expected result or attach screenshots where relevant.
- After opening a PR, check the status/logs of the **Rust** CI workflow. If it fails, analyze logs and attempt a fix. When the Rust CI succeeds, note it in conversation with a green checkmark (✅).

## Issue Workflow
- When working on a GitHub issue, do not commit directly to `master`.
- Create a feature branch (e.g., `feature/issue-123-short-name`) and open a PR for review.

## CI & Container Notes
- CI runs Rust tests in a container image that includes `ffmpeg`.
- The image build workflow only runs when `ci/Dockerfile` changes or via manual dispatch.
