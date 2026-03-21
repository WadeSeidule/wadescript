# WadeScript Language Reference

Quick reference for WadeScript syntax and features. For detailed documentation on specific topics, see the linked docs.

## Program Structure

Every WadeScript program requires a `main` function:

```wadescript
def main() -> int {
    print("Hello, World!")
    return 0
}
```

## Variables and Types

### Primitives
- `int` (i64), `float` (f64), `str` (C string), `bool`, `void`

```wadescript
x: int = 42
pi: float = 3.14
name: str = "Alice"
active: bool = True
```

### Collections
- `list[T]`, `dict[K, V]`, `array[T, N]`, `(T1, T2, ...)` (tuples)

```wadescript
nums: list[int] = [1, 2, 3]
ages: dict[str, int] = {"Alice": 25}
point: (int, int) = (10, 20)
```

**Type compatibility:** Float accepts Int (automatic promotion). Collections require exact type matching.

See `DATA_STRUCTURES.md`, `TUPLES.md` for details.

## Functions

```wadescript
def add(a: int, b: int) -> int {
    return a + b
}

# Default parameters and named arguments
def greet(name: str = "World", excited: bool = False) -> void {
    if excited {
        print(f"Hello, {name}!")
    } else {
        print(f"Hello, {name}")
    }
}

greet()                          # Uses defaults
greet("Alice")                   # Positional
greet(name="Bob")                # Named
greet(excited=True)              # Skip to later param
greet("Charlie", excited=True)   # Mix positional and named
```

See `NAMED_ARGS.md` for details.

## Classes

```wadescript
class Person {
    name: str
    age: int

    def greet(self: Person) -> void {
        print(self.name)
    }
}

p: Person = Person("Alice", 25)
p.greet()
```

## Control Flow

```wadescript
# If/elif/else
if x > 0 {
    print("positive")
} elif x < 0 {
    print("negative")
} else {
    print("zero")
}

# While loop
while condition {
    # body (supports break/continue)
}

# For loop (lists, ranges, strings)
for item in items { print(item) }
for i in range(10) { print(i) }
for char in "hello" { print(char) }
```

See `FOR_LOOPS.md` for details.

## Strings

```wadescript
s: str = "hello"
len: int = s.length           # Property
upper: str = s.upper()        # Method
has: bool = s.contains("ell") # Method

# F-strings
msg: str = f"Name: {name}, Age: {age}"
```

## Built-in Functions

### Polymorphic (work with all types)
- `str(value)` - Convert any value to string
- `print(value)` - Print any value

### Type-specific (legacy)
- `print_int(n)`, `print_float(f)`, `print_str(s)`, `print_bool(b)`

### Other
- `range(n)` / `range(start, end)` / `range(start, end, step)` - Generate integer sequences
- `assert` - Test assertions

See `STR_PRINT.md` for details.

## Tuples

```wadescript
point: (int, int) = (10, 20)
x: int = point.0         # Indexing (compile-time)
a, b = point              # Unpacking
```

See `TUPLES.md` for details.

## Slices

```wadescript
nums: list[int] = [0, 1, 2, 3, 4, 5]
sub: list[int] = nums[1:4]       # [1, 2, 3]
every2: list[int] = nums[::2]    # [0, 2, 4]
rev: list[int] = nums[::-1]      # [5, 4, 3, 2, 1, 0]

s: str = "hello world"
hello: str = s[:5]               # "hello"
```

See `SLICES.md` for details.

## Operators

- Arithmetic: `+`, `-`, `*`, `/`, `%`
- Comparison: `==`, `!=`, `<`, `>`, `<=`, `>=`
- Logical: `and`, `or`, `not`
- Compound assignment: `+=`, `-=`, `*=`, `/=`
- Increment/decrement: `++`, `--`

## Exception Handling

```wadescript
try {
    raise ValueError("something went wrong")
} except ValueError as e {
    print("caught error")
} except KeyError {
    print("key error")
} finally {
    print("cleanup")
}
```

See `EXCEPTION_SYSTEM.md` for details.

## Module System

```wadescript
import "path/to/module"
Module.function()
```

### Standard Library Modules

```wadescript
# CLI arguments
import "cli"
args: list[str] = cli.get_args()

# HTTP requests
import "http"
response: HttpResponse = http.get("https://example.com")
print(response.status)
print(response.body)

# File I/O
import "io"
handle: int = io.open("file.txt", "r")
content: str = io.read(handle)
io.close(handle)
```

See `IMPORTS.md`, `CLI.md`, `HTTP.md` for details.

## Memory Management

Automatic reference counting (RC) with multi-phase optimization:
- Phase 1: Basic RC with inline operations
- Phase 2: Move semantics + last-use analysis
- Phase 3: Escape analysis for non-escaping variables
- Phase 4b: Loop hoisting for loop-invariant RC operations

Non-escaping local variables have zero RC overhead. Overall ~5-8% overhead vs non-RC baseline.

See `RC_IMPLEMENTATION.md`, `RC_LOOP_HOISTING.md`, `BENCHMARK_RESULTS.md` for details.
