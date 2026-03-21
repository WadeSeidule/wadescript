# WadeScript Lists

## Supported Types

Lists support three element types: `list[int]`, `list[float]`, `list[str]`.

```wadescript
nums: list[int] = [1, 2, 3, 4, 5]
prices: list[float] = [1.5, 2.7, 3.14]
names: list[str] = ["Alice", "Bob", "Charlie"]
empty: list[int] = []
```

## Operations

### Index Access and Assignment
```wadescript
x: int = nums[0]       # Get element
nums[2] = 99           # Set element
```

### Methods
```wadescript
nums.push(6)            # Append element
last: int = nums.pop()  # Remove and return last
val: int = nums.get(0)  # Get element (same as indexing)
```

### Properties
```wadescript
len: int = nums.length  # Number of elements
```

### Iteration
```wadescript
for num in nums {
    print(num)
}

for i in range(nums.length) {
    print(nums[i])
}
```

### Slicing
```wadescript
sub: list[int] = nums[1:4]     # [2, 3, 4]
first3: list[int] = nums[:3]   # [1, 2, 3]
rev: list[int] = nums[::-1]    # Reversed
every2: list[int] = nums[::2]  # Every other element
```

### String Conversion
```wadescript
s: str = str(nums)      # "[1, 2, 3, 4, 5]"
print(nums)             # Prints: [1, 2, 3, 4, 5]
```

## Memory Layout

```
List Structure (24 bytes):
+------------------+----------+------------+
| data ptr (8)     | len (8)  | cap (8)    |
+------------------+----------+------------+
         |
         +-> [elem0][elem1][elem2]...
            (8 bytes each)
```

- Elements are 8 bytes each (i64 for ints, f64 for floats, pointer for strings)
- Capacity doubles when full (starting at 4)
- Managed by automatic reference counting

## Runtime Functions

Each element type has its own set of runtime functions:

| Operation | int | float | str |
|-----------|-----|-------|-----|
| get | `list_get_i64` | `list_get_f64` | `list_get_str` |
| push | `list_push_i64` | `list_push_f64` | `list_push_str` |
| pop | `list_pop_i64` | `list_pop_f64` | `list_pop_str` |
| set | `list_set_i64` | `list_set_f64` | `list_set_str` |
| slice | `list_slice_i64` | `list_slice_f64` | `list_slice_str` |
| to_string | `list_to_string` | `list_to_string_f64` | `list_to_string_str` |

The codegen dispatches to the correct function based on the element type at compile time.

## Performance

| Operation | Time | Space |
|-----------|------|-------|
| Create empty | O(1) | 24 bytes |
| Push | Amortized O(1) | 8 bytes/elem |
| Pop | O(1) | - |
| Get/Set by index | O(1) | - |
| Length | O(1) | - |
| Slice | O(n) | New list allocated |

All operations include bounds checking with descriptive error messages.
