# Named Arguments and Default Parameters in WadeScript

WadeScript supports named arguments and default parameter values, providing flexible function call syntax similar to Python.

## Default Parameters

Functions can specify default values for parameters. Parameters with defaults must come after parameters without defaults.

### Syntax

```wadescript
def function_name(param: type = default_value) -> return_type {
    # body
}
```

### Examples

```wadescript
# Function with all default parameters
def greet(name: str = "World", excited: bool = False) -> str {
    if excited {
        return f"Hello, {name}!"
    } else {
        return f"Hello, {name}"
    }
}

# Function with some default parameters
def add(a: int, b: int = 10, c: int = 100) -> int {
    return a + b + c
}
```

### Calling Functions with Defaults

```wadescript
# Use all defaults
greet()              # "Hello, World"

# Override first parameter
greet("Alice")       # "Hello, Alice"

# Override with positional args
add(1)               # 1 + 10 + 100 = 111
add(1, 2)            # 1 + 2 + 100 = 103
add(1, 2, 3)         # 1 + 2 + 3 = 6
```

## Named Arguments

When calling a function, you can specify arguments by name. This allows you to:
- Skip parameters that have defaults
- Pass arguments in any order
- Make code more readable

### Syntax

```wadescript
function_name(param_name=value)
```

### Examples

```wadescript
# Use named argument
greet(name="Bob")                    # "Hello, Bob"

# Skip first param, provide second
greet(excited=True)                  # "Hello, World!"

# Both params as named args
greet(name="Charlie", excited=True)  # "Hello, Charlie!"

# Named args in any order
greet(excited=True, name="Dana")     # "Hello, Dana!"

# Mix positional and named
greet("Eve", excited=True)           # "Hello, Eve!"
```

### Rules

1. **Positional before named**: Positional arguments must come before named arguments
   ```wadescript
   # VALID
   add(1, c=5)        # a=1 positionally, c=5 by name

   # INVALID - named before positional
   add(a=1, 5)        # Error!
   ```

2. **No duplicate parameters**: Each parameter can only be specified once
   ```wadescript
   # INVALID - 'a' specified twice
   add(1, a=2)        # Error! 'a' specified both positionally and by name
   ```

3. **Required parameters**: Parameters without defaults must be provided
   ```wadescript
   # 'a' has no default, must be provided
   add()              # Error! Missing required argument 'a'
   add(a=5)           # OK - 'a' provided by name
   ```

## Type Checking

The type checker validates:
- Default values match parameter types
- All named arguments match declared parameter names
- No parameter is specified multiple times
- All required parameters (those without defaults) are provided
- Argument types match parameter types

### Error Examples

```wadescript
# Error: Default value type mismatch
def bad(x: int = "hello") { }  # Type error!

# Error: Unknown parameter name
add(1, d=5)                     # No parameter named 'd'

# Error: Missing required argument
add()                           # Missing required argument 'a'

# Error: Duplicate parameter
add(1, a=2)                     # 'a' specified multiple times
```

## Implementation Details

### Default Value Evaluation

Default values are evaluated at call time (not at function definition time). This means:

```wadescript
def append_to(item: int, items: list[int] = []) -> list[int] {
    items.push(item)
    return items
}

# Each call gets a fresh default list
x: list[int] = append_to(1)   # [1]
y: list[int] = append_to(2)   # [2], not [1, 2]
```

### Performance

Named arguments and defaults are resolved at compile time where possible. The runtime performance impact is minimal as the argument reordering happens during code generation.

## See Also

- [Functions](QUICKSTART.md) - Basic function syntax
- [Type System](DATA_STRUCTURES.md) - WadeScript type system
