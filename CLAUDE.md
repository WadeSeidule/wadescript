# WadeScript - AI Assistant Guide

## Project Overview

WadeScript is a statically-typed programming language that compiles to native code via LLVM. It features Python-like syntax with strong type checking and compiles to efficient native executables.

**Language Features:**
- Static type system with type inference
- Functions with explicit return types
- Control flow (if/elif/else, while, for loops)
- Data structures (lists, dictionaries, arrays)
- Classes with methods and fields
- Module system with imports
- F-strings for string interpolation
- Built-in functions (print_int, print_float, print_str, print_bool, range)

## Building and Running

**IMPORTANT:** Always use the `ws` tool for building and running WadeScript programs.

### Build the compiler:
```bash
cargo build  # or: . "$HOME/.cargo/env" && cargo build
```

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
│   └── codegen.rs        # LLVM IR code generation
├── runtime/
│   ├── list_runtime.c    # C runtime for dynamic lists
│   ├── list.o            # Compiled list runtime
│   ├── dict_runtime.c    # C runtime for dictionaries
│   └── dict.o            # Compiled dict runtime
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

1. **Lexing** (lexer.rs): Source code → Tokens
2. **Parsing** (parser.rs): Tokens → AST
3. **Type Checking** (typechecker.rs): Validates types and semantics
4. **Code Generation** (codegen.rs): AST → LLVM IR → Object file
5. **Linking** (main.rs): Links object file with runtime libraries using clang

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

### Runtime Libraries
**Lists** (runtime/list_runtime.c):
- Structure: `{ ptr data, i64 length, i64 capacity }`
- Functions: `list_create_i64`, `list_push_i64`, `list_get_i64`, `list_pop_i64`, `list_set_i64`
- Dynamic resizing (capacity doubles when full)

**Dictionaries** (runtime/dict_runtime.c):
- Structure: `{ ptr buckets, i64 capacity, i64 length }`
- Hash table with separate chaining for collision handling
- Hash function: djb2 algorithm for strings
- Each bucket: Linked list of entries `{ ptr key, i64 value, ptr next }`
- Functions: `dict_create`, `dict_set`, `dict_get`, `dict_has`
- Initial capacity: 16 buckets
- Load factor: 0.75 (rehashes when exceeded)
- O(1) average case for get/set operations
- Automatic rehashing doubles capacity and redistributes entries

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
6. **Test**: Create test in `tests/` directory (see Testing section)
7. **Build and run** using `cargo build` and `./ws run`

### Testing

WadeScript has a comprehensive test suite in the `tests/` directory.

**Run all tests:**
```bash
./ws test
```

**Test file structure:**
- Each test: `tests/test_*.ws`
- Expected output: `tests/test_*.expected`
- Test runner compiles, runs, and compares output

**Adding a new test:**
1. Create `tests/test_feature.ws` with test code
2. Run it: `./ws run tests/test_feature.ws`
3. Save output to `tests/test_feature.expected`
4. Verify with `./ws test`

**Current test coverage:**
- Basic types (int, float, bool, str)
- Functions and recursion
- Control flow (if/elif/else, while)
- For loops and range()
- Lists (creation, indexing, methods)
- Dictionaries (hash table operations)
- Comparisons and logical operators
- Module imports
- Integration tests

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

# While loop
while condition {
    # body
}
```

## Important Notes

1. **Always use `./ws` tool** for building and running - it handles environment setup
2. **Main function required**: Every program needs a `def main() -> int` entry point
3. **Runtime libraries**: Changes to list_runtime.c or dict_runtime.c require recompilation to .o files
4. **LLVM version**: Uses LLVM 17 (set via LLVM_SYS_170_PREFIX)
5. **Linking**: Final executable links object file with list.o and dict.o using clang

## Future Enhancements

- HashMap-based dictionary implementation for better performance
- Array type full implementation
- More collection methods (map, filter, reduce)
- String methods and string builder
- File I/O operations
- Error handling (try/catch)
- Generics
- Traits/interfaces

## Troubleshooting

**Cargo not found:**
The `ws` tool handles this - always use `./ws build` or `./ws run`

**LLVM not found:**
The `ws` tool sets LLVM_SYS_170_PREFIX - use `./ws build`

**Linking errors:**
- Ensure runtime/*.o files are present
- Check that clang is installed
- Verify runtime function signatures match declarations in codegen.rs

**Type errors:**
Read the error message carefully - it shows expected vs actual types and the location of the mismatch.
