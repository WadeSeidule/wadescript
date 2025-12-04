# WadeScript REPL

The WadeScript REPL (Read-Eval-Print Loop) provides an interactive environment for experimenting with WadeScript code. It uses LLVM JIT compilation to execute code immediately.

## Starting the REPL

```bash
./ws repl
```

Or directly:

```bash
./target/debug/wadescript repl
```

## Features

### Immediate Execution
Code is compiled and executed immediately using LLVM's JIT (Just-In-Time) compilation engine. This provides native execution speed for your interactive sessions.

### Multi-line Input
The REPL automatically detects incomplete statements by counting braces, brackets, and parentheses. When you start a function or control structure, you can continue on multiple lines:

```
>>> def factorial(n: int) -> int {
...     if n <= 1 {
...         return 1
...     }
...     return n * factorial(n - 1)
... }
>>> print_int(factorial(5))
120
```

### Function Persistence
Functions defined in one input are available in subsequent inputs:

```
>>> def greet(name: str) -> void {
...     print_str("Hello, ")
...     print_str(name)
... }
>>> greet("World")
Hello,
World
```

### All WadeScript Features
The REPL supports all WadeScript language features:

- **Functions**: Define and call functions with parameters and return values
- **Control Flow**: if/elif/else, while loops, for loops
- **Data Structures**: Lists, dictionaries
- **Built-in Functions**: print_int, print_str, print_float, print_bool, range()
- **Booleans**: Use Python-style `True` and `False` (capitalized)
- **Recursion**: Full support for recursive functions
- **String Operations**: String literals and concatenation

## Example Session

```
$ ./ws repl
Starting WadeScript REPL...
WadeScript REPL v0.1.0
Type 'exit' or Ctrl+D to quit

>>> print_str("Hello, REPL!")
Hello, REPL!
>>> def add(a: int, b: int) -> int {
...     return a + b
... }
>>> print_int(add(10, 20))
30
>>> def fib(n: int) -> int {
...     if n <= 1 {
...         return n
...     }
...     return fib(n - 1) + fib(n - 2)
... }
>>> print_int(fib(10))
55
>>> def sum_list() -> void {
...     nums: list[int] = [1, 2, 3, 4, 5]
...     total: int = 0
...     for n in nums {
...         total = total + n
...     }
...     print_int(total)
... }
>>> sum_list()
15
>>> exit
Goodbye!
```

## Limitations

### Variable Scope
Variables declared in one input do not persist to subsequent inputs. Each input runs in its own function scope:

```
>>> x: int = 42
>>> print_int(x)
Error: Undefined variable 'x'
```

To work with persistent data, wrap your code in functions:

```
>>> def work_with_data() -> void {
...     x: int = 42
...     y: int = x * 2
...     print_int(y)
... }
>>> work_with_data()
84
```

### No Import Support
The REPL currently does not support import statements. All code must be self-contained.

## Keyboard Shortcuts

- **Enter**: Submit current line (or continue multi-line input if incomplete)
- **Ctrl+C**: Cancel current input and start fresh
- **Ctrl+D**: Exit the REPL (same as typing `exit`)
- **Up/Down Arrow**: Navigate command history (interactive mode only)

## Non-Interactive Mode

The REPL can also accept piped input for scripting:

```bash
echo -e 'print_int(42)\nexit' | ./ws repl
```

This is useful for testing or automation.

## Technical Details

### JIT Compilation
The REPL uses LLVM's ExecutionEngine to compile WadeScript code to native machine code on-the-fly. Each input is:

1. Parsed into an AST
2. Type-checked
3. Compiled to LLVM IR
4. JIT-compiled to native code
5. Executed immediately

### Runtime Symbol Resolution
All WadeScript runtime functions (list operations, dictionary operations, string functions, I/O, etc.) are registered with the JIT engine, allowing seamless interoperability between your REPL code and the runtime library.

### Function Linking
When you define a function in one REPL input and call it from another, the JIT engine automatically links the calls across compilation units. This is achieved by declaring previously-defined functions as external symbols in each new compilation unit.

## Troubleshooting

### "Failed to start REPL"
Ensure that LLVM is properly installed and that the wadescript binary was built successfully:
```bash
cargo build
```

### Segmentation Fault
If you encounter a segfault, it may be due to:
- Recursive functions without proper base cases
- Invalid memory access in complex data structures

Try simplifying your code to isolate the issue.

### Type Errors
Type errors are caught at compile time. The REPL will display the error message and continue, allowing you to correct your code:

```
>>> x: int = "hello"
Error: Type mismatch: expected int, got str
>>>
```
