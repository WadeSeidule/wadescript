# Tuples in WadeScript

Tuples are heterogeneous fixed-size collections that allow grouping values of different types together. They are useful for returning multiple values from functions, destructuring data, and grouping related values.

## Syntax

### Type Declaration

Tuple types are declared using parentheses with comma-separated types:

```wadescript
# Two-element tuple
point: (int, int) = (10, 20)

# Mixed-type tuple
data: (str, int, bool) = ("Alice", 30, True)

# Three-element tuple
coords: (float, float, float) = (1.5, 2.5, 3.5)
```

### Tuple Literals

Create tuples using parentheses with comma-separated values:

```wadescript
# Simple tuple
point: (int, int) = (10, 20)

# Mixed types
person: (str, int) = ("Bob", 25)

# Expression values
x: int = 5
y: int = 10
computed: (int, int) = (x * 2, y + 3)
```

### Accessing Tuple Elements

Access elements using dot notation with numeric indices (0-based):

```wadescript
point: (int, int) = (10, 20)

# Access elements
x: int = point.0    # 10
y: int = point.1    # 20

# Mixed-type access
data: (str, int, bool) = ("Alice", 30, True)
name: str = data.0       # "Alice"
age: int = data.1        # 30
active: bool = data.2    # True
```

### Tuple Unpacking

Unpack tuples into individual variables:

```wadescript
point: (int, int) = (10, 20)

# Unpack into variables
x, y = point
# x is 10, y is 20

# Mixed-type unpacking
data: (str, int, bool) = ("Alice", 30, True)
name, age, active = data
# name is "Alice", age is 30, active is True
```

## Functions Returning Tuples

Functions can return tuples to return multiple values:

```wadescript
def get_point() -> (int, int) {
    return (100, 200)
}

def get_person() -> (str, int) {
    return ("Bob", 25)
}

def main() -> int {
    # Store in tuple variable
    point: (int, int) = get_point()
    x: int = point.0
    y: int = point.1

    # Direct unpacking
    px, py = get_point()

    # Mixed types
    name, age = get_person()

    return 0
}
```

## Use Cases

### Returning Multiple Values

```wadescript
def divide_with_remainder(a: int, b: int) -> (int, int) {
    quotient: int = a / b
    remainder: int = a % b
    return (quotient, remainder)
}

def main() -> int {
    q, r = divide_with_remainder(17, 5)
    # q is 3, r is 2
    return 0
}
```

### Swapping Values

```wadescript
def main() -> int {
    a: int = 10
    b: int = 20

    # Create tuple and unpack to swap
    temp: (int, int) = (b, a)
    a, b = temp
    # a is now 20, b is now 10

    return 0
}
```

### Grouping Related Data

```wadescript
def process_user() -> (str, int, bool) {
    name: str = "Alice"
    age: int = 30
    verified: bool = True
    return (name, age, verified)
}

def main() -> int {
    user_name, user_age, is_verified = process_user()

    if is_verified {
        print_str(f"Verified user: {user_name}")
    }

    return 0
}
```

## Tuple Operations

### Using in Expressions

```wadescript
point: (int, int) = (5, 10)

# Arithmetic with tuple elements
sum: int = point.0 + point.1        # 15
product: int = point.0 * point.1    # 50

# Comparison
if point.0 < point.1 {
    print_str("x is less than y")
}
```

### Nested Access

```wadescript
def get_bounds() -> (int, int) {
    return (0, 100)
}

def main() -> int {
    # Access directly from function return
    min_val: int = get_bounds().0
    max_val: int = get_bounds().1

    return 0
}
```

## Type Checking

The type checker enforces:

1. **Type matching**: Tuple element types must match declaration
2. **Index bounds**: Tuple indices must be valid at compile time
3. **Unpacking count**: Number of variables must match tuple size

```wadescript
# Error: wrong type
point: (int, int) = (10, "twenty")  # Type error

# Error: index out of bounds
point: (int, int) = (10, 20)
z: int = point.2  # Error: only indices 0-1 valid

# Error: unpacking mismatch
point: (int, int) = (10, 20)
x, y, z = point  # Error: 3 names but tuple has 2 elements
```

## Implementation Notes

- Tuples are represented as LLVM struct types
- Elements are stored inline (no heap allocation)
- Index access compiles to struct field extraction
- Unpacking compiles to multiple extractions

## See Also

- [Data Structures](DATA_STRUCTURES.md) - Lists, dicts, arrays
- [Functions](QUICKSTART.md) - Function definitions
