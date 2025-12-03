# WadeScript - AI Assistant Guide

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
- Built-in functions (print_int, print_float, print_str, print_bool, range)

## Building and Running

**IMPORTANT:** Always use the `ws` tool for building and running WadeScript programs.

### Build the Compiler and Runtime (using Make - Recommended)

The project includes a comprehensive Makefile that handles building both the compiler and runtime library:

```bash
make              # Build compiler + runtime (debug mode)
make release      # Build optimized release version
make test         # Run the test suite
make test-rust    # Run the rust test suite (compiler + runtime)
make examples     # Compile all example programs
make clean        # Clean debug build artifacts
make clean-all    # Clean all build artifacts (debug + release)
make help         # Show all available commands
```

Additional make targets:
```bash
make compiler          # Build only the compiler (debug)
make compiler-release  # Build only the compiler (release)
make runtime           # Build only the runtime library (debug)
make runtime-release   # Build only the runtime library (release)
make check             # Fast code check without building
make fmt               # Format Rust code
make lint              # Run clippy linter
make info              # Show build configuration
make rebuild           # Clean and rebuild from scratch
make install           # Install to ~/.local/bin
```

### Build using Cargo directly:
```bash
cargo build         # Debug build
cargo build --release  # Release build
# or: . "$HOME/.cargo/env" && cargo build
```

Note: The Makefile automatically sets `LLVM_SYS_170_PREFIX` and manages both the compiler binary and runtime static library (`libwadescript_runtime.a`).

### Run a WadeScript program:
```bash
./ws run examples/hello.ws
```

### Compile a WadeScript program:
```bash
./ws build examples/hello.ws
```

### Run the test suite:
```bash
make test
# or
./ws test
```

The `ws` tool handles all necessary environment setup and ensures consistent builds across different systems. The test suite compiles and runs all tests in the `tests/` directory, comparing output against expected results.

## Project Structure

```
wadescript/
├── src/
│   ├── main.rs           # Entry point, handles imports and linking
│   ├── lexer.rs          # Tokenization
│   ├── parser.rs         # Parsing tokens into AST
│   ├── ast.rs            # Abstract Syntax Tree definitions
│   ├── typechecker.rs    # Type checking and validation
│   ├── codegen.rs        # LLVM IR code generation
│   └── runtime/          # Rust runtime library (compiled to staticlib)
│       ├── lib.rs        # Runtime library entry point
│       ├── list.rs       # Dynamic list operations
│       ├── dict.rs       # Hash table dictionary operations
│       └── string.rs     # String manipulation operations
├── examples/
│   ├── hello.ws          # Hello world example
│   ├── fibonacci.ws      # Fibonacci sequence
│   ├── loops.ws          # Loop examples
│   ├── conditions.ws     # Conditional examples
│   ├── dict_test.ws      # Dictionary examples
│   ├── class_demo.ws     # Class examples
│   └── ...               # More examples
└── tests/
    └── test_imports/     # Import system tests
```

## Compilation Pipeline

### WadeScript Program Compilation

1. **Lexing** (lexer.rs): Source code → Tokens
2. **Parsing** (parser.rs): Tokens → AST
3. **Type Checking** (typechecker.rs): Validates types and semantics
4. **Code Generation** (codegen.rs): AST → LLVM IR → Object file (.o)
5. **Linking** (main.rs): Links object file with runtime library using clang → Executable

### Complete Build Process (via Makefile)

When you run `make`, the following happens:

