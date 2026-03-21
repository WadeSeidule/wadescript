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

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::{CStr, CString};

    #[test]
    fn test_exception_create() {
        let exc_type = CString::new("ValueError").unwrap();
        let message = CString::new("invalid value").unwrap();
        let file = CString::new("test.ws").unwrap();

        let exc = exception_create(
            exc_type.as_ptr(),
            message.as_ptr(),
            file.as_ptr(),
            42,
        );

        assert!(!exc.is_null());
        unsafe {
            let exc_ref = &*exc;
            assert_eq!(CStr::from_ptr(exc_ref.exception_type), exc_type.as_c_str());
            assert_eq!(CStr::from_ptr(exc_ref.message), message.as_c_str());
            assert_eq!(CStr::from_ptr(exc_ref.file), file.as_c_str());
            assert_eq!(exc_ref.line, 42);
            let _ = Box::from_raw(exc);
        }
    }

    #[test]
    fn test_exception_get_set_current() {
        unsafe {
            // Clear any existing state
            CURRENT_EXCEPTION = ptr::null_mut();

            assert!(exception_get_current().is_null());

            let exc_type = CString::new("Error").unwrap();
            let message = CString::new("test").unwrap();
            let file = CString::new("test.ws").unwrap();
            let exc = exception_create(exc_type.as_ptr(), message.as_ptr(), file.as_ptr(), 1);

            exception_set_current(exc);
            assert!(!exception_get_current().is_null());
            assert_eq!(exception_get_current(), exc);

            // Cleanup
            exception_clear();
        }
    }

    #[test]
    fn test_exception_clear() {
        unsafe {
            CURRENT_EXCEPTION = ptr::null_mut();

            let exc_type = CString::new("Error").unwrap();
            let message = CString::new("msg").unwrap();
            let file = CString::new("f.ws").unwrap();
            let exc = exception_create(exc_type.as_ptr(), message.as_ptr(), file.as_ptr(), 1);

            exception_set_current(exc);
            assert!(!exception_get_current().is_null());

            exception_clear();
            assert!(exception_get_current().is_null());

            // Double clear should be safe
            exception_clear();
            assert!(exception_get_current().is_null());
        }
    }

    #[test]
    fn test_exception_get_type_and_message() {
        let exc_type = CString::new("TypeError").unwrap();
        let message = CString::new("expected int, got str").unwrap();
        let file = CString::new("test.ws").unwrap();

        let exc = exception_create(exc_type.as_ptr(), message.as_ptr(), file.as_ptr(), 10);

        let got_type = exception_get_type(exc);
        let got_msg = exception_get_message(exc);

        unsafe {
            assert_eq!(CStr::from_ptr(got_type), exc_type.as_c_str());
            assert_eq!(CStr::from_ptr(got_msg), message.as_c_str());
            let _ = Box::from_raw(exc);
        }
    }

    #[test]
    fn test_exception_get_type_null() {
        assert!(exception_get_type(ptr::null()).is_null());
        assert!(exception_get_message(ptr::null()).is_null());
    }

    #[test]
    fn test_exception_matches() {
        let exc_type = CString::new("ValueError").unwrap();
        let message = CString::new("bad").unwrap();
        let file = CString::new("test.ws").unwrap();

        let exc = exception_create(exc_type.as_ptr(), message.as_ptr(), file.as_ptr(), 1);

        let check_match = CString::new("ValueError").unwrap();
        let check_no_match = CString::new("TypeError").unwrap();

        assert_eq!(exception_matches(exc, check_match.as_ptr()), 1);
        assert_eq!(exception_matches(exc, check_no_match.as_ptr()), 0);

        unsafe { let _ = Box::from_raw(exc); }
    }

    #[test]
    fn test_exception_matches_null() {
        let exc_type = CString::new("Error").unwrap();
        let message = CString::new("msg").unwrap();
        let file = CString::new("f.ws").unwrap();
        let exc = exception_create(exc_type.as_ptr(), message.as_ptr(), file.as_ptr(), 1);

        assert_eq!(exception_matches(ptr::null(), exc_type.as_ptr()), 0);
        assert_eq!(exception_matches(exc, ptr::null()), 0);
        assert_eq!(exception_matches(ptr::null(), ptr::null()), 0);

        unsafe { let _ = Box::from_raw(exc); }
    }

    #[test]
    fn test_exception_push_pop_handler() {
        unsafe {
            (*std::ptr::addr_of_mut!(EXCEPTION_HANDLERS)).clear();

            let mut buf = JmpBuf { _private: [0u8; 200] };
            let buf_ptr = &mut buf as *mut JmpBuf;

            exception_push_handler(buf_ptr);
            assert_eq!((*std::ptr::addr_of!(EXCEPTION_HANDLERS)).len(), 1);

            exception_pop_handler();
            assert_eq!((*std::ptr::addr_of!(EXCEPTION_HANDLERS)).len(), 0);

            // Pop on empty should not panic
            exception_pop_handler();
        }
    }
}
