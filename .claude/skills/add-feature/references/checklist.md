# Feature Addition Checklist

Complete each step in order. Skip steps that don't apply to the feature.

## 1. AST (`src/ast.rs`)
- Add new `Statement` or `Expression` enum variants
- Add any new types to the `Type` enum if needed
- Include `Span` for error reporting on new nodes

## 2. Lexer (`src/lexer.rs`)
- Add new `Token` variants for new keywords/operators
- Add keyword string mappings in the keyword lookup
- Handle new character sequences in the tokenizer

## 3. Parser (`src/parser.rs`)
- Parse new syntax into the AST nodes from step 1
- Follow existing patterns for precedence and associativity
- Add error recovery for malformed syntax

## 4. Type Checker (`src/typechecker.rs`)
- Add type checking rules for new expressions/statements
- Register new built-in functions if applicable
- Handle type inference for new constructs
- Add to symbol tables where needed

## 5. Code Generator (`src/codegen.rs`)
- Generate LLVM IR for new AST nodes
- Use inkwell builder methods consistently
- Handle RC retain/release for any new heap-allocated values
- Add push_call_stack/pop_call_stack for new function-like constructs

## 6. Runtime (`src/runtime/*.rs`)
- Implement with `#[no_mangle] pub extern "C"` for C ABI
- Follow existing patterns in the relevant runtime module
- See [runtime-guide.md](runtime-guide.md) for detailed steps

## 7. Runtime Symbols (`src/runtime_symbols.rs`)
- Import the function at top of `get_runtime_symbols()`
- Add `RuntimeSymbol { name: "func_name", addr: func_name as usize }`
- Required for JIT/REPL to find the function

## 8. Language Defs (`src/language_defs.rs`)
- `get_keywords()` — new keywords (must match lexer.rs)
- `get_type_keywords()` — new type keywords (must match lexer.rs)
- `get_builtin_functions()` — new built-ins (must match typechecker.rs)
- `get_list_methods()` / `get_string_methods()` — new methods
- `get_stdlib_modules()` — new std library modules

## 9. VS Code Syntax (`editors/vscode/syntaxes/wadescript.tmLanguage.json`)
- Add new keywords to the keywords pattern
- Add new types to the type pattern
- Only needed if new syntax/keywords were added

## 10. Tests
- Create `tests/test_<feature>.ws` with:
  - `def main() -> int { ... return 0 }`
  - `assert` statements for normal cases, edge cases, boundaries
- For error tests: create `.expected` file with expected error output
- Add Rust unit tests in the relevant source file

## 11. Documentation
- Create or update doc in `docs/` directory
- Update `docs/LANGUAGE_REFERENCE.md` with syntax examples
- Update documentation index in `CLAUDE.md` if new doc file created
