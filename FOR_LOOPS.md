# For Loops in WadeScript

WadeScript implements Python-style for loops that can iterate over lists and other collections. The implementation uses a desugaring approach, transforming for loops into while loops at compile time.

## Syntax

```wadescript
for variable in iterable {
    # loop body
}
```

## Supported Iterables

### Lists
```wadescript
numbers: list[int] = [1, 2, 3, 4, 5]
for num in numbers {
    print_int(num)  # Prints each number
}
```

### The range() Function
```wadescript
# Iterate from 0 to n-1
for i in range(10) {
    print_int(i)  # Prints 0, 1, 2, ..., 9
}
```

## Implementation Details

### Desugaring Strategy

For loops are transformed into while loops during code generation:

**WadeScript code:**
```wadescript
for item in list {
    print_int(item)
}
```

**Desugared to (conceptually):**
```wadescript
_idx: int = 0
while _idx < list.length {
    item: int = list[_idx]
    print_int(item)
    _idx = _idx + 1
}
```

### Type Checking

The type checker infers the loop variable type from the iterable:
- `list[int]` → element type is `int`
- `list[str]` → element type is `str`
- Arrays and dictionaries work similarly

### Code Generation

The codegen phase:
1. Evaluates the iterable once and stores it
2. Gets the length using `list.length`
3. Creates an index variable initialized to 0
4. Generates three basic blocks:
   - **Condition block**: Checks `idx < length`
   - **Body block**: Loads element, executes body, increments index
   - **Exit block**: Continues after loop

### The range() Function

`range(n)` creates a list containing integers from 0 to n-1.

**Implementation:**
```wadescript
range(5)  # Returns list[int] containing [0, 1, 2, 3, 4]
```

The function:
1. Creates an empty list
2. Loops from 0 to n-1
3. Pushes each integer to the list
4. Returns the populated list

**Type signature:**
```wadescript
range(n: int) -> list[int]
```

## Examples

### Computing Sum
```wadescript
numbers: list[int] = [1, 2, 3, 4, 5]
sum: int = 0
for num in numbers {
    sum = sum + num
}
print_int(sum)  # Prints: 15
```

### Finding Maximum
```wadescript
scores: list[int] = [45, 92, 67, 88, 73]
max: int = 0
for score in scores {
    if score > max {
        max = score
    }
}
print_int(max)  # Prints: 92
```

### Counting Elements
```wadescript
values: list[int] = [10, 25, 30, 5, 50]
count: int = 0
for v in values {
    if v > 20 {
        count = count + 1
    }
}
print_int(count)  # Prints: 3
```

### Using range() for Multiplication
```wadescript
for i in range(10) {
    result: int = i * 7
    print_int(result)  # Prints multiples of 7
}
```

### Empty Collections
```wadescript
empty: list[int] = []
for item in empty {
    print_int(item)  # Never executes
}

for i in range(0) {
    print_int(i)  # Never executes
}
```

## Performance

### Time Complexity
- **For loop over list**: O(n) where n is the list length
- **range(n)**: O(n) to build the list + O(n) to iterate = O(n)

### Space Complexity
- **For loop**: O(1) extra space (just the index and loop variable)
- **range(n)**: O(n) space to store the list

### Optimization Notes

The desugaring approach:
- ✅ Simple and predictable
- ✅ No need for iterator protocol
- ✅ Efficient index-based access
- ✅ Works with existing list runtime

Future optimizations could include:
- Iterator protocol for zero-copy iteration
- Optimizing away the range() list allocation for simple loops
- LLVM loop optimization passes

## Compiler Implementation

### Files Modified

1. **src/typechecker.rs** (lines 214-243)
   - Infers element type from iterable
   - Declares loop variable in new scope
   - Type checks loop body

2. **src/codegen.rs** (lines 490-592)
   - Desugars for loop to while loop
   - Generates LLVM IR for iteration
   - Manages loop variable scope

3. **src/codegen.rs** (lines 986-1051)
   - Implements range() as special built-in
   - Creates list and populates with loop
   - Returns populated list

### Integration with Lists

For loops depend on:
- `list.length` property (src/codegen.rs:252-271)
- `list_get_i64(list, index)` runtime function (runtime/list.c)
- `list_push_i64(list, value)` for range() (runtime/list.c)

## Testing

Run the examples to see for loops in action:

```bash
# Basic for loop demo
./target/release/wadescript examples/for_loops_demo.ws
./for_loops_demo

# Range function demo
./target/release/wadescript examples/range_demo.ws
./range_demo
```

## Future Enhancements

Potential additions:
- **range(start, end)**: Two-argument version
- **range(start, end, step)**: Three-argument version with step
- **for-in over arrays**: Fixed-size array iteration
- **for-in over dicts**: Iterate over keys
- **break and continue**: Loop control statements
- **enumerate()**: Get index and value
- **zip()**: Iterate over multiple lists
- **Iterator protocol**: For custom iteration logic

## Comparison with Python

### Similarities
```python
# Python
for i in range(10):
    print(i)

for item in [1, 2, 3]:
    print(item)
```

```wadescript
# WadeScript
for i in range(10) {
    print_int(i)
}

for item in [1, 2, 3] {
    print_int(item)
}
```

### Differences
- WadeScript requires type annotations on variables
- WadeScript uses braces `{}` instead of colons and indentation
- WadeScript needs explicit print functions per type

## Summary

For loops in WadeScript provide:
- ✅ Python-style iteration syntax
- ✅ Type-safe element access
- ✅ Efficient compiled code
- ✅ Full integration with lists
- ✅ range() function for numeric loops
- ✅ Predictable performance characteristics

The implementation is complete, tested, and ready for use!
