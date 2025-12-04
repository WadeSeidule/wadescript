//! CLI argument parsing runtime for WadeScript
//!
//! Provides functions to access command-line arguments and parse values.

use std::alloc::{alloc, Layout};
use std::ffi::{CStr, CString};
use std::os::raw::c_char;
use std::ptr;
use std::sync::OnceLock;

// Cache for command-line arguments (to avoid repeated allocations)
static ARGS_CACHE: OnceLock<Vec<CString>> = OnceLock::new();

fn get_cached_args() -> &'static Vec<CString> {
    ARGS_CACHE.get_or_init(|| {
        std::env::args()
            .map(|s| CString::new(s).unwrap_or_else(|_| CString::new("").unwrap()))
            .collect()
    })
}

/// Get command line argument count
#[no_mangle]
pub extern "C" fn cli_get_argc() -> i64 {
    get_cached_args().len() as i64
}

/// Get command line argument at index
/// Returns a C string pointer (caller does not own, do not free)
#[no_mangle]
pub extern "C" fn cli_get_argv(index: i64) -> *const u8 {
    let args = get_cached_args();

    if index < 0 || index as usize >= args.len() {
        return ptr::null();
    }

    args[index as usize].as_ptr() as *const u8
}

/// Get command line argument at index, returning a newly allocated copy
/// Returns a C string pointer (caller owns, should free)
#[no_mangle]
pub extern "C" fn cli_get_argv_copy(index: i64) -> *mut u8 {
    let args = get_cached_args();

    if index < 0 || index as usize >= args.len() {
        return ptr::null_mut();
    }

    let arg = &args[index as usize];
    let bytes = arg.as_bytes_with_nul();

    unsafe {
        let layout = Layout::array::<u8>(bytes.len()).unwrap();
        let dest = alloc(layout);

        if dest.is_null() {
            return ptr::null_mut();
        }

        ptr::copy_nonoverlapping(bytes.as_ptr(), dest, bytes.len());
        dest
    }
}

/// Parse integer from string
/// Returns 0 on error (and prints error message)
#[no_mangle]
pub extern "C" fn cli_parse_int(s: *const u8) -> i64 {
    if s.is_null() {
        eprintln!("CLI error: cannot parse null as int");
        return 0;
    }

    unsafe {
        let s_str = match CStr::from_ptr(s as *const c_char).to_str() {
            Ok(s) => s,
            Err(_) => {
                eprintln!("CLI error: invalid string encoding");
                return 0;
            }
        };

        match s_str.parse::<i64>() {
            Ok(n) => n,
            Err(_) => {
                eprintln!("CLI error: '{}' is not a valid integer", s_str);
                0
            }
        }
    }
}

/// Parse boolean from string (handles "true", "false", "1", "0", "yes", "no")
/// Returns 0 (false) on error
#[no_mangle]
pub extern "C" fn cli_parse_bool(s: *const u8) -> i64 {
    if s.is_null() {
        return 0;
    }

    unsafe {
        let s_str = match CStr::from_ptr(s as *const c_char).to_str() {
            Ok(s) => s.to_lowercase(),
            Err(_) => return 0,
        };

        match s_str.as_str() {
            "true" | "1" | "yes" => 1,
            "false" | "0" | "no" | "" => 0,
            _ => {
                eprintln!("CLI error: '{}' is not a valid boolean", s_str);
                0
            }
        }
    }
}

/// Check if string starts with prefix
/// Returns 1 if true, 0 if false
#[no_mangle]
pub extern "C" fn cli_starts_with(s: *const u8, prefix: *const u8) -> i64 {
    if s.is_null() || prefix.is_null() {
        return 0;
    }

    unsafe {
        let s_bytes = CStr::from_ptr(s as *const c_char).to_bytes();
        let prefix_bytes = CStr::from_ptr(prefix as *const c_char).to_bytes();

        if s_bytes.starts_with(prefix_bytes) {
            1
        } else {
            0
        }
    }
}

/// Compare two C strings for equality
/// Returns 1 if equal, 0 if not
#[no_mangle]
pub extern "C" fn cli_str_eq(a: *const u8, b: *const u8) -> i64 {
    if a.is_null() && b.is_null() {
        return 1;
    }
    if a.is_null() || b.is_null() {
        return 0;
    }

    unsafe {
        let a_str = CStr::from_ptr(a as *const c_char);
        let b_str = CStr::from_ptr(b as *const c_char);

        if a_str == b_str {
            1
        } else {
            0
        }
    }
}

/// Get substring after a prefix (e.g., "--name=value" with prefix "--name=" returns "value")
/// Returns pointer to the character after prefix, or null if doesn't start with prefix
#[no_mangle]
pub extern "C" fn cli_after_prefix(s: *const u8, prefix: *const u8) -> *const u8 {
    if s.is_null() || prefix.is_null() {
        return ptr::null();
    }

    unsafe {
        let s_bytes = CStr::from_ptr(s as *const c_char).to_bytes();
        let prefix_bytes = CStr::from_ptr(prefix as *const c_char).to_bytes();

        if s_bytes.starts_with(prefix_bytes) {
            s.add(prefix_bytes.len())
        } else {
            ptr::null()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cli_parse_int() {
        let s = CString::new("42").unwrap();
        assert_eq!(cli_parse_int(s.as_ptr() as *const u8), 42);

        let s = CString::new("-123").unwrap();
        assert_eq!(cli_parse_int(s.as_ptr() as *const u8), -123);

        let s = CString::new("0").unwrap();
        assert_eq!(cli_parse_int(s.as_ptr() as *const u8), 0);
    }

    #[test]
    fn test_cli_parse_bool() {
        let s = CString::new("true").unwrap();
        assert_eq!(cli_parse_bool(s.as_ptr() as *const u8), 1);

        let s = CString::new("false").unwrap();
        assert_eq!(cli_parse_bool(s.as_ptr() as *const u8), 0);

        let s = CString::new("1").unwrap();
        assert_eq!(cli_parse_bool(s.as_ptr() as *const u8), 1);

        let s = CString::new("yes").unwrap();
        assert_eq!(cli_parse_bool(s.as_ptr() as *const u8), 1);
    }

    #[test]
    fn test_cli_str_eq() {
        let a = CString::new("hello").unwrap();
        let b = CString::new("hello").unwrap();
        assert_eq!(cli_str_eq(a.as_ptr() as *const u8, b.as_ptr() as *const u8), 1);

        let c = CString::new("world").unwrap();
        assert_eq!(cli_str_eq(a.as_ptr() as *const u8, c.as_ptr() as *const u8), 0);
    }

    #[test]
    fn test_cli_starts_with() {
        let s = CString::new("--verbose").unwrap();
        let prefix = CString::new("--").unwrap();
        assert_eq!(cli_starts_with(s.as_ptr() as *const u8, prefix.as_ptr() as *const u8), 1);

        let prefix2 = CString::new("-x").unwrap();
        assert_eq!(cli_starts_with(s.as_ptr() as *const u8, prefix2.as_ptr() as *const u8), 0);
    }

    #[test]
    fn test_cli_after_prefix() {
        let s = CString::new("--name=value").unwrap();
        let prefix = CString::new("--name=").unwrap();
        let result = cli_after_prefix(s.as_ptr() as *const u8, prefix.as_ptr() as *const u8);
        assert!(!result.is_null());
        unsafe {
            let result_str = CStr::from_ptr(result as *const c_char).to_str().unwrap();
            assert_eq!(result_str, "value");
        }
    }
}
