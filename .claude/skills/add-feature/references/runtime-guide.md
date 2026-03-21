# Adding Runtime Functions

When a feature needs runtime support (heap operations, I/O, string manipulation, etc.), follow these three steps.

## 1. Implement the Function

Add to the appropriate file in `src/runtime/`:
- `list.rs` — list operations
- `dict.rs` — dictionary operations
- `string.rs` — string operations
- `rc.rs` — reference counting
- `exceptions.rs` — exception handling
- `io.rs` — file I/O
- `cli.rs` — CLI argument access
- `http.rs` — HTTP client

```rust
#[no_mangle]
pub extern "C" fn ws_my_function(arg: i64) -> i64 {
    // Implementation
}
```

Requirements:
- `#[no_mangle]` — prevents Rust name mangling
- `pub extern "C"` — C ABI for LLVM interop
- Use C-compatible types (i64, f64, *mut u8, *mut c_void, etc.)
- Prefix with `ws_` for namespace clarity

## 2. Declare in Code Generator

In `src/codegen.rs`, find the appropriate `declare_*_functions` method and add:

```rust
// Declare the function signature so LLVM knows about it
let fn_type = self.context.i64_type().fn_type(&[self.context.i64_type().into()], false);
self.module.add_function("ws_my_function", fn_type, None);
```

Match the LLVM types to the Rust function signature exactly.

## 3. Register for JIT

In `src/runtime_symbols.rs`, inside `get_runtime_symbols()`:

```rust
use crate::runtime::module_name::ws_my_function;

// In the symbols vector:
RuntimeSymbol { name: "ws_my_function", addr: ws_my_function as usize },
```

This ensures the REPL's JIT engine can resolve the function at runtime.

## Common Patterns

### Returning strings
```rust
#[no_mangle]
pub extern "C" fn ws_make_string() -> *mut u8 {
    let s = CString::new("result").unwrap();
    s.into_raw() as *mut u8
}
```

### Receiving strings
```rust
#[no_mangle]
pub extern "C" fn ws_process_string(s: *const u8) -> i64 {
    let c_str = unsafe { CStr::from_ptr(s as *const i8) };
    let rust_str = c_str.to_str().unwrap();
    // ...
}
```

### Working with lists
```rust
#[no_mangle]
pub extern "C" fn ws_list_operation(list_ptr: *mut c_void) -> i64 {
    let list = unsafe { &mut *(list_ptr as *mut WsList) };
    // ...
}
```
