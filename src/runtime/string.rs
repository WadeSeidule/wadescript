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
}
