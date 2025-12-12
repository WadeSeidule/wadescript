# Slice Syntax in WadeScript

WadeScript supports Python-style slice syntax for lists and strings, allowing you to extract subsequences with optional start, end, and step parameters.

## Basic Syntax

```
object[start:end:step]
```

- **start**: Starting index (inclusive), defaults to 0
- **end**: Ending index (exclusive), defaults to length
- **step**: Step size, defaults to 1

All parameters are optional.

## List Slicing

### Basic Slices

```wadescript
nums: list[int] = [0, 1, 2, 3, 4, 5]

# Get elements from index 1 to 4 (exclusive)
sub: list[int] = nums[1:4]    # [1, 2, 3]

# Get first 3 elements
first3: list[int] = nums[:3]  # [0, 1, 2]

# Get elements from index 3 to end
last3: list[int] = nums[3:]   # [3, 4, 5]

# Copy entire list
copy: list[int] = nums[:]     # [0, 1, 2, 3, 4, 5]
```

### Slices with Step

```wadescript
nums: list[int] = [0, 1, 2, 3, 4, 5]

# Every second element
every2: list[int] = nums[::2]    # [0, 2, 4]

# Every third element
every3: list[int] = nums[::3]    # [0, 3]

# Every second element starting from index 1
odd_indices: list[int] = nums[1::2]  # [1, 3, 5]

# Slice with step
sub_step: list[int] = nums[0:6:2]    # [0, 2, 4]
```

### Reverse with Negative Step

```wadescript
nums: list[int] = [0, 1, 2, 3, 4, 5]

# Reverse the list
reversed: list[int] = nums[::-1]     # [5, 4, 3, 2, 1, 0]

# Reverse part of the list
rev_part: list[int] = nums[4:1:-1]   # [4, 3, 2]
```

## String Slicing

String slicing works the same way as list slicing:

```wadescript
s: str = "hello world"

# Get substring
hello: str = s[:5]       # "hello"
world: str = s[6:]       # "world"

# Get middle substring
middle: str = s[3:8]     # "lo wo"

# Copy string
copy: str = s[:]         # "hello world"

# Reverse string
rev: str = s[::-1]       # "dlrow olleh"
```

## Slice Patterns

| Pattern | Description |
|---------|-------------|
| `[:]` | Copy entire sequence |
| `[start:]` | From start to end |
| `[:end]` | From beginning to end |
| `[start:end]` | From start to end |
| `[::step]` | Every Nth element |
| `[start::step]` | From start, every Nth |
| `[:end:step]` | To end, every Nth |
| `[start:end:step]` | Full slice |
| `[::-1]` | Reverse |

## Type Safety

The type checker ensures:
- Only lists and strings can be sliced
- Start, end, step must be integers
- Result has the same type as the input

```wadescript
nums: list[int] = [1, 2, 3]
sub: list[int] = nums[1:3]    # OK - returns list[int]

s: str = "hello"
sub_str: str = s[1:4]         # OK - returns str

# Error: cannot slice int
x: int = 42
# x[1:3]  # Type error!
```

## Implementation Notes

- Slices create new sequences (don't modify original)
- Negative indices are supported (count from end)
- Out-of-bounds indices are handled gracefully (clamped to valid range)
- Empty slices return empty sequences of the same type

## Examples

### Extract Sublist

```wadescript
def get_middle(items: list[int]) -> list[int] {
    len: int = items.length
    start: int = len / 4
    end: int = len * 3 / 4
    return items[start:end]
}

def main() -> int {
    nums: list[int] = [1, 2, 3, 4, 5, 6, 7, 8]
    middle: list[int] = get_middle(nums)
    # middle is [3, 4, 5, 6]
    return 0
}
```

### Split String

```wadescript
def split_at(s: str, pos: int) -> (str, str) {
    first: str = s[:pos]
    second: str = s[pos:]
    return (first, second)
}

def main() -> int {
    text: str = "hello world"
    first, second = split_at(text, 5)
    # first = "hello", second = " world"
    return 0
}
```

### Pagination

```wadescript
def get_page(items: list[int], page: int, page_size: int) -> list[int] {
    start: int = page * page_size
    end: int = start + page_size
    return items[start:end]
}
```

## See Also

- [Data Structures](DATA_STRUCTURES.md) - Lists, dicts, arrays
- [Tuples](TUPLES.md) - Tuple operations
