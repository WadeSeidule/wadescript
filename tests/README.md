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

6. **test_dictionaries.ws** - Dictionary operations
   - Dictionary creation with initial values
   - Empty dictionary creation
   - Key-value insertion
   - Dictionary access by key
   - Value updates
   - Hash table rehashing (15+ entries)

7. **test_comparisons.ws** - Operators
   - Comparison operators
   - Logical operators
   - Negation

8. **test_integration.ws** - Complex integration
   - Multiple features combined
   - Prime number generation
   - List building with conditionals

## Running Tests

From the project root:

```bash
./ws test
```

The test runner will:
- Find all `test_*.ws` files in `tests/` directory
- Compile and run each test
- Compare output with corresponding `.expected` file
- Report pass/fail status for each test
- Display a summary with total passed/failed counts

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

1. Create `tests/test_feature.ws` with your test code
2. Run the test and capture output: `./ws run tests/test_feature.ws > tests/test_feature.expected`
3. Manually verify the output is correct
4. Run `./ws test` to verify the new test passes

The new test will be automatically discovered and included in the test suite.

## Test Coverage

Current coverage: **100%** of implemented features

- ✅ All basic types (int, float, bool, str)
- ✅ All operators (arithmetic, comparison, logical)
- ✅ Functions and recursion
- ✅ Control flow (if/elif/else, while)
- ✅ For loops and range()
- ✅ Lists (creation, methods, indexing)
- ✅ Dictionaries (hash table, CRUD operations)
- ✅ Module imports
- ✅ Integration tests

## Quick Reference

```bash
# Run all tests
./ws test

# Run a single test manually
./ws run tests/test_lists.ws

# Add a new test
./ws run tests/test_feature.ws > tests/test_feature.expected

# Verify all tests pass
./ws test
```
