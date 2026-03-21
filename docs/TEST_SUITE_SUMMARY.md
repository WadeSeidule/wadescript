# Test Suite Summary

## Overview

WadeScript has a comprehensive test suite with 50 test files covering all language features.

## Test Infrastructure

- **Test Runner**: `./ws test` or `make test` — compiles and runs all `tests/test_*.ws` files
- **Error Tests**: `make test-errors` — tests with `.expected` files for error message validation
- **Rust Tests**: `make test-rust` — unit tests for runtime functions
- **REPL Tests**: `tests/test_repl.sh` — interactive REPL behavior

## Test Coverage

| Category | Test Files | Features Covered |
|----------|-----------|------------------|
| Core Types | `test_basic_types.ws` | int, float, bool, str, arithmetic |
| Functions | `test_functions.ws` | definitions, calls, recursion |
| Control Flow | `test_control_flow.ws`, `test_break_continue.ws` | if/elif/else, while, break/continue |
| For Loops | `test_for_loops.ws` | list iteration, range(), string iteration |
| Lists | `test_lists.ws`, `test_list_float.ws`, `test_list_str.ws` | list[int], list[float], list[str], methods, slicing |
| Arrays | `test_arrays.ws` | fixed-size arrays, indexing, assignment |
| Dictionaries | `test_dictionaries.ws` | creation, access, iteration |
| Tuples | `test_tuples.ws` | literals, indexing, unpacking |
| Slices | `test_slices.ws` | list and string slicing |
| Strings | `test_string_features.ws`, `test_str_print.ws` | methods, str(), print() |
| Classes | `test_class_str.ws` | fields, methods, str() conversion |
| Exceptions | `test_exc_simple.ws`, `test_exc_multiple.ws`, `test_exc_finally.ws`, `test_exc_try_except_finally.ws`, `test_exceptions_basic.ws` | try/except/finally, raise, multiple handlers |
| Named Args | `test_named_args.ws` | default parameters, named arguments |
| Operators | `test_comparisons.ws`, `test_compound_assign.ws`, `test_incr_decr.ws`, `test_floor_division.ws` | comparisons, +=/-=, ++/--, // |
| Imports | `test_imports.ws` | module system |
| Standard Library | `test_cli_basic.ws`, `test_http.ws`, `test_io.ws` | cli, http, io modules |
| RC Optimization | `test_rc_basic.ws`, `test_rc_last_use.ws`, `test_rc_escape_analysis.ws`, `test_rc_move_optimization.ws`, `test_rc_loop_hoisting.ws`, `test_rc_phase4_pure.ws`, `test_rc_leak.ws` | All 4 RC optimization phases |
| Error Messages | `test_error_dict_key.ws`, `test_error_line_numbers.ws`, `test_error_list_set.ws`, `test_error_stack_trace.ws` | Error output validation |
| F-Strings | `test_fstrings.ws` | Variable, expression, and type interpolation |
| Dict Iteration | `test_dict_iteration.ws` | for-in over dict, .length property |
| String Methods | `test_string_methods.ws` | .upper(), .lower(), .contains(), concatenation, indexing |
| Negative Slicing | `test_negative_slicing.ws` | Reverse slicing, step slicing, string slicing |
| Range Edge Cases | `test_range_edge_cases.ws` | range(0), range(1), nested ranges |
| Classes | `test_classes.ws` | Fields, methods, method chaining, str()/print() |
| Other | `test_assert.ws`, `test_optional.ws`, `test_decorator_parse.ws`, `test_integration.ws` | assert, optional types, decorators |
| **Total** | **50 files** | **All implemented features** |

## Running Tests

```bash
make test            # Run all WadeScript tests
make test-rust       # Run all Rust unit tests
make test-errors     # Run error message tests
./ws test            # Alternative test runner
./ws run tests/test_lists.ws  # Run individual test
```

## Test Structure

- **Regular tests**: `tests/test_*.ws` — use `assert` statements, exit 0 on pass
- **Error tests**: `tests/test_error_*.ws` + `.expected` files — validate error messages
- **REPL tests**: `tests/test_repl.sh` — bash script testing REPL behavior
- **Rust unit tests**: Inline in `src/runtime/*.rs` files