1. **Cargo builds the project** with LLVM_SYS_170_PREFIX set
   - Compiles compiler (src/main.rs, lexer.rs, parser.rs, etc.) → `target/debug/wadescript`
   - Compiles runtime library (src/runtime/*.rs) → `target/debug/libwadescript_runtime.a`

2. **When compiling a WadeScript program** (via `./ws build example.ws`):
   - Compiler parses and type-checks the .ws file
   - Generates LLVM IR with external runtime function declarations (`list_push_i64`, `dict_create`, etc.)
   - LLVM compiles IR to native object file (example.o)
   - Clang links example.o with libwadescript_runtime.a
   - Runtime library provides implementations of external functions
   - Final executable: `./example` (single binary, no external dependencies)

## Key Components

### AST (ast.rs)
Defines the structure of WadeScript programs:
- `Program`: Top-level container with statements and module map
- `Statement`: Variable declarations, functions, classes, control flow
- `Expression`: Literals, operations, function calls, member access
- `Type`: Int, Float, Str, Bool, List, Dict, Array, Custom classes

### Type Checker (typechecker.rs)
- Maintains symbol tables for variables and functions
- Tracks class definitions with fields and methods
- Validates type compatibility (e.g., Float can accept Int)
- Handles module imports and function visibility
- Checks method calls and field access

### Code Generator (codegen.rs)
- Uses inkwell (LLVM bindings for Rust)
- Generates LLVM IR for all WadeScript constructs
- Manages variables as stack-allocated pointers
- Handles runtime function calls (list/dict operations)
- Creates string literals and format strings
- Emits stack trace tracking calls:
  - `push_call_stack(func_name)` at function entry
  - `pop_call_stack()` before all returns (both explicit and implicit)
  - Return values computed before popping to maintain accurate stack traces

### Runtime Libraries (Rust)
The runtime is implemented in Rust and compiled as a static library (`libwadescript_runtime.a`).

**Lists** (src/runtime/list.rs):
- Structure: `{ ptr data, i64 length, i64 capacity }`
- Functions: `list_push_i64`, `list_get_i64`, `list_pop_i64`, `list_set_i64`
- Dynamic resizing (capacity doubles when full, starts at 4)
- C-compatible FFI with `#[no_mangle]` and `extern "C"`
- Memory-safe Rust implementation with unsafe blocks for FFI

**Dictionaries** (src/runtime/dict.rs):
- Structure: `{ ptr buckets, i64 capacity, i64 length }`
- Hash table with separate chaining for collision handling
- Hash function: djb2 algorithm for strings
- Each bucket: Linked list of entries `{ ptr key, i64 value, ptr next }`
- Functions: `dict_create`, `dict_set`, `dict_get`, `dict_has`
- Initial capacity: 16 buckets
- Load factor: 0.75 (rehashes when exceeded)
- O(1) average case for get/set operations
- Automatic rehashing doubles capacity and redistributes entries
- Custom string operations (dup, cmp) for C-string compatibility

**Strings** (src/runtime/string.rs):
- Functions: `str_length`, `str_upper`, `str_lower`, `str_contains`, `str_char_at`
- All functions work with null-terminated C strings (ptr to u8)
- `str_length`: Returns length of string as i64
- `str_upper` / `str_lower`: Allocate and return new strings with case conversion
- `str_contains`: Returns 1 if substring found, 0 otherwise
- `str_char_at`: Returns single-character string at given index (used for iteration)
- Memory allocation via Rust's alloc API for new strings
- UTF-8 aware string operations via Rust's str methods

**Error Handling** (src/runtime/lib.rs):
- Functions: `runtime_error`, `push_call_stack`, `pop_call_stack`
- `runtime_error`: Prints colored error message with stack trace and exits
- `push_call_stack`: Adds function name to global call stack (called at function entry)
- `pop_call_stack`: Removes function from call stack (called before returns)
- Global `CALL_STACK` mutex protects the call stack vector
- Stack traces show function call history when errors occur
- All list and dict operations use `runtime_error` for error reporting
- Provides contextual information (indices, keys, lengths) in error messages

## Type System

**Primitive Types:**
- `int` - 64-bit signed integer
- `float` - 64-bit floating point
- `str` - String (null-terminated C string)
- `bool` - Boolean
- `void` - No return value

**Collection Types:**
- `list[T]` - Dynamic array of type T
- `dict[K, V]` - Dictionary with key type K and value type V
- `array[T, N]` - Fixed-size array (not fully implemented)

**Custom Types:**
- Classes defined with `class` keyword

**Type Compatibility:**
- Float accepts Int (automatic promotion)
- Collections require exact element type matching
- No implicit conversions between primitive types (except Int→Float)

## Module System

Import syntax:
```wadescript
import "path/to/module"
```

- Imports are resolved relative to the importing file
- Functions from imported modules are available in the current scope
- Circular imports are detected and prevented
- Module functions can be called as `module.function()`

## Development Workflow

### Adding a New Feature

1. **Update AST** (ast.rs): Add new Statement/Expression variants
2. **Update Lexer** (lexer.rs): Add new tokens if needed
3. **Update Parser** (parser.rs): Parse new syntax
4. **Update Type Checker** (typechecker.rs): Add type checking logic
5. **Update Code Generator** (codegen.rs): Generate LLVM IR
6. **Update Runtime** (src/runtime/*.rs): Add new runtime functions if needed
7. **Test**: Create test in `tests/` directory (see Testing section)
8. **Build and run**: Use `make` or `make test` to build and verify

**Quick development cycle:**
```bash
make check       # Fast syntax check
make test        # Build and run all tests
make examples    # Verify examples compile
make fmt         # Format code before committing
```

### Testing

WadeScript has a comprehensive assertion-based test suite in the `tests/` directory.

**Run all tests:**
```bash
make test
# or
./ws test
```

**Test system:**
- All test files match `tests/test_*.ws`
- Tests use `assert` statements to verify correctness
- Tests pass if they exit with code 0, fail otherwise
- No need for `.expected` output files

**Adding a new test:**
1. Create `tests/test_feature.ws` with test code using assertions:
```wadescript
def main() -> int {
    # Test your feature
    result: int = my_function(5)
    assert result == 10, "Expected 10"

    # More assertions
    assert condition1
    assert condition2

    return 0  # Test passes
}
```
2. Run it: `./ws run tests/test_feature.ws` or `make test`
3. Test passes if all assertions succeed and program exits with 0

**Writing good tests:**
- Use descriptive assertion messages: `assert x > 0, "x must be positive"`
- Test edge cases and boundary conditions
- Group related assertions together
- Use comments to explain what's being tested

**Current test coverage:**
- Basic types (int, float, bool, str)
- Arithmetic and comparison operators
- Functions and recursion
- Control flow (if/elif/else, while)
- For loops with range() and iterables
- Break and continue statements
- Lists (creation, indexing, methods, iteration)
- Dictionaries (hash table operations, rehashing)
- Strings (methods: upper, lower, contains; iteration; length property)
- Module imports and namespacing
- Compound assignments (+=, -=, *=, /=)
- Increment/decrement operators (++, --)
- Assert statements
- Integration tests combining multiple features

### Debugging

**Emit LLVM IR:**
```bash
./ws run examples/program.ws --emit-llvm
```

**Check type errors:**
Type errors are reported during compilation with clear messages about what went wrong.

**Segfaults:**
- Usually caused by incorrect pointer handling in codegen
- Check that variables are loaded before use
- Verify runtime function signatures match C implementations

## Error Handling

WadeScript provides comprehensive error handling with colored output and detailed messages to help debug issues quickly.

### Parse Errors

When the compiler encounters syntax errors, it provides:
- **Colored error messages** (red for errors, gray for details)
- **Token position** showing where the error occurred
- **Expected vs actual** showing what was expected and what was found

Example:
```
Parse Error: Expected parameter name in function definition
  at token position 32
  got: IntType
```

### Runtime Errors

Runtime errors include detailed contextual information:

**List Operations:**
- **Bounds checking** - Validates indices are within valid range
- **Pop from empty** - Prevents popping from empty lists
- Example: `List index out of bounds: index 10 is out of range for list of length 3`

**Dictionary Operations:**
- **Key validation** - Checks if keys exist before access
- **Null checking** - Validates dictionary and key pointers
- Example: `Dictionary key error: key 'Charlie' not found in dictionary`

### Stack Traces

When runtime errors occur, WadeScript shows the complete call stack:

```
Runtime Error: List index out of bounds: index 10 is out of range for list of length 3

Call stack:
  1. level3
  2. level2
  3. level1
  4. main
```

The stack trace shows:
- Function names in order from most recent (where error occurred) to oldest (main)
- Numbered entries for easy counting of call depth
- Colored output (cyan for "Call stack:", gray for numbers)

**How it works:**
- Each function push/pop is tracked automatically
- Stack is maintained in a global mutex-protected vector
- Error messages print the stack before exiting

**Testing error handling:**
```bash
./ws run examples/test_stack_trace.ws      # Test nested function errors
./ws run examples/test_dict_error.ws       # Test dictionary key errors
./ws run examples/test_bounds_error.ws     # Test list bounds errors
./ws run examples/test_parse_error.ws      # Test parse errors
```

## Common Patterns

### Variable Declaration
```wadescript
x: int = 42
name: str = "Alice"
ages: dict[str, int] = {"Alice": 25}
```

### Functions
```wadescript
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

### Loops
```wadescript
# For loop over list
for item in items {
    print_int(item)
}

# For loop over string
for char in "hello" {
    print_str(char)  # prints each character
}

# While loop
while condition {
    # body
}
```

### Strings
```wadescript
# String properties
s: str = "hello"
len: int = s.length  # 5

# String methods
upper: str = s.upper()         # "HELLO"
lower: str = "WORLD".lower()   # "world"
has: bool = s.contains("ell")  # true

# String iteration
for char in "abc" {
    print_str(char)  # prints: a, b, c
}

# F-strings
name: str = "Alice"
age: int = 25
msg: str = f"Name: {name}, Age: {age}"
```

### Exception Handling
```wadescript
# Basic try/except
try {
    raise ValueError("Something went wrong")
} except ValueError {
    print_str("Caught ValueError")
}

# Multiple except clauses
try {
    # code that might fail
} except ValueError {
    print_str("Value error")
} except KeyError {
    print_str("Key error")
}

# Exception variable binding
try {
    raise RuntimeError("Error message")
} except RuntimeError as e {
    # e is the exception object (pointer)
    # Note: accessing e.message not yet implemented
    print_str("Caught error")
}

# Finally block (always executes)
try {
    # code
} except ValueError {
    # handle
} finally {
    # cleanup code - always runs
}

# Raise exceptions
def divide(a: int, b: int) -> int {
    if b == 0 {
        raise ValueError("Division by zero")
    }
    return a / b
}
```

**Built-in Exception Types:**
- `ValueError` - Invalid value
- `KeyError` - Dictionary key not found
- `IndexError` - List/array index out of bounds (raised automatically)
- `RuntimeError` - General runtime error
- `TypeError` - Type mismatch (raised automatically during type checking)

**Exception System:**
- Uses setjmp/longjmp for efficient exception handling
- Zero overhead when no exception is raised
- Exceptions unwind the stack automatically
- Finally blocks always execute, even if exception occurs

## Important Notes

1. **Always use `./ws` tool** for building and running - it handles environment setup
2. **Main function required**: Every program needs a `def main() -> int` entry point
3. **Runtime libraries**: The runtime is written in Rust (src/runtime/) and compiled as a static library (`libwadescript_runtime.a`) by Cargo
4. **Build system**: Use `make` for a streamlined build experience, or `cargo build` for direct compilation
5. **LLVM version**: Uses LLVM 17 (set via LLVM_SYS_170_PREFIX - handled automatically by Makefile)
6. **Linking**: Final executable links object file with `libwadescript_runtime.a` using clang
7. **Profile matching**: The runtime library path automatically matches the build profile (debug or release)

## Future Enhancements

- Array type full implementation
- More collection methods (map, filter, reduce)
- More string methods (split with list return, replace, trim, starts_with, ends_with)
- String builder for efficient concatenation
- File I/O operations
- Error handling (try/catch)
- Const variables
- Tuple type
- Generics
- Traits/interfaces
- Package manager
- Standard library expansion

## Troubleshooting

**Cargo not found:**
```bash
make              # Uses automatic Cargo detection
# or
export PATH="$HOME/.cargo/bin:$PATH"
```

**LLVM not found:**
```bash
make              # Automatically sets LLVM_SYS_170_PREFIX
# or manually:
export LLVM_SYS_170_PREFIX=/opt/homebrew/opt/llvm@17
```

**Build errors:**
```bash
make clean-all    # Clean all artifacts
make rebuild      # Rebuild from scratch
make info         # Check build configuration
```

**Linking errors:**
- Ensure `libwadescript_runtime.a` exists in target/debug/ or target/release/
- Run `make` to rebuild both compiler and runtime
- Check that clang is installed
- Verify runtime function signatures match declarations in codegen.rs

**Test failures:**
```bash
./ws run tests/test_name.ws     # Run individual test
make test                        # Run all tests
make examples                    # Verify examples compile
```

**Type errors:**
Read the error message carefully - it shows expected vs actual types and the location of the mismatch.

**Runtime changes:**
When modifying src/runtime/*.rs files:
```bash
make runtime      # Rebuild just the runtime library
# or
make              # Rebuild everything
```
