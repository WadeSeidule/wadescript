# Code Review Checklist

## General Correctness
- [ ] No logic errors, off-by-one errors, or missing edge cases
- [ ] Error handling is appropriate (not over-engineered, not missing)
- [ ] No unused imports, variables, or dead code introduced

## WadeScript-Specific Checks

### Type System (`typechecker.rs`)
- [ ] Type compatibility rules respected (Float accepts Int, collections exact match)
- [ ] New built-in functions registered with correct signatures
- [ ] Symbol table entries added for new variables/functions

### Code Generation (`codegen.rs`)
- [ ] LLVM IR types match Rust function signatures exactly
- [ ] Pointer types correct (i8*, opaque pointers for LLVM 17)
- [ ] Calling conventions match (`extern "C"` on both sides)
- [ ] Stack trace balanced: every `push_call_stack` has matching `pop_call_stack`

### Memory Safety (RC)
- [ ] RC retain/release balanced for new heap-allocated values
- [ ] No retain without corresponding release path
- [ ] Temporaries released after use (especially in expressions)
- [ ] String returns properly managed (CString ownership)

### Pipeline Completeness (for new features)
- [ ] AST nodes added (`src/ast.rs`)
- [ ] Tokens added if new syntax (`src/lexer.rs`)
- [ ] Parser handles new syntax (`src/parser.rs`)
- [ ] Type checker validates new constructs (`src/typechecker.rs`)
- [ ] Code generator emits correct IR (`src/codegen.rs`)
- [ ] Runtime functions added if needed (`src/runtime/*.rs`)
- [ ] Runtime symbols registered for JIT (`src/runtime_symbols.rs`)
- [ ] Language defs updated for LSP (`src/language_defs.rs`)
- [ ] VS Code syntax updated if new keywords (`editors/vscode/syntaxes/`)
- [ ] Tests added (both `.ws` and Rust unit tests)
- [ ] Documentation updated (`docs/`)

## Consistency
- [ ] Follows existing patterns in the file being modified
- [ ] Naming conventions match surrounding code
- [ ] Error messages follow existing format (colored, with span info)

## Common Pitfalls
| Issue | Where to check |
|-------|---------------|
| Forgot runtime symbol registration | `src/runtime_symbols.rs` — JIT/REPL will crash |
| Forgot language def update | `src/language_defs.rs` — LSP won't autocomplete |
| Wrong LLVM type width | `codegen.rs` — will segfault at runtime |
| Missing RC release on error path | `codegen.rs` — memory leak |
| String not null-terminated | `runtime/string.rs` — undefined behavior |
