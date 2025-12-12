use std::alloc::{alloc, Layout};
use std::ffi::CStr;
use std::ptr;

/// Get the length of a C string
#[no_mangle]
pub extern "C" fn str_length(s: *const u8) -> i64 {
    unsafe {
        if s.is_null() {
            return 0;
        }
        CStr::from_ptr(s as *const i8).to_bytes().len() as i64
    }
}

/// Convert string to uppercase
#[no_mangle]
pub extern "C" fn str_upper(s: *const u8) -> *mut u8 {
    unsafe {
        if s.is_null() {
            return ptr::null_mut();
        }

        let c_str = CStr::from_ptr(s as *const i8);
        let rust_str = c_str.to_str().unwrap_or("");
        let upper = rust_str.to_uppercase();

        // Allocate new C string
        let len = upper.len();
        let layout = Layout::array::<u8>(len + 1).unwrap();
        let dest = alloc(layout);

        ptr::copy_nonoverlapping(upper.as_ptr(), dest, len);
        *dest.add(len) = 0; // Null terminator

        dest
    }
}

/// Convert string to lowercase
#[no_mangle]
pub extern "C" fn str_lower(s: *const u8) -> *mut u8 {
    unsafe {
        if s.is_null() {
            return ptr::null_mut();
        }

        let c_str = CStr::from_ptr(s as *const i8);
        let rust_str = c_str.to_str().unwrap_or("");
        let lower = rust_str.to_lowercase();

        // Allocate new C string
        let len = lower.len();
        let layout = Layout::array::<u8>(len + 1).unwrap();
        let dest = alloc(layout);

        ptr::copy_nonoverlapping(lower.as_ptr(), dest, len);
        *dest.add(len) = 0; // Null terminator

        dest
    }
}

/// Check if string contains substring
#[no_mangle]
pub extern "C" fn str_contains(s: *const u8, substring: *const u8) -> i32 {
    unsafe {
        if s.is_null() || substring.is_null() {
            return 0;
        }

        let s_cstr = CStr::from_ptr(s as *const i8);
        let sub_cstr = CStr::from_ptr(substring as *const i8);

        let s_str = s_cstr.to_str().unwrap_or("");
        let sub_str = sub_cstr.to_str().unwrap_or("");

        if s_str.contains(sub_str) { 1 } else { 0 }
    }
}

/// Get character at index as a single-character string
#[no_mangle]
pub extern "C" fn str_char_at(s: *const u8, index: i64) -> *mut u8 {
    unsafe {
        if s.is_null() || index < 0 {
            return ptr::null_mut();
        }

        let c_str = CStr::from_ptr(s as *const i8);
        let rust_str = c_str.to_str().unwrap_or("");
        let chars: Vec<char> = rust_str.chars().collect();

        if (index as usize) >= chars.len() {
            return ptr::null_mut();
        }

        let ch = chars[index as usize];
        let char_str = ch.to_string();

        // Allocate new C string for single character
        let len = char_str.len();
        let layout = Layout::array::<u8>(len + 1).unwrap();
        let dest = alloc(layout);

        ptr::copy_nonoverlapping(char_str.as_ptr(), dest, len);
        *dest.add(len) = 0; // Null terminator

        dest
    }
}

