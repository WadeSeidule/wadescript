# WadeScript - AI Assistant Guide

## Rules

- Use `./ws` to run/build WadeScript (binaries use `.o` extension)
- When adding features, always add tests to both Rust code and `tests/` directory
- When tests fail, fix the underlying bug. Do not rewrite tests to work around bugs.
- When adding or changing functionality, document changes in `docs/` directory
- After any implementation, look for optimization opportunities. Do not go with an unoptimized solution unless explicitly told to.
- Always run `make test test-rust` after changes to verify no regressions

## Commands

```bash
# Build
make                         # Build compiler + runtime (debug)
make clean-all && make       # Full rebuild from scratch

# Run
./ws run file.ws             # Compile and run a program
./ws run file.ws --emit-llvm # Emit LLVM IR for debugging
./ws build file.ws           # Compile to executable (.o)
./ws build file.ws -o name   # Compile with custom output name

# Test
make test                    # Run all WadeScript tests
make test-rust               # Run all Rust unit tests
./ws test                    # Alternative test runner
./ws run tests/test_foo.ws   # Run individual test

# Development
make check                   # Fast syntax check
make fmt                     # Format Rust code
make examples                # Compile all examples
./ws repl                    # Start interactive REPL
./ws lsp                     # Start language server
```

## Project Structure

```
wadescript/
├── src/
│   ├── main.rs              # Entry point, CLI args, linking
│   ├── lexer.rs             # Tokenization
│   ├── parser.rs            # Tokens → AST
│   ├── ast.rs               # AST node definitions
│   ├── typechecker.rs       # Type checking, symbol tables
│   ├── codegen.rs           # AST → LLVM IR (largest file, ~4K lines)
│   ├── jit.rs               # JIT engine for REPL
│   ├── repl.rs              # Interactive REPL
│   ├── runtime_symbols.rs   # Runtime symbol registry (keeps JIT in sync)
│   ├── language_defs.rs     # Language definitions (keeps LSP in sync)
│   ├── lsp/                 # Language Server Protocol
│   │   ├── server.rs        # LSP server (tower-lsp)
│   │   ├── analysis.rs      # Code analysis
│   │   ├── document.rs      # Document state
│   │   ├── diagnostics.rs   # Error → diagnostic conversion
│   │   └── span.rs          # Position utilities
│   └── runtime/             # Rust runtime → libwadescript_runtime.a
│       ├── list.rs          # Dynamic lists
│       ├── dict.rs          # Hash table dictionaries
│       ├── string.rs        # String operations
│       ├── rc.rs            # Reference counting
│       ├── exceptions.rs    # Exception handling
│       ├── io.rs            # File I/O
│       ├── cli.rs           # CLI argument parsing
│       └── http.rs          # HTTP client
├── std/                     # Standard library (.ws modules: cli, http, io)
├── tests/                   # Test suite (44 .ws test files)
├── examples/                # Example programs (64 .ws files)
├── benchmarks/              # Performance benchmarks
├── editors/vscode/          # VS Code extension (syntax + LSP)
└── docs/                    # All detailed documentation
```

### Compilation Pipeline

1. **Lexing** (lexer.rs): Source → Tokens
2. **Parsing** (parser.rs): Tokens → AST
3. **Type Checking** (typechecker.rs): Type validation
4. **Code Generation** (codegen.rs): AST → LLVM IR → Object file
5. **Linking** (main.rs): Object file + runtime → Executable

## Development Workflows

### Adding a New Feature

