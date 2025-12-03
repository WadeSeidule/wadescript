//! WadeScript Runtime Library
//!
//! Provides C-compatible functions for list, dictionary, and string operations.
//! This is compiled as a static library and linked with generated WadeScript programs.

#![allow(dead_code)]

pub mod list;
pub mod dict;
pub mod string;

use std::ffi::CStr;
use std::os::raw::c_char;
use std::sync::Mutex;
use backtrace::Backtrace;

// Re-export the functions to ensure they're available for linking
pub use list::*;
pub use dict::*;
pub use string::*;

// Global call stack for stack traces
static CALL_STACK: Mutex<Vec<String>> = Mutex::new(Vec::new());

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

                // Capture backtrace with symbol resolution
                let bt = Backtrace::new();

                // Collect all .ws frames from the backtrace
                let mut ws_frames: Vec<(String, u32)> = Vec::new();
                for frame in bt.frames() {
                    for symbol in frame.symbols() {
                        if let Some(filename) = symbol.filename() {
                            if let Some(filename_str) = filename.to_str() {
                                if filename_str.ends_with(".ws") {
                                    if let Some(line) = symbol.lineno() {
                                        ws_frames.push((filename_str.to_string(), line));
                                        break; // Only take first .ws symbol per frame
                                    }
                                }
                            }
                        }
                    }
                }

                // Show stack trace with line numbers from debug info
                if !ws_frames.is_empty() {
                    eprintln!("\n\x1b[36;1mStack trace:\x1b[0m");
                    for (file, line) in ws_frames {
                        eprintln!("  at {}:{}", file, line);
                    }
                } else if let Ok(stack) = CALL_STACK.lock() {
                    // Fallback to manual call stack if no debug info found
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
