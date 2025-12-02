# WadeScript

A statically-typed, compiled programming language with Python-like syntax, built in Rust and compiled to native code via LLVM.

## Features

- **Python-like syntax** without mandatory whitespace (uses braces for blocks)
- **Static typing** with type inference
- **Compiled to native code** via LLVM
- **Type safety** with compile-time type checking
- Support for:
  - Basic types: `int`, `float`, `bool`, `str`
  - **Dynamic lists**: `list[int]`, `list[str]` with runtime support
  - **Dictionaries**: `dict[str, int]` with hash table implementation
  - **String methods**: `.upper()`, `.lower()`, `.contains()`, `.length`
  - **String iteration**: Iterate over strings character by character
  - Functions with parameters and return types
  - Control flow: `if`/`elif`/`else`, `while`, `for` loops, `break`/`continue`
  - **Python-style iteration** over lists and strings
  - **`range()` function** for numeric iteration
  - **Compound operators**: `+=`, `-=`, `*=`, `/=`, `++`, `--`
  - **Assert statements** for testing
  - **F-strings** for string interpolation
  - **Import system** for code reuse across files
  - Classes with methods and fields
  - Recursion

## Syntax Examples

### Variable Declaration
```wadescript
x: int = 42
name: str = "WadeScript"
pi: float = 3.14159
```

### Functions
```wadescript
def add(a: int, b: int) -> int {
    return a + b
}

def greet(name: str) -> void {
    # Function body
}
```

### Imports
```wadescript
import "math_lib"
import "utils"

def main() -> int {
    result: int = add(5, 10)  # From math_lib.ws
    print_int(result)
    return 0
}
```

The `.ws` extension is optional - it's added automatically. See `IMPORTS.md` for complete documentation.

### Control Flow
```wadescript
if x > 10 {
    # then branch
} elif x > 5 {
    # elif branch
} else {
    # else branch
}

while condition {
    # loop body
}

# For loops - Python-style iteration
numbers: list[int] = [1, 2, 3, 4, 5]
for num in numbers {
    print_int(num)
}

# Iterate over strings
for char in "hello" {
    print_str(char)  # Prints each character
}

# Range function for numeric iteration
for i in range(10) {
    print_int(i)  # Prints 0 through 9
}

# Break and continue
while True {
    if condition {
        break     # Exit loop
    }
    if other {
        continue  # Skip to next iteration
    }
}
```

### Dynamic Lists

WadeScript has full list support with runtime dynamic allocation!

```wadescript
def main() -> int {
    # Create lists with elements
    numbers: list[int] = [1, 2, 3, 4, 5]

    # Get length
    print_int(numbers.length)  # Prints: 5

    # Iterate with for loops
    for num in numbers {
        print_int(num)         # Prints: 1, 2, 3, 4, 5
    }

    # Use range() for numeric iteration
    for i in range(10) {
        print_int(i)           # Prints: 0 through 9
    }

    return 0
}
```

See `LISTS.md` for complete documentation.

### Strings

WadeScript has full string support with methods and iteration!

```wadescript
def main() -> int {
    # String properties
    s: str = "hello"
    len: int = s.length        # Get string length: 5

    # String methods
    upper: str = s.upper()         # Convert to uppercase: "HELLO"
    lower: str = "WORLD".lower()   # Convert to lowercase: "world"
    has: bool = s.contains("ell")  # Check if contains substring: true

    # String iteration - iterate over characters
    for char in "abc" {
        print_str(char)    # Prints: a, b, c (one per line)
    }

    # F-strings for string interpolation
    name: str = "Alice"
    age: int = 25
    msg: str = f"Name: {name}, Age: {age}"
    print_str(msg)  # Prints: Name: Alice, Age: 25

    return 0
}
```

**Available String Methods:**
- `.length` - Returns the length of the string (property)
- `.upper()` - Returns a new string with all characters in uppercase
- `.lower()` - Returns a new string with all characters in lowercase
- `.contains(substring: str) -> bool` - Checks if the string contains a substring

