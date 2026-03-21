# WadeScript Test Conventions

## Test Types

### Regular Tests (`tests/test_<name>.ws`)
- Test functionality with `assert` statements
- Must have `def main() -> int` returning 0 on success
- Non-zero exit = failure

```wadescript
def main() -> int {
    # Test basic functionality
    x: int = 42
    assert x == 42

    # Test edge cases
    y: int = 0
    assert y == 0

    # Test boundary conditions
    z: int = -1
    assert z < 0

    return 0
}
```

### Error Tests (`tests/test_error_<name>.ws`)
- Test that the compiler produces correct error messages
- Paired with `tests/test_error_<name>.ws.expected` file
- The `.expected` file contains the exact error output to match
- The test runner compiles the file and compares stderr to expected

```
# test_error_example.ws.expected
Error: Type mismatch: expected int, got str
  --> test_error_example.ws:3:5
```

### REPL Tests (`tests/test_repl.sh`)
- Bash script testing REPL interactive behavior
- Uses heredoc input piped to `./ws repl`
- Checks stdout for expected output

## Naming Conventions

| Pattern | Example | Purpose |
|---------|---------|---------|
| `test_<feature>.ws` | `test_lists.ws` | Feature functionality |
| `test_<feature>_<aspect>.ws` | `test_list_methods.ws` | Specific aspect |
| `test_error_<feature>.ws` | `test_error_types.ws` | Error message testing |

## Existing Test Coverage

Key test files to reference for patterns:
- `test_assert.ws` — basic assertions
- `test_lists.ws` — list operations
- `test_dicts.ws` — dictionary operations
- `test_classes.ws` — class features
- `test_for_loops.ws` — iteration
- `test_exceptions.ws` — exception handling
- `test_slices.ws` — slice syntax
- `test_tuples.ws` — tuple operations
- `test_named_args.ws` — named arguments
- `test_fstrings.ws` — string interpolation

## Best Practices

1. **Test normal cases first**, then edge cases, then boundaries
2. **One assert per logical check** — makes failures easy to locate
3. **Use descriptive variable names** that indicate what's being tested
4. **Test both positive and negative cases** where applicable
5. **Keep tests self-contained** — don't depend on external files unless testing I/O
6. **Update `docs/TEST_SUITE_SUMMARY.md`** if it tracks test counts
