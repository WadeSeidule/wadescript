# WadeScript Data Structures

## Current Status

We've added foundational support for arrays, lists, and dictionaries to WadeScript! The compiler now understands the syntax and performs type checking, but full runtime implementation is still in progress.

## Syntax

### Type Declarations

```wadescript
# Fixed-size arrays
numbers: int[5]           # Array of 5 integers

# Dynamic lists
names: list[str]          # Dynamic list of strings
scores: list[int]         # Dynamic list of integers

# Dictionaries/hashmaps
ages: dict[str, int]      # Dict mapping strings to ints
config: dict[str, float]  # Dict mapping strings to floats
```

### Array/List Literals

```wadescript
# List literals
numbers: list[int] = [1, 2, 3, 4, 5]
names: list[str] = ["Alice", "Bob", "Charlie"]
```

### Dictionary Literals

```wadescript
# Dict literals
ages: dict[str, int] = {"Alice": 30, "Bob": 25}
```

### Indexing

```wadescript
# Access elements by index
first: int = numbers[0]
second: int = numbers[1]

# Dict access
age: int = ages["Alice"]
```

### Properties

```wadescript
# Get length of arrays/lists
len: int = numbers.length
```

### Method Calls

```wadescript
# List methods (type-checked)
numbers.push(6)           # Add element to end
last: int = numbers.pop() # Remove and return last element
val: int = numbers.get(2) # Get element at index
```

## What's Implemented

### ✅ Fully Working

1. **Type System** - Complete type support for:
   - Fixed arrays: `int[5]`, `float[10]`
   - Dynamic lists: `list[int]`, `list[str]`
   - Dictionaries: `dict[str, int]`, `dict[int, float]`

2. **Type Checking** - Full type safety:
   - Array/list element type consistency
   - Index type validation (must be int for arrays/lists)
   - Dict key/value type checking
   - Method signature validation

3. **Parser** - Complete syntax support:
   - Type declarations with brackets
   - List literals: `[1, 2, 3]`
   - Dict literals: `{"key": value}`
   - Index access: `arr[0]`
   - Method calls: `list.push(5)`
   - Property access: `list.length`

### ⚠️ Partially Implemented

**Code Generation** - Basic LLVM type mapping exists, but runtime operations need implementation:
- Fixed arrays map to LLVM array types
- Lists/dicts use opaque pointer types (placeholder)
- Actual memory allocation and operations not yet implemented

## What Still Needs Implementation

To make data structures fully functional at runtime, we need:

1. **Runtime Memory Management**
   - Dynamic memory allocation for lists/dicts
   - Malloc/free wrappers or custom allocator
   - Reference counting or garbage collection

2. **List Implementation**
   - Struct with capacity, length, and data pointer
   - Push/pop operations with reallocation
   - Index bounds checking

3. **Dictionary Implementation**
   - Hash table data structure
   - Hash function for different key types
   - Collision handling (chaining or open addressing)
   - Get/set/delete operations

4. **Array Operations**
   - Element access code generation
   - Bounds checking (optional)
   - Initialization from literals

## Example Code

Even though runtime isn't complete, you can write and type-check code like this:

```wadescript
def process_numbers() -> int {
    # Create a list
    numbers: list[int] = [1, 2, 3, 4, 5]

    # Access elements
    first: int = numbers[0]
    last: int = numbers[4]

    # Get length
    count: int = numbers.length

    # Method calls
    numbers.push(6)
    popped: int = numbers.pop()

    return first + last
}

def use_dict() -> int {
    # Create a dictionary
    ages: dict[str, int] = {
        "Alice": 30,
        "Bob": 25,
        "Charlie": 35
    }

    # Access values
    alice_age: int = ages["Alice"]

    return alice_age
}
```

The type checker will verify:
- All list elements are the same type
- Index operations use integers
- Dict keys/values are consistent types
- Methods are called with correct arguments

## Next Steps

To complete the implementation:

1. **Implement List Runtime** (~500 lines)
   - Create LLVM struct for list metadata
   - Implement push/pop/get in LLVM IR
   - Add bounds checking

2. **Implement Dict Runtime** (~800 lines)
   - Create hash table structure
   - Implement hashing functions
   - Add get/set/has operations

3. **Add Array Initialization** (~200 lines)
   - Compile array literals to LLVM arrays
   - Generate initialization code

4. **Memory Management** (~300 lines)
   - Add malloc/free extern declarations
   - Implement allocation helpers
   - Add cleanup on scope exit (optional)

## Design Decisions

- **Python-style syntax**: `list[int]` and `dict[str, int]` feel natural
- **Fixed vs Dynamic**: Clear distinction between `int[5]` (stack) and `list[int]` (heap)
- **Type safety**: All operations are statically type-checked
- **Methods over functions**: `list.push()` instead of `push(list, item)`
- **.length property**: More intuitive than `len(list)` function

This foundation makes it straightforward to add the runtime implementation!
