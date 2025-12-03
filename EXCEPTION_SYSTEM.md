# WadeScript Exception Handling System

## Overview

WadeScript now has a complete Python-style exception handling system implemented using setjmp/longjmp for efficient exception unwinding.

## Features

### ✅ Implemented

1. **Raise Statement**
   - Syntax: `raise ExceptionType("message")`
   - Creates exception with type, message, file, and line number
   - Unwinds stack to nearest exception handler

2. **Try/Except Blocks**
   - Syntax: `try { ... } except ExceptionType { ... }`
   - Multiple except clauses supported
   - Exception type matching
   - Catch-all except clause (no type specified)

3. **Exception Variable Binding**
   - Syntax: `except ExceptionType as e { ... }`
   - Binds exception object to variable

4. **Finally Blocks**
   - Syntax: `try { ... } except { ... } finally { ... }`
   - Always executes, even if exception occurs
   - Useful for cleanup code

5. **Built-in Exception Types**
   - ValueError
   - KeyError
   - IndexError (automatically raised by runtime)
   - RuntimeError
   - TypeError

## Implementation Details

### Architecture

**Runtime (Rust):**
- `src/runtime/exceptions.rs` - Exception handling runtime
- Exception structure: `{ exception_type, message, file, line }`
- Global stack of jump buffers for nested try blocks
- setjmp/longjmp for stack unwinding

**Compiler:**
- AST: Try/Except/Finally and Raise statements
- Lexer: Keywords try, except, finally, raise, as
- Parser: Full Python-style exception syntax
- Type Checker: Validates exception handling code
- Codegen: LLVM IR generation with setjmp/longjmp

### Exception Flow

1. **Try Block Setup:**
   - Allocate jmp_buf on stack (200 bytes)
   - Call `exception_push_handler(jmp_buf_ptr)`
   - Call `setjmp(jmp_buf_ptr)`
   - If setjmp returns 0: execute try block
   - If setjmp returns 1: exception occurred, jump to exception handling

2. **Exception Raised:**
   - Create exception object with type, message, file, line
   - Set as current exception
   - Pop jump buffer from handler stack
   - Call `longjmp(jmp_buf, 1)` to return to try block

3. **Exception Matching:**
   - Get current exception
   - Check against each except clause type
   - If match found: execute except body, clear exception
   - If no match: re-raise (unhandled exception)

4. **Finally Block:**
   - Pop exception handler from stack
   - Execute finally statements
   - Always runs, regardless of exception

### Performance

- **Zero overhead when no exception**: setjmp is very fast (~10 instructions)
- **Exception overhead**: longjmp is fast but does unwind the stack
- **Memory**: Each try block uses 200 bytes of stack for jmp_buf
- **Better than**: Traditional error code checking (if err != 0 pattern)
- **Worse than**: Zero-cost LLVM exceptions (but much simpler to implement)

## Examples

### Basic Exception

```wadescript
def divide(a: int, b: int) -> int {
    if b == 0 {
        raise ValueError("Cannot divide by zero")
    }
    return a / b
}

def main() -> int {
    try {
        result: int = divide(10, 0)
        print_int(result)
    } except ValueError {
        print_str("Caught division by zero!")
    }
    return 0
}
```

### Multiple Except Clauses

```wadescript
def process(choice: int) -> void {
    try {
        if choice == 1 {
            raise ValueError("Invalid value")
        } else {
            raise KeyError("Key not found")
        }
    } except ValueError {
        print_str("Value error occurred")
    } except KeyError {
        print_str("Key error occurred")
    }
}
```

### Finally Block

```wadescript
def read_file() -> void {
    try {
        print_str("Opening file...")
        raise RuntimeError("File not found")
    } except RuntimeError {
        print_str("Error reading file")
    } finally {
        print_str("Cleanup: closing file")
    }
}
```

### Nested Try Blocks

```wadescript
def outer() -> void {
    try {
        print_str("Outer try")
        inner()
    } except RuntimeError {
        print_str("Caught in outer")
    }
}

def inner() -> void {
    try {
        print_str("Inner try")
        raise ValueError("Inner error")
    } except KeyError {
        print_str("Wrong handler")
    }
    # ValueError not caught here, propagates to outer
}
```

## Future Enhancements

### Not Yet Implemented

1. **Exception Properties**
   - Accessing `e.message`, `e.type`, `e.line`, `e.file`
   - Requires member access support for Exception type

2. **User-Defined Exception Types**
   - Allow users to create custom exception classes
   - Inheritance from base Exception class

3. **Re-raising Exceptions**
   - `raise` without arguments to re-raise current exception
   - Useful in except blocks

4. **Exception Chaining**
   - `raise ExceptionType("message") from original_exception`
   - Python-style exception context

5. **Context Managers**
   - `with` statement for resource management
   - Automatic exception-safe cleanup

## Testing

Comprehensive tests in:
- `examples/test_exceptions.ws` - Basic functionality
- `examples/test_exceptions_comprehensive.ws` - All features
- `examples/test_raise_simple.ws` - Simple raise without handler

All tests pass ✓

## Technical Notes

### Thread Safety

Current implementation uses global static for exception handlers and is **not thread-safe**. For multi-threading support, would need:
- Thread-local storage for exception handlers
- Thread-local current exception
- Or per-thread exception context passed as implicit parameter

### Stack Unwinding

Uses C's setjmp/longjmp which:
- ✓ Works across function boundaries
- ✓ Preserves stack frame integrity
- ✓ Compatible with C libraries
- ✗ Does not call destructors (not an issue for WadeScript)
- ✗ May leak resources if not careful with finally blocks

### Alternative: LLVM Native Exceptions

Could implement using LLVM's invoke/landingpad instructions:
- ✓ Zero-cost when no exception (no setjmp overhead)
- ✓ Proper DWARF unwinding
- ✓ Interoperable with C++ exceptions
- ✗ Much more complex to implement
- ✗ Requires personality function
- ✗ Requires landing pad generation for every function

## Comparison with Other Languages

| Feature | WadeScript | Python | C++ | Java | Go |
|---------|-----------|--------|-----|------|-----|
| Try/Catch | ✓ | ✓ | ✓ | ✓ | ✗ |
| Finally | ✓ | ✓ | ✗ | ✓ | defer |
| Multiple except | ✓ | ✓ | ✓ | ✓ | N/A |
| Exception types | String-based | Class-based | Class-based | Class-based | N/A |
| Zero-cost | ✗ | ✗ | ✓ | ✗ | N/A |
| Stack traces | ✓ | ✓ | ✗ | ✓ | N/A |

## Conclusion

WadeScript now has a robust, Python-style exception handling system that:
- Is intuitive and easy to use
- Provides clear error messages with line numbers
- Handles cleanup properly with finally blocks
- Integrates seamlessly with the existing type system
- Has minimal performance overhead

This brings WadeScript closer to being a production-ready language with proper error handling capabilities.
