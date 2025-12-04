# WadeScript CLI Module

The `cli` module provides command-line argument parsing utilities for WadeScript programs.

## Basic Usage

```wadescript
import "cli"

def main() -> int {
    # Get all command-line arguments as a list
    args: list[str] = cli.get_args()

    # Print each argument
    for arg in args {
        print_str(arg)
    }

    return 0
}
```

## Functions

### `get_args() -> list[str]`

Returns all command-line arguments as a list of strings, including the program name at index 0.

```wadescript
args: list[str] = cli.get_args()
# args[0] = program name
# args[1] = first argument
# ...
```

### `argc() -> int`

Returns the number of command-line arguments (including the program name).

```wadescript
count: int = cli.argc()
```

### `argv(index: int) -> str`

Returns the argument at the specified index. Index 0 is the program name.

```wadescript
program_name: str = cli.argv(0)
first_arg: str = cli.argv(1)
```

### `parse_int(s: str) -> int`

Parses a string as an integer. Returns 0 if parsing fails.

```wadescript
value: int = cli.parse_int("42")  # Returns 42
invalid: int = cli.parse_int("abc")  # Returns 0
```

### `parse_bool(s: str) -> bool`

Parses a string as a boolean. Recognizes "true", "True", "1" as True, everything else as False.

```wadescript
flag: bool = cli.parse_bool("true")   # Returns True
flag2: bool = cli.parse_bool("false") # Returns False
```

### `starts_with(s: str, prefix: str) -> bool`

Checks if a string starts with the given prefix.

```wadescript
is_flag: bool = cli.starts_with("--verbose", "--")  # Returns True
is_short: bool = cli.starts_with("-v", "--")        # Returns False
```

### `str_eq(a: str, b: str) -> bool`

Compares two strings for equality.

```wadescript
same: bool = cli.str_eq("hello", "hello")  # Returns True
diff: bool = cli.str_eq("hello", "world")  # Returns False
```

## Example: Simple Argument Parser

```wadescript
import "cli"

def main() -> int {
    args: list[str] = cli.get_args()
    verbose: bool = False
    output_file: str = "output.txt"

    i: int = 1  # Skip program name
    while i < args.length {
        arg: str = args[i]

        if cli.str_eq(arg, "-v") or cli.str_eq(arg, "--verbose") {
            verbose = True
        } elif cli.str_eq(arg, "-o") or cli.starts_with(arg, "--output=") {
            if cli.starts_with(arg, "--output=") {
                # Handle --output=filename format
                output_file = cli.argv(i)  # Would need substring support
            } else {
                # Handle -o filename format
                i = i + 1
                if i < args.length {
                    output_file = args[i]
                }
            }
        }

        i = i + 1
    }

    if verbose {
        print_str("Verbose mode enabled")
    }
    print_str(f"Output file: {output_file}")

    return 0
}
```

## Decorator-Based CLI (Planned)

WadeScript supports decorators on class fields for declarative CLI argument specification:

```wadescript
import "cli"

class Args {
    @arg(help="Input file path")
    input_file: str

    @option(short="o", long="output", help="Output file")
    output: str

    @option(short="v", long="verbose")
    verbose: bool
}
```

### Supported Decorators

#### `@arg`

Marks a field as a positional argument. Only valid on `str` fields.

```wadescript
@arg(help="Description of the argument")
field_name: str
```

#### `@option`

Marks a field as a named option. Valid on `str`, `int`, and `bool` fields.

```wadescript
@option(short="o", long="output", help="Description")
output: str

@option(short="n", long="number")
count: int

@option(short="v", long="verbose")
verbose: bool  # Boolean options are flags (no value needed)
```

**Parameters:**
- `short` - Single character for short option (e.g., `-o`). Must be exactly one character.
- `long` - Long option name (e.g., `--output`)
- `help` - Description for help text

### Type Checking

The type checker validates decorator usage:
- `@arg` requires `str` type
- `@option` requires `str`, `int`, or `bool` type
- `short` must be a single character
- Unknown decorators produce an error

## Runtime Functions (Low-Level)

These functions are used internally by `std/cli.ws`. You typically won't need to call them directly:

| Function | Description |
|----------|-------------|
| `cli_get_argc()` | Get argument count |
| `cli_get_argv(i)` | Get argument at index (borrowed) |
| `cli_get_argv_copy(i)` | Get argument at index (owned copy) |
| `cli_parse_int(s)` | Parse string to int |
| `cli_parse_bool(s)` | Parse string to bool |
| `cli_starts_with(s, p)` | Check if string starts with prefix |
| `cli_str_eq(a, b)` | Compare strings for equality |
| `cli_after_prefix(s, p)` | Get substring after prefix |
