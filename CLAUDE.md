# WadeScript - AI Assistant Guide

## Documentation Location

**All detailed documentation is in the `docs/` directory.**

- **Quick Start**: `docs/QUICKSTART.md` - Get started quickly
- **Building**: `docs/BUILD.md` - Build system and compilation details
- **Testing**: `docs/TESTING.md` - Test suite and writing tests
- **Data Structures**: `docs/DATA_STRUCTURES.md` - Lists, dicts, arrays
- **Lists**: `docs/LISTS.md` - List implementation details
- **For Loops**: `docs/FOR_LOOPS.md` - For loop implementation
- **Imports**: `docs/IMPORTS.md` - Module system details
- **Exceptions**: `docs/EXCEPTION_SYSTEM.md` - Exception handling system
- **RC Implementation**: `docs/RC_IMPLEMENTATION.md` - Reference counting internals
- **Benchmarks**: `docs/BENCHMARK_RESULTS.md` - Performance benchmarks and optimizations
- **Test Summary**: `docs/TEST_SUITE_SUMMARY.md` - Test coverage overview

## Project Overview

WadeScript is a statically-typed programming language that compiles to native code via LLVM. It features Python-like syntax with strong type checking and compiles to efficient native executables.

**Language Features:**
- Static type system with type inference
- Functions with explicit return types
- Control flow (if/elif/else, while, for loops, break/continue)
- Exception handling (try/except/finally, raise)
- Data structures (lists, dictionaries, arrays)
- String methods (upper, lower, contains) and string iteration
- Classes with methods and fields
- Module system with imports
- F-strings for string interpolation
- Compound assignment operators (+=, -=, *=, /=)
- Increment/decrement operators (++, --)
- Assert statements for testing
- Reference counting with automatic memory management
- Built-in functions (print_int, print_float, print_str, print_bool, range)

## Quick Reference

### Essential Commands

```bash
# Build and test
make                 # Build compiler + runtime (debug)
make test            # Run all tests
./ws run file.ws     # Run a WadeScript program
./ws build file.ws   # Compile to executable

# Development
make check           # Fast syntax check
make fmt             # Format Rust code
make examples        # Compile all examples
```

See `docs/BUILD.md` for complete build documentation.

### Project Structure

```
wadescript/
├── src/
│   ├── main.rs           # Entry point, imports, linking
│   ├── lexer.rs          # Tokenization
│   ├── parser.rs         # Parsing
│   ├── ast.rs            # Abstract Syntax Tree
│   ├── typechecker.rs    # Type checking
│   ├── codegen.rs        # LLVM IR generation (includes RC optimizations)
│   └── runtime/          # Rust runtime library
│       ├── lib.rs        # Error handling, call stack
│       ├── list.rs       # Dynamic lists
│       ├── dict.rs       # Hash table dictionaries
│       └── string.rs     # String operations
├── docs/                 # All detailed documentation
├── examples/             # Example WadeScript programs
├── tests/                # Test suite
└── benchmarks/           # Performance benchmarks
```

### Compilation Pipeline

1. **Lexing** (lexer.rs): Source → Tokens
2. **Parsing** (parser.rs): Tokens → AST
3. **Type Checking** (typechecker.rs): Type validation
4. **Code Generation** (codegen.rs): AST → LLVM IR → Object file
5. **Linking** (main.rs): Object file + runtime → Executable

See `docs/BUILD.md` for detailed compilation process.

## Key Components

### AST (ast.rs)
- `Program`: Top-level container with statements and module map
- `Statement`: Variable declarations, functions, classes, control flow
- `Expression`: Literals, operations, function calls, member access
- `Type`: Int, Float, Str, Bool, List, Dict, Array, Custom classes

### Type Checker (typechecker.rs)
- Symbol tables for variables and functions
- Class definitions with fields and methods
- Type compatibility validation (Float accepts Int)
- Module imports and function visibility

### Code Generator (codegen.rs)
- Uses inkwell (LLVM bindings for Rust)
- Generates LLVM IR for all WadeScript constructs
- Implements reference counting with optimizations:
  - **Phase 1**: Basic RC with inline operations
  - **Phase 2**: Move semantics + last-use analysis
  - **Phase 3**: Escape analysis for non-escaping variables
- Stack trace tracking (push_call_stack/pop_call_stack)
- See `docs/RC_IMPLEMENTATION.md` for RC details

### Runtime (src/runtime/)
Rust-based runtime compiled as static library (`libwadescript_runtime.a`).

- **Lists**: Dynamic arrays with automatic resizing
- **Dicts**: Hash tables with separate chaining
- **Strings**: UTF-8 string operations (upper, lower, contains, char_at)
- **Error Handling**: Colored errors with stack traces

See `docs/DATA_STRUCTURES.md` for implementation details.

## Type System

**Primitives**: `int` (i64), `float` (f64), `str` (C string), `bool`, `void`
**Collections**: `list[T]`, `dict[K, V]`, `array[T, N]`
**Custom**: Classes

**Type Compatibility:**
- Float accepts Int (automatic promotion)
- Collections require exact type matching

## Development Workflow

### Adding a New Feature

