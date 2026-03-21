Review recent code changes for correctness, consistency, and potential issues.

Steps:

1. Run `git diff HEAD~1` (or `git diff --staged` if uncommitted) to see what changed
2. For each changed file, review for:
   - **Correctness**: Logic errors, off-by-one errors, missing edge cases
   - **Type safety**: Proper type handling in typechecker.rs and codegen.rs
   - **Memory safety**: Correct RC operations, no leaks or use-after-free in codegen/runtime
   - **Consistency**: Changes follow existing patterns and conventions in the codebase
   - **Missing pieces**: If a new feature was added, verify ALL steps from the feature workflow were completed:
     - AST, lexer, parser, typechecker, codegen all updated?
     - Runtime symbols registered if runtime functions added?
     - Language defs updated for LSP?
     - Tests added (both .ws and Rust)?
     - Docs updated?
3. Check for common WadeScript-specific issues:
   - LLVM IR generation correctness (pointer types, calling conventions)
   - Stack trace push/pop balanced in codegen
   - RC retain/release balanced for new expressions
4. Run `make test test-rust` to verify tests pass
5. Report findings with specific file:line references
