---
description: Scaffold a new WadeScript test file with correct conventions
user-invocable: true
---

Create a new WadeScript test file for: $ARGUMENTS

## Existing test patterns

!`ls tests/test_*.ws 2>/dev/null | head -15`

## Steps

1. Check existing tests in `tests/` to understand naming conventions and patterns
2. Determine the test type:
   - **Regular test**: `tests/test_<name>.ws` - tests functionality with `assert` statements
   - **Error test**: `tests/test_error_<name>.ws` + `tests/test_error_<name>.ws.expected` - tests error messages
3. Create the test file with:
   - A `def main() -> int` function
   - Comprehensive `assert` statements covering normal cases, edge cases, and boundary conditions
   - `return 0` at the end (exit 0 = pass)
4. Run the new test with `./ws run tests/test_<name>.ws` to verify it passes
5. Run `make test` to verify no regressions
6. Update `docs/TEST_SUITE_SUMMARY.md` if it tracks test counts
