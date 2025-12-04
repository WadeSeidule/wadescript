// Runtime library for WadeScript
// Provides list and dictionary implementations with C-compatible FFI

pub mod list;
pub mod dict;
pub mod string;
pub mod rc;
pub mod io;
pub mod exceptions;

use std::ffi::CStr;
use std::os::raw::c_char;
use std::sync::Mutex;

// Global call stack for stack traces
pub static CALL_STACK: Mutex<Vec<String>> = Mutex::new(Vec::new());

/// Push a function name onto the call stack
#[no_mangle]
pub extern "C" fn push_call_stack(func_name: *const c_char) {
    unsafe {
        if !func_name.is_null() {
            if let Ok(name) = CStr::from_ptr(func_name).to_str() {
                if let Ok(mut stack) = CALL_STACK.lock() {
                    stack.push(name.to_string());
                }
            }
        }
    }
}

/// Pop a function name from the call stack
#[no_mangle]
pub extern "C" fn pop_call_stack() {
    if let Ok(mut stack) = CALL_STACK.lock() {
        stack.pop();
    }
}

/// Print runtime error message with stack trace and exit
#[no_mangle]
pub extern "C" fn runtime_error(message: *const c_char) {
    unsafe {
        if !message.is_null() {
            if let Ok(msg) = CStr::from_ptr(message).to_str() {
                eprintln!("\n\x1b[31;1mRuntime Error:\x1b[0m {}", msg);

                // Show call stack if available
                if let Ok(stack) = CALL_STACK.lock() {
                    if !stack.is_empty() {
                        eprintln!("\n\x1b[36;1mCall stack:\x1b[0m");
                        for (i, func) in stack.iter().rev().enumerate() {
                            eprintln!("  \x1b[90m{}\x1b[0m. {}", i + 1, func);
                        }
                    }
                }
            }
        }
        std::process::exit(1);
    }
}
