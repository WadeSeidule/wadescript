# WadeScript Quick Start

## What is WadeScript?

WadeScript is a statically-typed, compiled programming language with Python-like syntax that compiles directly to native code via LLVM. It offers the familiarity of Python with the performance of compiled languages.

## Installation

```bash
# Already installed!
cargo build --release
```

## Your First Program

Create a file `hello.ws`:

```wadescript
def main() -> int {
    x: int = 42
    y: int = 10
    return x + y
}
```

Compile and run:

```bash
./target/release/wadescript hello.ws
./hello
echo $?  # Prints: 52
```

## Key Features

### Static Typing
Every variable and function must declare its type:

```wadescript
name: str = "Wade"
age: int = 25
pi: float = 3.14159
is_active: bool = True
```

### Functions
Functions must specify parameter types and return type:

```wadescript
def add(a: int, b: int) -> int {
    return a + b
}

def greet(name: str) -> void {
    # void functions return nothing
}
```

### Control Flow

**If/Elif/Else:**
```wadescript
if x > 10 {
    # do something
} elif x > 5 {
    # do something else
} else {
    # default case
}
```

**While Loops:**
```wadescript
i: int = 0
while i < 10 {
    i = i + 1
}
```

### Recursion
```wadescript
def factorial(n: int) -> int {
    if n <= 1 {
        return 1
    }
    return n * factorial(n - 1)
}
```

### Boolean Logic
```wadescript
def check(x: int, y: int) -> bool {
    return (x > 0) and (y > 0) or (x == y)
}
```

### Printing to Console
WadeScript provides built-in functions to print values:

```wadescript
def main() -> int {
    # Print different types
    print_int(42)
    print_float(3.14159)
    print_str("Hello, WadeScript!")
    print_bool(True)

    # Print expressions
    x: int = 10 + 5
    print_int(x)  # Prints: 15

    # Print function results
    result: bool = 10 > 5
    print_bool(result)  # Prints: True

    return 0
}
```

Available print functions:
- `print_int(int)` - Print an integer
- `print_float(float)` - Print a floating point number
- `print_str(str)` - Print a string
- `print_bool(bool)` - Print True or False

## Differences from Python

| Feature | Python | WadeScript |
|---------|--------|------------|
| Typing | Optional | **Required** |
| Blocks | Indentation | **Braces `{}`** |
| Return type | Implicit | **Explicit `->`** |
| Compilation | Interpreted | **Compiled to native** |
| Performance | Slower | **Much faster** |

## Examples

Check out the `examples/` directory:
- `hello.ws` - Basic arithmetic
- `fibonacci.ws` - Recursive Fibonacci
- `factorial.ws` - Recursive factorial
- `loops.ws` - While loop example
- `conditions.ws` - If/elif/else logic
- `print_demo.ws` - Console printing demonstration
- `comprehensive.ws` - Prime counting and exponentiation

## Viewing LLVM IR

To see the generated LLVM IR:

```bash
./target/release/wadescript examples/hello.ws --emit-llvm
```

## Current Limitations

- No for loops (use while instead)
- No arrays or lists
- No string operations beyond literals
- Classes are parsed but not fully implemented
- Exit codes are limited to 0-255 (return values mod 256)

## Next Steps

Try modifying the examples or create your own WadeScript programs! The language currently supports:
- ✅ Integer and float arithmetic
- ✅ Boolean logic
- ✅ Recursion
- ✅ While loops
- ✅ If/elif/else
- ✅ Function calls
- ✅ Console output with print functions
- ✅ Type checking
- ✅ Native code generation

Happy coding! 🚀
