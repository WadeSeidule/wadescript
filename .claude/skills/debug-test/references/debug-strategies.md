# Test Debugging Strategies

## Running Tests

```bash
# Single WadeScript test
./ws run tests/test_<name>.ws

# Single Rust test
cargo test <test_name>

# All tests
make test test-rust

# Error tests (compare against .expected files)
make test-errors
```

## Error Classification

### Compile Error
The compiler rejects the test program.

**Diagnosis:**
1. Read the error message — it shows file, line, and expected vs actual
2. Trace through the pipeline in order:
   - `src/lexer.rs` — Is the token recognized?
   - `src/parser.rs` — Is the syntax parsed correctly?
   - `src/typechecker.rs` — Are the types valid?
3. Add `--emit-llvm` to see if codegen is reached

### Runtime Error / Segfault
The program compiles but crashes at runtime.

**Diagnosis:**
1. Run with `./ws run <file> --emit-llvm` to inspect generated LLVM IR
2. Common causes in `src/codegen.rs`:
   - Wrong pointer type (i8* vs opaque pointer)
   - Missing null check before dereference
   - Wrong calling convention for runtime function
   - Incorrect struct field offsets
3. Check RC balance — use-after-free if released too early
4. Check stack trace output for the crash location

### Assertion Failure
The program runs but an `assert` fails.

**Diagnosis:**
1. Identify which assert failed (line number in output)
2. Add `print()` calls before the assert to inspect values
3. Trace the computation:
   - Is the expression evaluated correctly in codegen?
   - Is the runtime function returning the right value?
   - Is there a type promotion issue (int vs float)?

### Wrong Output
The program produces incorrect output (for error tests with `.expected`).

**Diagnosis:**
1. Compare actual vs expected output
2. Check if error messages changed format
3. Verify `.expected` file is up to date with current error format

## Pipeline Tracing

For a construct that doesn't work, trace through each compiler phase:

```
Source code
  ↓ lexer.rs: Is each token correct?
Tokens
  ↓ parser.rs: Is the AST node correct?
AST
  ↓ typechecker.rs: Are types resolved correctly?
Typed AST
  ↓ codegen.rs: Is the LLVM IR correct?
LLVM IR
  ↓ runtime: Does the runtime function behave correctly?
Output
```

At each stage, verify the output matches expectations before moving to the next.

## LLVM IR Inspection

```bash
./ws run file.ws --emit-llvm
```

Key things to look for in the IR:
- Function signatures match runtime declarations
- Correct types on `call` instructions
- `getelementptr` indices match struct layouts
- Branch targets are correct for control flow
- RC calls (`ws_rc_retain`, `ws_rc_release`) are balanced
