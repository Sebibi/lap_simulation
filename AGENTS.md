# Repository Guidelines

## Project Structure & Modules
- `src/`: main Rust source; entry points in `src/bin/`, core logic in modules like `models/`, `simulation/`, `tracks/`, and `plotting/`.
- `tests/`: integration tests; shared helpers in `tests/common/`.
- `results/`: generated outputs (treat as artifacts, not source).
- `.github/workflows/`: CI configuration.
- `ci/`: container build assets for CI.

## Build, Test, and Run
- `cargo build --verbose`: build with detailed output.
- `cargo test --verbose`: run unit + integration tests.
- `cargo run --bin lap_simulation`: run the main simulation binary.
- `cargo test --features ffmpeg`: run tests that require `ffmpeg`.

## Coding Style & Naming
- Indentation: 4 spaces (Rust standard).
- Run `cargo fmt` when formatting changes.
- Modules/files: `snake_case` (e.g., `point_mass.rs`).
- Types: `PascalCase`; functions/variables: `snake_case`.

## Testing Guidelines
- Use Rust’s built-in test harness (`#[test]`) and integration tests under `tests/`.
- Name integration tests descriptively (e.g., `simulation/open_loop.rs`).
- Gate tests that require external tools (like `ffmpeg`) behind feature flags and document how to run them.

## Commit & Pull Request Guidelines
- Commit messages: short, imperative (e.g., “Add duration parameter”, “Refactor rust conventions”); optional prefixes like `ci:` are acceptable.
- PRs: include a verbose summary of what changed, why, and which files were touched; list test commands run.
- If outputs/artifacts change (plots, media), mention the expected result or attach screenshots.
- After opening a PR, check the **Tests and Integration** workflow. If it fails, inspect logs and attempt a fix. When it succeeds, note it in conversation with a green checkmark (✅).

## Issue Workflow
- When working on a GitHub issue, do not commit directly to `master`.
- Create a feature branch (e.g., `feature/issue-123-short-name`) and open a PR for review.
- In the PR description, link the issue and include a closing keyword (e.g., “Fixes #123”) so the issue auto-resolves on merge.

## CI & Container Notes
- CI runs Rust tests in a container image that includes `ffmpeg`.
- The image build workflow runs only when `ci/Dockerfile` changes or via manual dispatch.
