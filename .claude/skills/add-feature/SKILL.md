---
description: Guided 11-step workflow for adding a new WadeScript language feature
user-invocable: true
---

Guide the implementation of a new WadeScript language feature. The feature to add: $ARGUMENTS

Before starting, read the relevant existing docs in `docs/` to understand the current implementation.

## Compilation pipeline context

!`head -5 src/ast.rs src/lexer.rs src/parser.rs src/typechecker.rs src/codegen.rs 2>/dev/null | head -30`

## Steps

Follow the standard feature addition workflow. For each step, check if changes are needed and implement them:

1. **AST** (`src/ast.rs`): Add any new Statement or Expression variants needed
2. **Lexer** (`src/lexer.rs`): Add new tokens if the feature introduces new syntax/keywords
3. **Parser** (`src/parser.rs`): Parse the new syntax into AST nodes
4. **Type Checker** (`src/typechecker.rs`): Add type checking rules for the new feature
5. **Code Generator** (`src/codegen.rs`): Generate LLVM IR for the new constructs
6. **Runtime** (`src/runtime/*.rs`): Add runtime functions if needed (with `#[no_mangle] pub extern "C"`)
7. **Runtime Symbols** (`src/runtime_symbols.rs`): Register any new runtime functions for JIT
8. **Language Defs** (`src/language_defs.rs`): Update keywords/builtins/methods for LSP
9. **VS Code syntax** (`editors/vscode/syntaxes/wadescript.tmLanguage.json`): Update if new keywords added
10. **Tests**: Create `tests/test_<feature>.ws` with comprehensive assertions AND add Rust unit tests
11. **Documentation**: Create or update docs in `docs/` directory

After implementing, run `make test test-rust` to verify no regressions.

Look for optimization opportunities in the implementation. Do not settle for an unoptimized solution.
