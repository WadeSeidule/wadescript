# Import System in WadeScript

WadeScript has a simple and effective import system that allows code reuse across multiple files.

## Syntax

```wadescript
import "path/to/file"
```

The path is relative to the importing file's location. The `.ws` extension is optional and will be added automatically if not present.

```wadescript
import "math_lib"        # Loads math_lib.ws
import "lib/utils"       # Loads lib/utils.ws
import "file.ws"         # Also works with explicit extension
```

## Basic Usage

### Creating a Library

`math_lib.ws`:
```wadescript
def add(a: int, b: int) -> int {
    return a + b
}

def multiply(a: int, b: int) -> int {
    return a * b
}
```

### Using a Library

`main.ws`:
```wadescript
import "math_lib"

def main() -> int {
    result: int = add(5, 10)
    print_int(result)  # Prints: 15
    return 0
}
```

## Features

### ✅ Relative Paths
Imports are resolved relative to the importing file:
```wadescript
import "lib/utils"       # Same directory/lib/utils.ws
import "helpers"         # Same directory/helpers.ws
```

### ✅ Multiple Imports
Import as many files as needed:
```wadescript
import "math"
import "list_utils"
import "helpers"
```

### ✅ Nested Imports
Imported files can also import other files:

`main.ws`:
```wadescript
import "high_level"
```

`high_level.ws`:
```wadescript
import "low_level"

def high_function() -> int {
    return low_function()
}
```

### ✅ Circular Import Detection
The compiler detects and prevents circular imports:

`a.ws`:
```wadescript
import "b"
```

`b.ws`:
```wadescript
import "a"  # Error: Circular import detected
```

## How It Works

### Compilation Process

1. **Parse main file**: Read and parse the entry point file
2. **Find imports**: Identify all `import` statements
3. **Recursive loading**: Recursively load and parse imported files
4. **Merge**: Combine all statements from all files
5. **Compile**: Type check and compile as a single program

### Namespace

- All functions from imported files are available in the global namespace
- No need for qualified names (e.g., `module.function`)
- All imported functions are directly accessible

### Import Order

- Imports are processed depth-first
- Functions from imported files are available immediately
- No forward declaration needed

## Examples

### Example 1: Math Library

`lib/math.ws`:
```wadescript
def square(n: int) -> int {
    return n * n
}

def cube(n: int) -> int {
    return n * n * n
}
```

`main.ws`:
```wadescript
import "lib/math"

def main() -> int {
    print_int(square(5))  # 25
    print_int(cube(3))    # 27
    return 0
}
```

### Example 2: List Utilities

`lib/list_utils.ws`:
```wadescript
def sum_list(numbers: list[int]) -> int {
    total: int = 0
    for num in numbers {
        total = total + num
    }
    return total
}

def max_in_list(numbers: list[int]) -> int {
    max: int = 0
    for num in numbers {
        if num > max {
            max = num
        }
    }
    return max
}
```

`main.ws`:
```wadescript
import "lib/list_utils"

def main() -> int {
    numbers: list[int] = [1, 2, 3, 4, 5]
    print_int(sum_list(numbers))    # 15
    print_int(max_in_list(numbers)) # 5
    return 0
}
```

### Example 3: Multiple Imports

`utils.ws`:
```wadescript
def is_even(n: int) -> bool {
    return n % 2 == 0
}
```

`math.ws`:
```wadescript
def add(a: int, b: int) -> int {
    return a + b
}
```

`main.ws`:
```wadescript
import "utils"
import "math"

def main() -> int {
    x: int = add(5, 10)
    print_int(x)           # 15
    print_bool(is_even(x)) # False
    return 0
}
```

## Implementation Details

### File Resolution

Paths are resolved as follows:
1. Convert to absolute path using `fs::canonicalize()`
2. Resolve relative to the importing file's directory
3. Check for circular dependencies
4. Load and parse the file

### AST Structure

Import statement in AST:
```rust
Statement::Import {
    path: String,  // The file path as a string
}
```

### Processing Steps

1. **Lexer**: Recognizes `import` keyword
2. **Parser**: Parses `import "path"` syntax
3. **Main**: Recursively loads all imported files
4. **Merger**: Combines all statements, removing `import` statements
5. **Type Checker**: Validates combined program
6. **Code Generator**: Generates code for all functions

### Circular Detection

Uses a `HashSet<PathBuf>` to track visited files:
- Add file to set when entering
- Check if file already in set (circular import)
- Prevents infinite recursion

## Best Practices

### 1. Organize Code into Modules
```
project/
  main.ws
  lib/
    math.ws
    list_utils.ws
    string_utils.ws
```

### 2. Keep Libraries Focused
Each file should have a specific purpose:
- `math.ws` - Math functions
- `list_utils.ws` - List operations
- `io.ws` - Input/output helpers

### 3. Avoid Name Collisions
Since all functions share the global namespace, use descriptive names:
```wadescript
# Good
def list_sum(lst: list[int]) -> int { ... }
def math_sqrt(n: float) -> float { ... }

# Avoid generic names
def sum(...) { ... }  # Ambiguous
def calc(...) { ... } # Ambiguous
```

### 4. Document Dependencies
Add comments at the top of files:
```wadescript
# Requires: math.ws, list_utils.ws
import "math.ws"
import "list_utils.ws"
```

## Limitations

### Current Limitations

1. **No selective imports**: Can't import specific functions
   - `import "lib.ws"` imports everything
   - No `from lib import func` syntax

2. **Global namespace**: All imports share namespace
   - No module prefixes (e.g., `math.add()`)
   - Potential for name collisions

3. **No import aliases**: Can't rename imports
   - No `import "lib.ws" as mylib`

4. **Compile-time only**: Imports are static
   - No dynamic/conditional imports

### Future Enhancements

Potential future features:
- Selective imports: `import { add, multiply } from "math.ws"`
- Module namespaces: `math::add(5, 10)`
- Import aliases: `import "long_name.ws" as short`
- Standard library: Built-in modules
- Package manager: External dependencies

## Error Messages

### File Not Found
```
Error loading program: Error reading file 'missing.ws': No such file or directory
```

### Circular Import
```
Error loading program: Circular import detected: /path/to/file.ws
```

### Syntax Error in Import
```
Expected string literal after 'import'
```

## Testing

The import system is fully tested in the test suite:

`tests/test_imports.ws`:
```wadescript
import "test_helpers"

def main() -> int {
    print_int(double(5))  # From test_helpers.ws
    return 0
}
```

Run tests:
```bash
./run_tests.sh
```

## Summary

The WadeScript import system provides:
- ✅ Simple syntax: `import "file"` (`.ws` extension optional)
- ✅ Relative paths
- ✅ Multiple imports
- ✅ Nested imports
- ✅ Circular import detection
- ✅ Compile-time resolution
- ✅ Full test coverage

Perfect for organizing code across multiple files and building reusable libraries!
