# Verification Pipeline Reference

## Steps Detail

### 1. Format Check (`make fmt-check`)
- Runs `cargo fmt --check`
- Auto-fix: `make fmt` then re-run check
- Common issue: New files not formatted before commit

### 2. Lint (`make lint`)
- Runs `cargo clippy -- -D warnings`
- Treats all warnings as errors
- Common issues:
  - Unused imports after refactoring
  - Unnecessary clones (clippy::redundant_clone)
  - Missing error handling

### 3. Build (`make`)
- Builds both compiler (`cargo build`) and runtime (`libwadescript_runtime.a`)
- If build fails after runtime changes, try `make clean-all && make`
- LLVM 17 is auto-detected by the Makefile

### 4. Rust Tests (`make test-rust`)
- Runs `cargo test`
- Unit tests for lexer, parser, typechecker
- If a specific test fails, run `cargo test <test_name>` for focused output

### 5. WadeScript Tests (`make test`)
- Compiles and runs all `tests/test_*.ws` files
- Exit code 0 = pass, non-zero = fail
- Error tests compare output against `.expected` files
- If a single test fails, run `./ws run tests/<test>.ws` for detailed output

## Troubleshooting

| Symptom | Fix |
|---------|-----|
| LLVM not found | `make info` to check config, Makefile auto-detects |
| Linker errors | `make clean-all && make` to rebuild runtime |
| Tests pass locally but fail in CI | Check for path-dependent tests |
| Clippy false positive | Add `#[allow(clippy::rule)]` with justification comment |
