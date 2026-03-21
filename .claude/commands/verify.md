Run the full verification pipeline to check code quality and correctness. This is the equivalent of a CI check and should be run before committing or after making changes.

Run these steps in order, stopping if any step fails:

1. **Format check**: Run `make fmt-check` to verify code formatting. If it fails, run `make fmt` to fix it, then re-check.
2. **Lint**: Run `make lint` to check for Clippy warnings. Fix any issues found.
3. **Build**: Run `make` to build the compiler and runtime.
4. **Rust tests**: Run `make test-rust` to run all Rust unit tests.
5. **WadeScript tests**: Run `make test` to run all WadeScript integration tests.

Report a summary at the end showing which steps passed/failed. If all steps pass, confirm the codebase is ready to commit.