1. **Update AST** (ast.rs): Add Statement/Expression variants
2. **Update Lexer** (lexer.rs): Add tokens if needed
3. **Update Parser** (parser.rs): Parse new syntax
4. **Update Type Checker** (typechecker.rs): Add type checking
5. **Update Code Generator** (codegen.rs): Generate LLVM IR
6. **Update Runtime** (src/runtime/*.rs): Add runtime functions if needed
7. **Test**: Create test in `tests/` directory
8. **Build and verify**: `make test`

See `docs/TESTING.md` for testing guidelines.

### Testing

```bash
make test            # Run all tests
make test-rust       # Run all rust tests
./ws test            # Alternative test runner
./ws run test.ws     # Run individual test
```

When making changes ALWAYS run `make test test-rust` to verify no regressions!

**Test structure:**
- Tests in `tests/test_*.ws`
- Use `assert` statements for validation
- Exit with 0 for pass, non-zero for fail
- No `.expected` files needed

See `docs/TESTING.md` and `docs/TEST_SUITE_SUMMARY.md` for details.

### Debugging

```bash
./ws run file.ws --emit-llvm   # Emit LLVM IR
make check                      # Fast syntax check
```

**Common issues:**
- **Type errors**: Check error message for expected vs actual types
- **Segfaults**: Verify pointer handling in codegen.rs
- **Runtime errors**: Check stack trace for call history

## Common Patterns

### Variables and Functions
```wadescript
# Variables
x: int = 42
name: str = "Alice"
ages: dict[str, int] = {"Alice": 25}

# Functions
def add(a: int, b: int) -> int {
    return a + b
}
```

### Classes
```wadescript
class Person {
    name: str
    age: int

    def greet(self: Person) -> void {
        print_str(self.name)
    }
}

def main() -> int {
    p: Person = Person("Alice", 25)
    p.greet()
    return 0
}
```

### Control Flow
```wadescript
# If/elif/else
if x > 0 {
    print_str("positive")
} elif x < 0 {
    print_str("negative")
} else {
    print_str("zero")
}

# While loop
while condition {
    # body
}

# For loop
for item in items {
    print_int(item)
}

# For loop over string
for char in "hello" {
    print_str(char)
}
```

See `docs/FOR_LOOPS.md` for for-loop implementation details.

### Strings
```wadescript
s: str = "hello"
len: int = s.length           # Property
upper: str = s.upper()        # Method
has: bool = s.contains("ell") # Method

# F-strings
msg: str = f"Name: {name}, Age: {age}"
```

### Exception Handling
```wadescript
# Basic try/except
try {
    raise ValueError("error")
} except ValueError {
    print_str("caught")
}

# Multiple except clauses
try {
    # code
} except ValueError as e {
    # handle ValueError
} except KeyError {
    # handle KeyError
} finally {
    # always runs
}
```

See `docs/EXCEPTION_SYSTEM.md` for exception system details.

### Module System
```wadescript
import "path/to/module"

# Call imported function
Module.function()
```

See `docs/IMPORTS.md` for module system details.

## Memory Management

WadeScript uses automatic reference counting (RC) with three-phase optimization:

- **Phase 1**: Basic RC with inline operations
- **Phase 2**: Move semantics for returns + last-use analysis
- **Phase 3**: Escape analysis for non-escaping variables

**Performance**: ~5-8% overhead (comparable to Swift's ARC)
**Zero-cost**: Non-escaping local variables have NO RC overhead

See `docs/RC_IMPLEMENTATION.md` and `docs/BENCHMARK_RESULTS.md` for details.

## Important Notes

1. **Use `./ws` tool** for building and running
2. **Main function required**: `def main() -> int`
3. **Runtime**: Rust static library linked with executables
4. **LLVM 17**: Handled automatically by Makefile
5. **Build system**: Use `make` for streamlined builds

## Troubleshooting

```bash
# Build issues
make clean-all       # Clean everything
make rebuild         # Rebuild from scratch
make info            # Show build configuration

# Test issues
./ws run test.ws     # Run individual test
make test            # Run all tests

# Runtime issues
make runtime         # Rebuild runtime library
```

**Common errors:**
- **Cargo not found**: Run `make` (auto-detects Cargo)
- **LLVM not found**: Run `make` (auto-sets LLVM path)
- **Linking errors**: Run `make` to rebuild runtime
- **Type errors**: Read error message carefully

See `docs/BUILD.md` for detailed troubleshooting.

## Performance

**Benchmark results** (see `docs/BENCHMARK_RESULTS.md`):
- Baseline operations: <0.01s for 50K iterations
- RC optimizations: ~70% reduction in RC operations
- Non-escaping variables: Zero RC overhead
- Overall: ~5-8% overhead vs non-RC baseline

**Run benchmarks:**
```bash
./ws build benchmarks/bench_rc_performance.ws
/usr/bin/time -p ./bench_rc_performance

./ws build benchmarks/bench_phase3_escape.ws
/usr/bin/time -p ./bench_phase3_escape
```

## References

- Repository structure: See above
- Detailed docs: See `docs/` directory
- Example code: See `examples/` directory
- Test suite: See `tests/` directory
- Benchmarks: See `benchmarks/` directory