/// Slice a string and return a new string
/// start: -1 means from beginning (0)
/// end: -1 means to end (length)
/// step: 0 means default step (1)
#[no_mangle]
pub extern "C" fn str_slice(s: *const u8, start: i64, end: i64, step: i64) -> *mut u8 {
    unsafe {
        if s.is_null() {
            return ptr::null_mut();
        }

        let c_str = CStr::from_ptr(s as *const i8);
        let rust_str = c_str.to_str().unwrap_or("");
        let chars: Vec<char> = rust_str.chars().collect();
        let len = chars.len() as i64;

        // Determine actual step
        let actual_step = if step == 0 { 1 } else { step };

        // Handle negative indices and defaults
        let (actual_start, actual_end) = if actual_step > 0 {
            // Forward iteration
            let s = if start == -1 { 0 } else if start < 0 { (len + start).max(0) } else { start.min(len) };
            let e = if end == -1 { len } else if end < 0 { (len + end).max(0) } else { end.min(len) };
            (s, e)
        } else {
            // Backward iteration (negative step)
            let s = if start == -1 { len - 1 } else if start < 0 { len + start } else { start.min(len - 1) };
            let e = if end == -1 { -1 } else if end < 0 { len + end } else { end };
            (s, e)
        };

        // Collect characters based on slice
        let mut result_chars: Vec<char> = Vec::new();
        let mut idx = actual_start;

        if actual_step > 0 {
            while idx < actual_end && idx < len {
                result_chars.push(chars[idx as usize]);
                idx += actual_step;
            }
        } else {
            while idx > actual_end && idx >= 0 {
                result_chars.push(chars[idx as usize]);
                idx += actual_step; // step is negative
            }
        }

        let result_str: String = result_chars.into_iter().collect();

        // Allocate new C string
        let result_len = result_str.len();
        let layout = Layout::array::<u8>(result_len + 1).unwrap();
        let dest = alloc(layout);

        ptr::copy_nonoverlapping(result_str.as_ptr(), dest, result_len);
        *dest.add(result_len) = 0; // Null terminator

        dest
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::ffi::CString;

    #[test]
    fn test_str_length() {
        let s = CString::new("hello").unwrap();
        assert_eq!(str_length(s.as_ptr() as *const u8), 5);

        let empty = CString::new("").unwrap();
        assert_eq!(str_length(empty.as_ptr() as *const u8), 0);
    }

    #[test]
    fn test_str_upper() {
        let s = CString::new("hello").unwrap();
        let result = str_upper(s.as_ptr() as *const u8);

        unsafe {
            let result_cstr = CStr::from_ptr(result as *const i8);
            assert_eq!(result_cstr.to_str().unwrap(), "HELLO");
        }
    }

    #[test]
    fn test_str_lower() {
        let s = CString::new("HELLO").unwrap();
        let result = str_lower(s.as_ptr() as *const u8);

        unsafe {
            let result_cstr = CStr::from_ptr(result as *const i8);
            assert_eq!(result_cstr.to_str().unwrap(), "hello");
        }
    }

    #[test]
    fn test_str_contains() {
        let s = CString::new("hello world").unwrap();
        let sub1 = CString::new("world").unwrap();
        let sub2 = CString::new("foo").unwrap();

        assert_eq!(str_contains(s.as_ptr() as *const u8, sub1.as_ptr() as *const u8), 1);
        assert_eq!(str_contains(s.as_ptr() as *const u8, sub2.as_ptr() as *const u8), 0);
    }

    #[test]
    fn test_str_char_at() {
        let s = CString::new("hello").unwrap();

        let ch0 = str_char_at(s.as_ptr() as *const u8, 0);
        let ch4 = str_char_at(s.as_ptr() as *const u8, 4);
        let ch_out = str_char_at(s.as_ptr() as *const u8, 10);

        unsafe {
            assert_eq!(CStr::from_ptr(ch0 as *const i8).to_str().unwrap(), "h");
            assert_eq!(CStr::from_ptr(ch4 as *const i8).to_str().unwrap(), "o");
            assert!(ch_out.is_null());
        }
    }

    #[test]
    fn test_str_length_null() {
        // Null pointer should return 0
        assert_eq!(str_length(ptr::null()), 0);
    }

    #[test]
    fn test_str_upper_null() {
        // Null pointer should return null
        assert!(str_upper(ptr::null()).is_null());
    }

    #[test]
    fn test_str_lower_null() {
        // Null pointer should return null
        assert!(str_lower(ptr::null()).is_null());
    }

    #[test]
    fn test_str_contains_null() {
        let s = CString::new("hello").unwrap();
        let sub = CString::new("lo").unwrap();

        // Null string should return 0
        assert_eq!(str_contains(ptr::null(), sub.as_ptr() as *const u8), 0);
        // Null substring should return 0
        assert_eq!(str_contains(s.as_ptr() as *const u8, ptr::null()), 0);
        // Both null should return 0
        assert_eq!(str_contains(ptr::null(), ptr::null()), 0);
    }

    #[test]
    fn test_str_char_at_null() {
        // Null pointer should return null
        assert!(str_char_at(ptr::null(), 0).is_null());
    }

    #[test]
    fn test_str_char_at_negative_index() {
        let s = CString::new("hello").unwrap();
        // Negative index should return null
        assert!(str_char_at(s.as_ptr() as *const u8, -1).is_null());
        assert!(str_char_at(s.as_ptr() as *const u8, -100).is_null());
    }

    #[test]
    fn test_str_upper_mixed_case() {
        let s = CString::new("Hello World!").unwrap();
        let result = str_upper(s.as_ptr() as *const u8);

        unsafe {
            let result_cstr = CStr::from_ptr(result as *const i8);
            assert_eq!(result_cstr.to_str().unwrap(), "HELLO WORLD!");
        }
    }

    #[test]
    fn test_str_lower_mixed_case() {
        let s = CString::new("Hello World!").unwrap();
        let result = str_lower(s.as_ptr() as *const u8);

        unsafe {
            let result_cstr = CStr::from_ptr(result as *const i8);
            assert_eq!(result_cstr.to_str().unwrap(), "hello world!");
        }
    }

    #[test]
    fn test_str_contains_empty_substring() {
        let s = CString::new("hello").unwrap();
        let empty = CString::new("").unwrap();

        // Empty substring is contained in any string
        assert_eq!(str_contains(s.as_ptr() as *const u8, empty.as_ptr() as *const u8), 1);
    }

    #[test]
    fn test_str_contains_exact_match() {
        let s = CString::new("hello").unwrap();
        let same = CString::new("hello").unwrap();

        // Exact match should return 1
        assert_eq!(str_contains(s.as_ptr() as *const u8, same.as_ptr() as *const u8), 1);
    }

    #[test]
    fn test_str_upper_empty() {
        let empty = CString::new("").unwrap();
        let result = str_upper(empty.as_ptr() as *const u8);

        unsafe {
            let result_cstr = CStr::from_ptr(result as *const i8);
            assert_eq!(result_cstr.to_str().unwrap(), "");
        }
    }

    #[test]
    fn test_str_lower_empty() {
        let empty = CString::new("").unwrap();
        let result = str_lower(empty.as_ptr() as *const u8);

        unsafe {
            let result_cstr = CStr::from_ptr(result as *const i8);
            assert_eq!(result_cstr.to_str().unwrap(), "");
        }
    }

    #[test]
    fn test_str_char_at_empty() {
        let empty = CString::new("").unwrap();
        // Any index on empty string should return null
        assert!(str_char_at(empty.as_ptr() as *const u8, 0).is_null());
    }

    #[test]
    fn test_str_upper_numbers_and_symbols() {
        let s = CString::new("abc123!@#").unwrap();
        let result = str_upper(s.as_ptr() as *const u8);

        unsafe {
            let result_cstr = CStr::from_ptr(result as *const i8);
            assert_eq!(result_cstr.to_str().unwrap(), "ABC123!@#");
        }
    }

    #[test]
    fn test_str_lower_numbers_and_symbols() {
        let s = CString::new("ABC123!@#").unwrap();
        let result = str_lower(s.as_ptr() as *const u8);

        unsafe {
            let result_cstr = CStr::from_ptr(result as *const i8);
            assert_eq!(result_cstr.to_str().unwrap(), "abc123!@#");
        }
    }

    #[test]
    fn test_str_contains_case_sensitive() {
        let s = CString::new("Hello World").unwrap();
        let sub1 = CString::new("hello").unwrap();
        let sub2 = CString::new("Hello").unwrap();

        // Should be case-sensitive
        assert_eq!(str_contains(s.as_ptr() as *const u8, sub1.as_ptr() as *const u8), 0);
        assert_eq!(str_contains(s.as_ptr() as *const u8, sub2.as_ptr() as *const u8), 1);
    }

    #[test]
    fn test_str_length_various_sizes() {
        let strings = vec![
            ("", 0),
            ("a", 1),
            ("hello", 5),
            ("hello world", 11),
            ("The quick brown fox jumps over the lazy dog", 43),
        ];

        for (text, expected_len) in strings {
            let s = CString::new(text).unwrap();
            assert_eq!(str_length(s.as_ptr() as *const u8), expected_len);
        }
    }
}
