---
description: Diagnose and fix a failing WadeScript test
user-invocable: true
---

Diagnose and fix a failing test. The test: $ARGUMENTS

## Test infrastructure

!`ls tests/test_*.ws 2>/dev/null | wc -l` test files in tests/

## Steps

1. Run the failing test to see the actual error:
   - For `.ws` tests: `./ws run tests/<test_file>.ws`
   - For Rust tests: `make test-rust` (or `cargo test <test_name>`)
2. Read the test file to understand what it expects
3. Analyze the error:
   - **Compile error**: Check lexer → parser → typechecker pipeline for the failing construct
   - **Runtime error / segfault**: Use `./ws run <file> --emit-llvm` to inspect generated IR. Check codegen.rs for the relevant expression/statement generation
   - **Wrong output**: Trace through codegen to find where the wrong value is produced
   - **Assertion failure**: Identify which assert failed and trace the logic
4. Read the relevant source files to find the bug
5. Fix the underlying bug in the compiler/runtime. Per project rules: **fix the bug, do not rewrite the test**
6. Run the fixed test to confirm it passes
7. Run `make test test-rust` to verify no regressions
