# WadeScript - AI Assistant Guide

## Development Guide for AI Assistant
- use ./ws util to run/build wadescript
  - wadescript binarys should have the .o extension
- when adding features always add tests to both rust code and wadescript/tests
  - When tests fail always fix the underlying bug. Do not rewrite test to get around the bug.
- when adding or changing any functionality document changes in docs/ dir
- After any implementation try and see if there are ways to optimize. Do not go with an unoptimized solution unless explicitly told to do so.

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
- **CLI Module**: `docs/CLI.md` - Command-line argument parsing
- **HTTP Module**: `docs/HTTP.md` - HTTP client for web requests
- **Language Server**: `docs/LSP.md` - LSP implementation for IDE integration
- **Tuples**: `docs/TUPLES.md` - Tuple types, literals, unpacking, indexing
- **Slices**: `docs/SLICES.md` - Python-style slice syntax for lists and strings
- **Named Arguments**: `docs/NAMED_ARGS.md` - Named arguments and default parameters
- **REPL**: `docs/REPL.md` - Interactive Read-Eval-Print Loop

## Project Overview

WadeScript is a statically-typed programming language that compiles to native code via LLVM. It features Python-like syntax with strong type checking and compiles to efficient native executables.

**Language Features:**
- Static type system with type inference
- Functions with explicit return types
- Named arguments and default parameters
- Control flow (if/elif/else, while, for loops, break/continue)
- Exception handling (try/except/finally, raise)
- Data structures (lists, dictionaries, arrays, tuples)
- Slice syntax for lists and strings (`list[1:5]`, `str[::2]`)
- String methods (upper, lower, contains) and string iteration
- Classes with methods and fields
- Module system with imports
- F-strings for string interpolation
- Compound assignment operators (+=, -=, *=, /=)
- Increment/decrement operators (++, --)
- Assert statements for testing
- Reference counting with automatic memory management
- Built-in functions (print_int, print_float, print_str, print_bool, range)
- Interactive REPL with JIT compilation

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

# IDE Integration
./ws lsp             # Start language server for IDE integration
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
│   ├── jit.rs            # JIT engine for REPL
│   ├── repl.rs           # Interactive REPL
│   ├── runtime_symbols.rs # Centralized runtime symbol registry
│   ├── lsp/              # Language Server Protocol implementation
│   │   ├── mod.rs        # LSP module root
│   │   ├── server.rs     # LSP server (tower-lsp)
│   │   ├── analysis.rs   # Code analysis coordinator
│   │   ├── document.rs   # Document state management
│   │   ├── diagnostics.rs # Error to diagnostic conversion
│   │   └── span.rs       # Span and position utilities
│   └── runtime/          # Rust runtime library
│       ├── lib.rs        # Library exports (for static library)
│       ├── mod.rs        # Module exports (for main binary)
│       ├── list.rs       # Dynamic lists
│       ├── dict.rs       # Hash table dictionaries
│       ├── string.rs     # String operations
│       ├── rc.rs         # Reference counting
│       ├── io.rs         # File I/O operations
│       ├── cli.rs        # CLI argument parsing
│       ├── http.rs       # HTTP client
│       └── exceptions.rs # Exception handling
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
7. **Update Runtime Symbols** (runtime_symbols.rs): Register new runtime functions (see below)
8. **Test**: Create test in `tests/` directory
9. **Build and verify**: `make test`

See `docs/TESTING.md` for testing guidelines.

### Adding a New Runtime Function

When adding a new runtime function that needs to be available in both compiled mode and REPL:

1. **Implement the function** in `src/runtime/*.rs` with `#[no_mangle] pub extern "C"`
2. **Declare it in codegen.rs** (in the appropriate `declare_*_functions` method)
3. **Register it in runtime_symbols.rs**:
   - Add the import at the top of `get_runtime_symbols()`
   - Add a `RuntimeSymbol { name: "func_name", addr: func_name as usize }` entry

This ensures the function is automatically available to the JIT for REPL usage. The centralized registry prevents JIT from falling out of sync with the compiler.

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

### Tuples
```wadescript
# Tuple type and literal
point: (int, int) = (10, 20)
data: (str, int, bool) = ("Alice", 30, True)

# Tuple indexing (compile-time indices)
x: int = point.0
y: int = point.1

# Tuple unpacking
a, b = point
name, age, active = data
```

### Slices
```wadescript
nums: list[int] = [0, 1, 2, 3, 4, 5]

# Basic slicing
sub: list[int] = nums[1:4]     # [1, 2, 3]
first3: list[int] = nums[:3]   # [0, 1, 2]
last3: list[int] = nums[3:]    # [3, 4, 5]

# With step
every2: list[int] = nums[::2]  # [0, 2, 4]
reversed: list[int] = nums[::-1]  # [5, 4, 3, 2, 1, 0]

# String slicing
s: str = "hello world"
hello: str = s[:5]   # "hello"
```

### Named Arguments and Defaults
```wadescript
# Function with default parameters
def greet(name: str = "World", excited: bool = False) -> void {
    if excited {
        print_str(f"Hello, {name}!")
    } else {
        print_str(f"Hello, {name}")
    }
}

# Calling with named arguments
greet()                          # Uses defaults
greet("Alice")                   # Positional
greet(name="Bob")                # Named
greet(excited=True)              # Skip to later param
greet("Charlie", excited=True)   # Mix positional and named
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
