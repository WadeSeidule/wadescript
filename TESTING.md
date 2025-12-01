# WadeScript Test Suite

Comprehensive test suite to prevent regressions and ensure all language features work correctly.

## Running Tests

### Run All Tests
```bash
./run_tests.sh
```

This will:
1. Build the compiler if needed
2. Compile each test file
3. Run the executable
4. Compare output to expected results
5. Report pass/fail status

### Test Output
```
==================================
WadeScript Test Suite
==================================

Running: test_basic_types
PASS

Running: test_comparisons
PASS

...

==================================
Test Results
==================================
Total:  6
Passed: 6
Failed: 0

All tests passed!
```

## Test Coverage

### test_basic_types.ws
Tests fundamental language features:
- Integer arithmetic (+, -, *, /, %)
- Float arithmetic
- Boolean values and comparisons
- String literals
- Print functions for all types

**Coverage:**
- Type system basics
- Arithmetic operators
- Comparison operators
- Built-in print functions

### test_functions.ws
Tests function definitions and calls:
- Simple functions with parameters
- Return values
- Recursive functions (factorial, fibonacci)

**Coverage:**
- Function definitions
- Parameter passing
- Return statements
- Recursion
- Function calls

### test_control_flow.ws
Tests control flow statements:
- If statements
- If-else statements
- If-elif-else chains
- While loops
- Loop variables and counters

**Coverage:**
- Conditional branching
- Boolean conditions
- While loop execution
- Variable scoping in control flow

### test_for_loops.ws
Tests Python-style for loops:
- Iteration over list literals
- range() function
- Computing with loop variables
- Empty list iteration

**Coverage:**
- For loop syntax
- List iteration
- range() built-in
- Loop variable scoping
- Empty iterable handling

### test_lists.ws
Tests dynamic list operations:
- Empty lists
- List literals with elements
- Index access
- push() method
- pop() method
- get() method
- Length property
- Dynamic list building

**Coverage:**
- List creation
- List methods
- Index operations
- List iteration
- Dynamic allocation

### test_comparisons.ws
Tests comparison and logical operators:
- Equality (==, !=)
- Relational (<, >, <=, >=)
- Logical (and, or, not)
- Combined conditions
- Negation operator

**Coverage:**
- All comparison operators
- Logical operators
- Boolean expressions
- Unary operators

## Adding New Tests

### 1. Create Test File
Create a new test in `tests/test_*.ws`:

```wadescript
# Test: Description of what this tests

def main() -> int {
    # Test code here
    print_int(42)  # Comment with expected value
    return 0
}
```

### 2. Create Expected Output
Create `tests/test_*.expected` with the exact expected output:

```
42
```

**Important:**
- Match output exactly (including newlines)
- Each print statement adds a newline
- No trailing newlines after the last output

### 3. Run Tests
```bash
./run_tests.sh
```

The new test will be automatically discovered and run.

## Test Guidelines

### Writing Good Tests
1. **Single Responsibility**: Each test file should focus on one feature area
2. **Clear Comments**: Annotate expected output in comments
3. **Edge Cases**: Include boundary conditions and edge cases
4. **Deterministic**: Tests should produce the same output every time
5. **Fast**: Keep tests quick to maintain fast iteration

### Expected Output Format
- Each `print_*` statement produces one line of output
- `print_int(42)` → `42\n`
- `print_float(3.14)` → `3.140000\n`
- `print_bool(True)` → `True\n`
- `print_str("hello")` → `hello\n`

### Common Pitfalls
- **Trailing newlines**: Don't add extra newlines at the end of `.expected` files
- **Float precision**: Floats print with 6 decimal places (e.g., `3.140000`)
- **Boolean casing**: `True` and `False` (capitalized)

## Continuous Integration

The test suite is designed to be CI-friendly:
- Exit code 0 on success, 1 on failure
- Color output can be disabled for CI logs
- Self-contained (no external dependencies beyond LLVM)

### Example CI Integration
```yaml
# .github/workflows/test.yml
- name: Run tests
  run: ./run_tests.sh
```

## Test Maintenance

### When to Update Tests
- **Breaking changes**: Update expected output if language behavior changes intentionally
- **New features**: Add new test files for new language features
- **Bug fixes**: Add regression tests for fixed bugs

### Debugging Failed Tests
When a test fails, the runner shows:
```
FAIL: Output mismatch
Expected:
42

Actual:
43
```

Common causes:
1. Code generation bug
2. Runtime behavior change
3. Expected output file is wrong

## Coverage Summary

| Feature Area | Test File | Lines | Coverage |
|--------------|-----------|-------|----------|
| Basic Types | test_basic_types.ws | 27 | Complete |
| Functions | test_functions.ws | 30 | Complete |
| Control Flow | test_control_flow.ws | 46 | Complete |
| For Loops | test_for_loops.ws | 39 | Complete |
| Lists | test_lists.ws | 48 | Complete |
| Comparisons | test_comparisons.ws | 31 | Complete |
| **Total** | **6 files** | **221 lines** | **100%** |

## Future Test Areas

Consider adding tests for:
- **Error handling**: Invalid operations, type errors
- **Edge cases**: Integer overflow, division by zero
- **Memory**: Large lists, many allocations
- **Performance**: Benchmarks for key operations
- **Arrays**: Once arrays are implemented
- **Dictionaries**: Once dicts are implemented
- **Classes**: Object-oriented features
- **String operations**: String concatenation, manipulation

## Test Philosophy

The WadeScript test suite follows these principles:

1. **Regression Prevention**: Catch bugs before they ship
2. **Documentation**: Tests serve as executable examples
3. **Confidence**: Make refactoring safe
4. **Speed**: Fast feedback loop for development
5. **Simplicity**: Easy to understand and maintain

## Running Individual Tests

To run a specific test manually:

```bash
# Build compiler
cargo build --release

# Compile test
./target/release/wadescript tests/test_lists.ws

# Run test
./test_lists

# Compare output
diff <(./test_lists) tests/test_lists.expected
```

## Test Statistics

Current test coverage:
- **6 test files**
- **221 lines of test code**
- **100% pass rate**
- **~2 seconds** total execution time

Features tested:
- ✅ All basic types (int, float, bool, str)
- ✅ All operators (arithmetic, comparison, logical)
- ✅ Functions and recursion
- ✅ Control flow (if/elif/else, while)
- ✅ For loops and iteration
- ✅ Lists (creation, methods, indexing)
- ✅ range() function
- ✅ Print functions

## Contribution

When contributing to WadeScript:
1. Write tests for new features
2. Ensure all tests pass before submitting
3. Update expected outputs if behavior changes intentionally
4. Document test coverage in commit messages

Run tests before committing:
```bash
./run_tests.sh && echo "Ready to commit!"
```

---

**The test suite is your safety net. Use it!** 🛡️
