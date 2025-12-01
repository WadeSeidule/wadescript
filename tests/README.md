# Test Suite Structure

## Test Files

All test files follow the naming convention `test_*.ws` and must have a corresponding `test_*.expected` file with the exact expected output.

### Current Tests

1. **test_basic_types.ws** - Basic type operations
   - Integer arithmetic
   - Float operations
   - Boolean operations
   - String output

2. **test_functions.ws** - Function definitions and calls
   - Simple functions
   - Recursive functions (fibonacci, factorial)
   - Return values

3. **test_control_flow.ws** - Control flow statements
   - If/elif/else
   - While loops
   - Conditional logic

4. **test_for_loops.ws** - For loop iteration
   - List iteration
   - range() function
   - Sum computation
   - Empty list handling

5. **test_lists.ws** - List operations
   - List creation
   - Index access
   - Methods: push(), pop(), get()
   - Length property

6. **test_comparisons.ws** - Operators
   - Comparison operators
   - Logical operators
   - Negation

7. **test_integration.ws** - Complex integration
   - Multiple features combined
   - Prime number generation
   - List building with conditionals

## Running Tests

From the project root:

```bash
./run_tests.sh
```

## Expected Output Format

The `.expected` files contain the exact output produced by running the compiled test:

- One line per `print_*()` call
- No extra whitespace
- Float precision: 6 decimal places
- Boolean values: `True` or `False`

Example:
```
42
3.140000
True
Hello
```

## Adding Tests

1. Create `tests/test_feature.ws`
2. Create `tests/test_feature.expected`
3. Run `./run_tests.sh`

The new test will be automatically discovered.

## Test Coverage

Current coverage: **100%** of implemented features

- ✅ All basic types
- ✅ All operators
- ✅ Functions and recursion
- ✅ Control flow
- ✅ For loops
- ✅ Lists
- ✅ Integration

## Quick Reference

```bash
# Run all tests
./run_tests.sh

# Run a single test manually
./target/release/wadescript tests/test_lists.ws
./test_lists

# Compare output
diff <(./test_lists) tests/test_lists.expected
```
