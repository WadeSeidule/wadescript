# WadeScript Data Structures

## Overview

WadeScript supports four collection types, all statically typed:

| Type | Syntax | Storage | Example |
|------|--------|---------|---------|
| List | `list[T]` | Heap (dynamic) | `list[int] = [1, 2, 3]` |
| Dictionary | `dict[K, V]` | Heap (hash table) | `dict[str, int] = {"a": 1}` |
| Array | `T[N]` | Stack (fixed-size) | `int[5] = [1, 2, 3, 4, 5]` |
| Tuple | `(T1, T2, ...)` | Stack (fixed-size) | `(int, str) = (1, "hi")` |

## Lists

Dynamic, resizable arrays. Support three element types: `int`, `float`, `str`.

```wadescript
nums: list[int] = [1, 2, 3]
prices: list[float] = [1.5, 2.7]
names: list[str] = ["Alice", "Bob"]

# Operations
nums.push(4)              # Append
last: int = nums.pop()    # Remove last
val: int = nums[0]        # Index access
nums[1] = 99              # Index assignment
len: int = nums.length    # Length property

# Iteration and slicing
for n in nums { print(n) }
sub: list[int] = nums[1:3]
```

See `LISTS.md` for implementation details.

## Dictionaries

Hash table mapping string keys to integer values.

```wadescript
ages: dict[str, int] = {"Alice": 30, "Bob": 25}

# Operations
age: int = ages["Alice"]        # Access
ages["Charlie"] = 35            # Assignment
for key in ages { print(key) }  # Iterate keys
```

Currently supports `dict[str, int]` only.

## Arrays

Fixed-size, stack-allocated arrays.

```wadescript
nums: int[5] = [10, 20, 30, 40, 50]
vals: float[3] = [1.5, 2.5, 3.5]

# Operations
x: int = nums[0]     # Index access
nums[2] = 99         # Index assignment
```

Arrays are allocated on the stack via LLVM's native array types and accessed via GEP instructions. No bounds checking at runtime.

## Tuples

Fixed-size heterogeneous collections.

```wadescript
point: (int, int) = (10, 20)
data: (str, int, bool) = ("Alice", 30, True)

# Indexing (compile-time)
x: int = point.0
name: str = data.0

# Unpacking
a, b = point
```

See `TUPLES.md` for details.

## Design Decisions

- **Python-style syntax**: `list[int]`, `dict[str, int]` feel natural
- **Fixed vs Dynamic**: Clear distinction between `int[5]` (stack) and `list[int]` (heap)
- **Static typing**: All operations are type-checked at compile time
- **Methods over functions**: `list.push()` instead of `push(list, item)`
- **`.length` property**: More intuitive than `len(list)`

## Implementation Status

See `DATA_STRUCTURES_STATUS.md` for detailed implementation status.