### Printing to Console

WadeScript provides built-in print functions:

```wadescript
def main() -> int {
    print_int(42)              # Prints: 42
    print_float(3.14)          # Prints: 3.140000
    print_str("Hello!")        # Prints: Hello!
    print_bool(True)           # Prints: True

    x: int = 10 + 5
    print_int(x)               # Prints: 15

    return 0
}
```

### Classes
```wadescript
class Person {
    def init(self: Person, name: str) -> void {
        # constructor
    }

    def greet(self: Person) -> void {
        # method
    }
}
```

## Building

Requires Rust and LLVM 17.

### Using Make (Recommended)

```bash
make              # Build compiler and runtime (debug mode)
make release      # Build optimized release version
make test         # Run test suite
make examples     # Compile all example programs
make clean        # Clean build artifacts
make help         # Show all available targets
```

### Using Cargo Directly

```bash
cargo build --release
```

Note: LLVM 17 must be installed. On macOS with Homebrew: `brew install llvm@17`

## Testing

Run the comprehensive test suite:

```bash
make test
# or
./ws test
```

See `TESTING.md` for details on the test suite and how to add new tests.

## Usage

Compile and run a WadeScript program:

```bash
./ws run examples/hello.ws
```

Or just compile:

```bash
./ws build examples/hello.ws
```

This produces an executable with the same name as the input file (without extension).

To emit LLVM IR instead:

```bash
./target/debug/wadescript examples/hello.ws --emit-llvm
```

## Examples

See the `examples/` directory for sample programs:
- `hello.ws` - Simple variable operations
- `fibonacci.ws` - Recursive fibonacci calculator
- `factorial.ws` - Recursive factorial calculator
- `loops.ws` - While loop example
- `conditions.ws` - If/elif/else examples
- `print_demo.ws` - Console printing demonstration
- **`for_loops_demo.ws`** - Python-style for loops with lists
- **`range_demo.ws`** - Using range() for numeric iteration
- **`list_methods.ws`** - List methods: push(), pop(), get()
- **`import_demo.ws`** - Using imports for code reuse
- **`multi_import.ws`** - Multiple imports and subdirectories
- `lists_demo.ws` - Dynamic list operations
- `comprehensive.ws` - Prime counting and exponentiation

Library files:
- `math_lib.ws` - Math utility functions
- `lib/list_utils.ws` - List processing functions

## Language Comparison

WadeScript vs Python:

```python
# Python
def fibonacci(n):
    if n <= 1:
        return n
    return fibonacci(n - 1) + fibonacci(n - 2)
```

```wadescript
# WadeScript
def fibonacci(n: int) -> int {
    if n <= 1 {
        return n
    }
    return fibonacci(n - 1) + fibonacci(n - 2)
}
```

Key differences:
- Type annotations are required
- Braces `{}` instead of indentation
- Explicit return types with `->`

## Project Status

WadeScript has complete implementations of:
- ✅ Core types (int, float, bool, str)
- ✅ Functions and recursion
- ✅ Control flow (if/elif/else, while, for, break/continue)
- ✅ Dynamic lists with full runtime support
- ✅ Dictionaries with hash table implementation
- ✅ String methods (upper, lower, contains, length)
- ✅ String iteration (character-by-character)
- ✅ Python-style iteration over lists and strings
- ✅ Compound operators (+=, -=, *=, /=, ++, --)
- ✅ Assert statements for testing
- ✅ F-strings for string interpolation
- ✅ Classes with methods and fields
- ✅ Import system for code reuse
- ✅ Comprehensive test suite (14 tests, 100% passing)
- ✅ 117 unit tests (22 runtime + 95 compiler)

See `DATA_STRUCTURES_STATUS.md` for detailed implementation status.
See `IMPORTS.md` for import system documentation.
