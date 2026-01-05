# str() and print() Functions

WadeScript provides two polymorphic built-in functions for converting values to strings and printing them.

## str(value) -> str

Converts any value to its string representation.

### Supported Types

| Type | Example | Result |
|------|---------|--------|
| int | `str(42)` | `"42"` |
| float | `str(3.14)` | `"3.14"` |
| bool | `str(True)` | `"True"` |
| str | `str("hello")` | `"hello"` |
| list[T] | `str([1, 2, 3])` | `"[1, 2, 3]"` |
| dict[K, V] | `str({"a": 1})` | `'{"a": 1}'` |
| Classes | `str(Person("Alice", 30))` | `'Person(name="Alice", age=30)'` |

### Basic Examples

```wadescript
# Convert primitives to strings
s: str = str(42)         # "42"
s: str = str(-123)       # "-123"
s: str = str(3.14)       # "3.14"
s: str = str(True)       # "True"
s: str = str(False)      # "False"

# String identity
s: str = str("hello")    # "hello"

# Lists
nums: list[int] = [1, 2, 3]
s: str = str(nums)       # "[1, 2, 3]"
s: str = str([])         # "[]"

# Dicts
ages: dict[str, int] = {"Alice": 30, "Bob": 25}
s: str = str(ages)       # '{"Alice": 30, "Bob": 25}'
```

### Class Representation

Classes are automatically converted to a representation showing the class name and all field values:

```wadescript
class Person {
    name: str
    age: int
}

class Point {
    x: int
    y: int
}

def main() -> int {
    p: Person = Person("Alice", 30)
    print_str(str(p))  # Person(name="Alice", age=30)

    pt: Point = Point(10, 20)
    print_str(str(pt)) # Point(x=10, y=20)

    return 0
}
```

Field formatting:
- **int**: Displayed as decimal number
- **float**: Displayed with decimal point
- **bool**: Displayed as `True` or `False`
- **str**: Displayed quoted (e.g., `"Alice"`)
- **Other types**: Displayed as `<...>`

## print(value) -> void

Prints any value to stdout with a trailing newline. Internally converts the value to a string using `str()`.

### Examples

```wadescript
print(42)           # Prints: 42
print(3.14)         # Prints: 3.14
print(True)         # Prints: True
print("hello")      # Prints: hello
print([1, 2, 3])    # Prints: [1, 2, 3]

class Point {
    x: int
    y: int
}

pt: Point = Point(10, 20)
print(pt)           # Prints: Point(x=10, y=20)
```

## Implementation Notes

- Both functions use compile-time type dispatch (no runtime overhead for type detection)
- String values passed to `str()` are returned as-is (no copy)
- List, dict, and class conversions allocate new memory for the result string
- The `print()` function is equivalent to `print_str(str(value))`

## Comparison with Type-Specific Print Functions

WadeScript also provides type-specific print functions:

```wadescript
# Type-specific (faster, no allocation)
print_int(42)
print_str("hello")
print_bool(True)
print_float(3.14)

# Generic (uses str() conversion)
print(42)
print("hello")
print(True)
print(3.14)
```

Use `print()` for convenience and debugging. Use type-specific functions when performance matters.