1. **AST** (ast.rs): Add Statement/Expression variants
2. **Lexer** (lexer.rs): Add tokens if needed
3. **Parser** (parser.rs): Parse new syntax
4. **Type Checker** (typechecker.rs): Add type rules
5. **Code Generator** (codegen.rs): Generate LLVM IR
6. **Runtime** (src/runtime/*.rs): Add runtime functions if needed
7. **Runtime Symbols** (runtime_symbols.rs): Register new runtime functions for JIT
8. **Language Defs** (language_defs.rs): Update keywords/builtins for LSP
9. **Tests**: Create `tests/test_*.ws` and Rust tests
10. **Docs**: Document in `docs/` directory
11. **Verify**: `make test test-rust`

### Adding a New Runtime Function

1. Implement in `src/runtime/*.rs` with `#[no_mangle] pub extern "C"`
2. Declare in codegen.rs (in the appropriate `declare_*_functions` method)
3. Register in runtime_symbols.rs:
   - Import at top of `get_runtime_symbols()`
   - Add `RuntimeSymbol { name: "func_name", addr: func_name as usize }`

### Updating LSP Language Definitions

Update `src/language_defs.rs` when adding keywords, types, or built-ins:
- `get_keywords()` - must match lexer.rs
- `get_type_keywords()` - must match lexer.rs type tokens
- `get_builtin_functions()` - must match typechecker.rs
- `get_list_methods()` / `get_string_methods()`
- `get_stdlib_modules()` - must match std/*.ws files

Also update `editors/vscode/syntaxes/wadescript.tmLanguage.json` if new syntax is added.

### Test Structure

- Regular tests: `tests/test_*.ws` - use `assert` statements, exit 0 on pass
- Error tests: `tests/test_error_*.ws` - with `.expected` files for expected error output
- REPL tests: `tests/test_repl.sh`
- Rust tests: `make test-rust`

## Documentation Index

All feature documentation lives in `docs/`. Refer to these before implementing changes:

| Doc | Topic |
|-----|-------|
| `LANGUAGE_REFERENCE.md` | Syntax quick reference with examples |
| `QUICKSTART.md` | Getting started guide |
| `BUILD.md` | Build system, compilation, troubleshooting |
| `TESTING.md` | Test suite guidelines |
| `TEST_SUITE_SUMMARY.md` | Test coverage overview |
| `DATA_STRUCTURES.md` | Lists, dicts, arrays implementation |
| `DATA_STRUCTURES_STATUS.md` | Implementation status tracker |
| `LISTS.md` | List implementation details |
| `TUPLES.md` | Tuple types, literals, unpacking, indexing |
| `SLICES.md` | Python-style slice syntax |
| `FOR_LOOPS.md` | For loop implementation |
| `NAMED_ARGS.md` | Named arguments and default parameters |
| `STR_PRINT.md` | Polymorphic str() and print() functions |
| `EXCEPTION_SYSTEM.md` | Exception handling system |
| `IMPORTS.md` | Module system |
| `CLI.md` | CLI argument parsing module |
| `HTTP.md` | HTTP client module |
| `REPL.md` | Interactive REPL |
| `LSP.md` | Language server for IDE integration |
| `RC_IMPLEMENTATION.md` | Reference counting internals |
| `RC_LOOP_HOISTING.md` | Phase 4b loop hoisting optimization |
| `BENCHMARK_RESULTS.md` | Performance benchmarks |
| `TODO.md` | Roadmap (sets, os module, expanded CLI) |

## Key Architecture Notes

- **Type system**: `int` (i64), `float` (f64), `str`, `bool`, `void`, `list[T]`, `dict[K,V]`, `array[T,N]`, tuples `(T1, T2, ...)`, classes. Float accepts Int (auto-promotion).
- **Memory management**: Automatic reference counting with 4-phase optimization (basic RC → move semantics → escape analysis → loop hoisting). ~5-8% overhead. See `docs/RC_IMPLEMENTATION.md`.
- **Runtime**: Rust static library (`libwadescript_runtime.a`) linked into compiled executables.
- **Standard library**: WadeScript modules in `std/` (cli.ws, http.ws, io.ws) that wrap runtime functions. Used via `import "cli"` etc.
- **LLVM 17**: Required, handled automatically by Makefile.
- **Every program needs**: `def main() -> int { ... return 0 }`

## Debugging

- **Type errors**: Check error message for expected vs actual types
- **Segfaults**: Verify pointer handling in codegen.rs
- **Runtime errors**: Check stack trace for call history
- **LLVM IR**: `./ws run file.ws --emit-llvm`
- **Build issues**: `make clean-all && make` or `make info` for config
