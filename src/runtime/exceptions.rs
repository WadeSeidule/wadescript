use std::ffi::CStr;
use std::os::raw::{c_char, c_int};
use std::ptr;

// Exception structure: { exception_type, message, file, line }
#[repr(C)]
pub struct Exception {
    pub exception_type: *const c_char,
    pub message: *const c_char,
    pub file: *const c_char,
    pub line: i64,
}

// Jump buffer for setjmp/longjmp (opaque, platform specific size)
#[repr(C)]
pub struct JmpBuf {
    _private: [u8; 200], // Large enough for most platforms
}

// External C functions
extern "C" {
    #[allow(dead_code)]
    pub fn setjmp(env: *mut JmpBuf) -> c_int;
    pub fn longjmp(env: *mut JmpBuf, val: c_int) -> !;
}

// Global exception state
static mut CURRENT_EXCEPTION: *mut Exception = ptr::null_mut();

// Stack of exception handlers (jump buffers) - using unsafe static with manual synchronization
// In a real implementation, this would use thread-local storage
static mut EXCEPTION_HANDLERS: Vec<*mut JmpBuf> = Vec::new();

/// Create a new exception object
#[no_mangle]
pub extern "C" fn exception_create(
    exception_type: *const c_char,
    message: *const c_char,
    file: *const c_char,
    line: i64,
) -> *mut Exception {
    let exc = Box::new(Exception {
        exception_type,
        message,
        file,
        line,
    });
    Box::into_raw(exc)
}

/// Get the current exception
#[no_mangle]
pub extern "C" fn exception_get_current() -> *mut Exception {
    unsafe { CURRENT_EXCEPTION }
}

/// Set the current exception
#[no_mangle]
pub extern "C" fn exception_set_current(exc: *mut Exception) {
    unsafe {
        CURRENT_EXCEPTION = exc;
    }
}

/// Clear the current exception
#[no_mangle]
pub extern "C" fn exception_clear() {
    unsafe {
        if !CURRENT_EXCEPTION.is_null() {
            let _ = Box::from_raw(CURRENT_EXCEPTION);
            CURRENT_EXCEPTION = ptr::null_mut();
        }
    }
}

/// Get exception type as string
#[no_mangle]
pub extern "C" fn exception_get_type(exc: *const Exception) -> *const c_char {
    unsafe {
        if exc.is_null() {
            return ptr::null();
        }
        (*exc).exception_type
    }
}

/// Get exception message as string
#[no_mangle]
pub extern "C" fn exception_get_message(exc: *const Exception) -> *const c_char {
    unsafe {
        if exc.is_null() {
            return ptr::null();
        }
        (*exc).message
    }
}

/// Check if exception matches a type (returns 1 if match, 0 if not)
#[no_mangle]
pub extern "C" fn exception_matches(exc: *const Exception, exception_type: *const c_char) -> c_int {
    unsafe {
        if exc.is_null() || exception_type.is_null() {
            return 0;
        }

        let exc_type = CStr::from_ptr((*exc).exception_type);
        let check_type = CStr::from_ptr(exception_type);

        if exc_type == check_type {
            1
        } else {
            0
        }
    }
}

/// Push an exception handler onto the stack
#[no_mangle]
pub extern "C" fn exception_push_handler(jmp_buf: *mut JmpBuf) {
    unsafe {
        (*std::ptr::addr_of_mut!(EXCEPTION_HANDLERS)).push(jmp_buf);
    }
}

/// Pop an exception handler from the stack
#[no_mangle]
pub extern "C" fn exception_pop_handler() {
    unsafe {
        (*std::ptr::addr_of_mut!(EXCEPTION_HANDLERS)).pop();
    }
}

/// Raise an exception (does not return)
#[no_mangle]
pub extern "C" fn exception_raise(
    exception_type: *const c_char,
    message: *const c_char,
    file: *const c_char,
    line: i64,
) -> ! {
    unsafe {
        // Create exception object
        let exc = exception_create(exception_type, message, file, line);
        exception_set_current(exc);

        // Try to longjmp to nearest exception handler
        if let Some(jmp_buf) = (*std::ptr::addr_of_mut!(EXCEPTION_HANDLERS)).pop() {
            // Jump back to the try block with value 1 (indicating exception)
            longjmp(jmp_buf, 1);
        }

        // No exception handler found - unhandled exception
        let exc_type_str = CStr::from_ptr(exception_type).to_str().unwrap_or("Unknown");
        let msg_str = CStr::from_ptr(message).to_str().unwrap_or("");

        eprintln!("\n\x1b[31;1mUnhandled Exception:\x1b[0m {} - {}", exc_type_str, msg_str);
        std::process::exit(1);
    }
}
