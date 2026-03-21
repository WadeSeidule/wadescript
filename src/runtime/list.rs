use std::alloc::{alloc, realloc, Layout};
use std::ffi::CString;

/// List structure: { ptr data, i64 length, i64 capacity }
#[repr(C)]
pub struct List {
    pub data: *mut i64,
    pub length: i64,
    pub capacity: i64,
}

// Import the runtime_error function
extern "C" {
    fn runtime_error(message: *const i8);
}

/// Get element at index from i64 list
#[no_mangle]
pub extern "C" fn list_get_i64(list: *const List, index: i64) -> i64 {
    unsafe {
        if list.is_null() {
            let msg = CString::new("List access error: null list").unwrap();
            runtime_error(msg.as_ptr());
        }

        let list_ref = &*list;

        if index < 0 || index >= list_ref.length {
            let msg = CString::new(format!(
                "List index out of bounds: index {} is out of range for list of length {}",
                index, list_ref.length
            )).unwrap();
            runtime_error(msg.as_ptr());
        }

        *list_ref.data.offset(index as isize)
    }
}

/// Push element to i64 list
#[no_mangle]
pub extern "C" fn list_push_i64(list: *mut List, value: i64) {
    unsafe {
        if list.is_null() {
            return;
        }

        let list_ref = &mut *list;

        // Check if we need to grow
        if list_ref.length >= list_ref.capacity {
            // Grow capacity (double it, or start with 4)
            let new_capacity = if list_ref.capacity == 0 {
                4
            } else {
                list_ref.capacity * 2
            };

            // Reallocate data array
            if list_ref.data.is_null() {
                // First allocation
                let layout = Layout::array::<i64>(new_capacity as usize).unwrap();
                list_ref.data = alloc(layout) as *mut i64;
            } else {
                // Reallocation
                let old_layout = Layout::array::<i64>(list_ref.capacity as usize).unwrap();
                let new_layout = Layout::array::<i64>(new_capacity as usize).unwrap();
                list_ref.data = realloc(
                    list_ref.data as *mut u8,
                    old_layout,
                    new_layout.size(),
                ) as *mut i64;
            }

            list_ref.capacity = new_capacity;
        }

        // Add element
        *list_ref.data.offset(list_ref.length as isize) = value;
        list_ref.length += 1;
    }
}

/// Pop element from i64 list
#[no_mangle]
pub extern "C" fn list_pop_i64(list: *mut List) -> i64 {
    unsafe {
        if list.is_null() {
            let msg = CString::new("List pop error: null list").unwrap();
            runtime_error(msg.as_ptr());
        }

        let list_ref = &mut *list;

        if list_ref.length == 0 {
            let msg = CString::new("List pop error: cannot pop from empty list").unwrap();
            runtime_error(msg.as_ptr());
        }

        list_ref.length -= 1;
        *list_ref.data.offset(list_ref.length as isize)
    }
}

/// Set element at index (used for index assignment)
#[no_mangle]
pub extern "C" fn list_set_i64(list: *mut List, index: i64, value: i64) {
    unsafe {
        if list.is_null() {
            let msg = CString::new("List assignment error: null list").unwrap();
            runtime_error(msg.as_ptr());
        }

        let list_ref = &mut *list;

        if index < 0 || index >= list_ref.length {
            let msg = CString::new(format!(
                "List index out of bounds: index {} is out of range for list of length {}",
                index, list_ref.length
            )).unwrap();
            runtime_error(msg.as_ptr());
        }

        *list_ref.data.offset(index as isize) = value;
    }
}

/// Slice a list and return a new list
/// start: -1 means from beginning (0)
/// end: -1 means to end (length)
/// step: 0 means default step (1)
#[no_mangle]
pub extern "C" fn list_slice_i64(list: *const List, start: i64, end: i64, step: i64) -> *mut List {
    unsafe {
        if list.is_null() {
            let msg = CString::new("List slice error: null list").unwrap();
            runtime_error(msg.as_ptr());
        }

        let list_ref = &*list;
        let len = list_ref.length;

        // Determine actual start, end, step values
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

        // Calculate result size
        let result_size = if actual_step > 0 {
            if actual_start >= actual_end { 0 } else { ((actual_end - actual_start - 1) / actual_step + 1) as usize }
        } else {
            if actual_start <= actual_end { 0 } else { ((actual_start - actual_end - 1) / (-actual_step) + 1) as usize }
        };

        // Allocate new list
        let layout = Layout::new::<List>();
        let new_list = alloc(layout) as *mut List;

        if result_size == 0 {
            (*new_list).data = std::ptr::null_mut();
            (*new_list).length = 0;
            (*new_list).capacity = 0;
        } else {
            // Allocate data array
            let data_layout = Layout::array::<i64>(result_size).unwrap();
            let new_data = alloc(data_layout) as *mut i64;

            // Copy elements
            let mut idx = actual_start;
            let mut dest_idx = 0usize;

            if actual_step > 0 {
                while idx < actual_end && dest_idx < result_size {
                    *new_data.add(dest_idx) = *list_ref.data.offset(idx as isize);
                    idx += actual_step;
                    dest_idx += 1;
                }
            } else {
                while idx > actual_end && dest_idx < result_size {
                    *new_data.add(dest_idx) = *list_ref.data.offset(idx as isize);
                    idx += actual_step; // step is negative
                    dest_idx += 1;
                }
            }

            (*new_list).data = new_data;
            (*new_list).length = dest_idx as i64;
            (*new_list).capacity = result_size as i64;
        }

        new_list
    }
}

// ============================================================
// Float list operations (list[float])
// Same List struct, data pointer cast to *mut f64
// ============================================================

/// Float list: reinterpret data as *mut f64 since f64 and i64 are both 8 bytes
#[repr(C)]
pub struct FloatList {
    pub data: *mut f64,
    pub length: i64,
    pub capacity: i64,
}

/// Get element at index from f64 list
#[no_mangle]
pub extern "C" fn list_get_f64(list: *const List, index: i64) -> f64 {
    unsafe {
        if list.is_null() {
            let msg = CString::new("List access error: null list").unwrap();
            runtime_error(msg.as_ptr());
        }

        let list_ref = &*(list as *const FloatList);

        if index < 0 || index >= list_ref.length {
            let msg = CString::new(format!(
                "List index out of bounds: index {} is out of range for list of length {}",
                index, list_ref.length
            )).unwrap();
            runtime_error(msg.as_ptr());
        }

        *list_ref.data.offset(index as isize)
    }
}

/// Push element to f64 list
#[no_mangle]
pub extern "C" fn list_push_f64(list: *mut List, value: f64) {
    unsafe {
        if list.is_null() {
            return;
        }

        let list_ref = &mut *(list as *mut FloatList);

        if list_ref.length >= list_ref.capacity {
            let new_capacity = if list_ref.capacity == 0 { 4 } else { list_ref.capacity * 2 };

            if list_ref.data.is_null() {
                let layout = Layout::array::<f64>(new_capacity as usize).unwrap();
                list_ref.data = alloc(layout) as *mut f64;
            } else {
                let old_layout = Layout::array::<f64>(list_ref.capacity as usize).unwrap();
                let new_layout = Layout::array::<f64>(new_capacity as usize).unwrap();
                list_ref.data = realloc(
                    list_ref.data as *mut u8,
                    old_layout,
                    new_layout.size(),
                ) as *mut f64;
            }

            list_ref.capacity = new_capacity;
        }

        *list_ref.data.offset(list_ref.length as isize) = value;
        list_ref.length += 1;
    }
}

/// Pop element from f64 list
#[no_mangle]
pub extern "C" fn list_pop_f64(list: *mut List) -> f64 {
    unsafe {
        if list.is_null() {
            let msg = CString::new("List pop error: null list").unwrap();
            runtime_error(msg.as_ptr());
        }

        let list_ref = &mut *(list as *mut FloatList);

        if list_ref.length == 0 {
            let msg = CString::new("List pop error: cannot pop from empty list").unwrap();
            runtime_error(msg.as_ptr());
        }

        list_ref.length -= 1;
        *list_ref.data.offset(list_ref.length as isize)
    }
}

/// Set element at index in f64 list
#[no_mangle]
pub extern "C" fn list_set_f64(list: *mut List, index: i64, value: f64) {
    unsafe {
        if list.is_null() {
            let msg = CString::new("List assignment error: null list").unwrap();
            runtime_error(msg.as_ptr());
        }

        let list_ref = &mut *(list as *mut FloatList);

        if index < 0 || index >= list_ref.length {
            let msg = CString::new(format!(
                "List index out of bounds: index {} is out of range for list of length {}",
                index, list_ref.length
            )).unwrap();
            runtime_error(msg.as_ptr());
        }

        *list_ref.data.offset(index as isize) = value;
    }
}

/// Slice a float list and return a new list
#[no_mangle]
pub extern "C" fn list_slice_f64(list: *const List, start: i64, end: i64, step: i64) -> *mut List {
    unsafe {
        if list.is_null() {
            let msg = CString::new("List slice error: null list").unwrap();
            runtime_error(msg.as_ptr());
        }

        let list_ref = &*(list as *const FloatList);
        let len = list_ref.length;

        let actual_step = if step == 0 { 1 } else { step };

        let (actual_start, actual_end) = if actual_step > 0 {
            let s = if start == -1 { 0 } else if start < 0 { (len + start).max(0) } else { start.min(len) };
            let e = if end == -1 { len } else if end < 0 { (len + end).max(0) } else { end.min(len) };
            (s, e)
        } else {
            let s = if start == -1 { len - 1 } else if start < 0 { len + start } else { start.min(len - 1) };
            let e = if end == -1 { -1 } else if end < 0 { len + end } else { end };
            (s, e)
        };

        let result_size = if actual_step > 0 {
            if actual_start >= actual_end { 0 } else { ((actual_end - actual_start - 1) / actual_step + 1) as usize }
        } else {
            if actual_start <= actual_end { 0 } else { ((actual_start - actual_end - 1) / (-actual_step) + 1) as usize }
        };

        let layout = Layout::new::<FloatList>();
        let new_list = alloc(layout) as *mut FloatList;

        if result_size == 0 {
            (*new_list).data = std::ptr::null_mut();
            (*new_list).length = 0;
            (*new_list).capacity = 0;
        } else {
            let data_layout = Layout::array::<f64>(result_size).unwrap();
            let new_data = alloc(data_layout) as *mut f64;

            let mut idx = actual_start;
            let mut dest_idx = 0usize;

            if actual_step > 0 {
                while idx < actual_end && dest_idx < result_size {
                    *new_data.add(dest_idx) = *list_ref.data.offset(idx as isize);
                    idx += actual_step;
                    dest_idx += 1;
                }
            } else {
                while idx > actual_end && dest_idx < result_size {
                    *new_data.add(dest_idx) = *list_ref.data.offset(idx as isize);
                    idx += actual_step;
                    dest_idx += 1;
                }
            }

            (*new_list).data = new_data;
            (*new_list).length = dest_idx as i64;
            (*new_list).capacity = result_size as i64;
        }

        new_list as *mut List
    }
}

/// Convert float list to string representation
#[no_mangle]
pub extern "C" fn list_to_string_f64(list: *const List) -> *mut u8 {
    unsafe {
        if list.is_null() || (&*list).length == 0 {
            let s = "[]";
            let layout = Layout::array::<u8>(3).unwrap();
            let dest = alloc(layout) as *mut u8;
            std::ptr::copy_nonoverlapping(s.as_ptr(), dest, 2);
            *dest.add(2) = 0;
            return dest;
        }

        let list_ref = &*(list as *const FloatList);
        let mut result = String::from("[");

        for i in 0..list_ref.length {
            if i > 0 {
                result.push_str(", ");
            }
            let val = *list_ref.data.offset(i as isize);
            // Format like Python: no trailing .0 for whole numbers would be wrong,
            // always show decimal for floats
            if val == val.floor() && val.abs() < 1e15 {
                result.push_str(&format!("{:.1}", val));
            } else {
                result.push_str(&val.to_string());
            }
        }
        result.push(']');

        let len = result.len();
        let layout = Layout::array::<u8>(len + 1).unwrap();
        let dest = alloc(layout) as *mut u8;
        std::ptr::copy_nonoverlapping(result.as_ptr(), dest, len);
        *dest.add(len) = 0;
        dest
    }
}

// ============================================================
// String list operations (list[str])
// Stores string pointers as i64 values (pointer-as-int)
// ============================================================

/// Get element at index from str list (returns string pointer)
#[no_mangle]
pub extern "C" fn list_get_str(list: *const List, index: i64) -> *const u8 {
    unsafe {
        if list.is_null() {
            let msg = CString::new("List access error: null list").unwrap();
            runtime_error(msg.as_ptr());
        }

        let list_ref = &*list;

        if index < 0 || index >= list_ref.length {
            let msg = CString::new(format!(
                "List index out of bounds: index {} is out of range for list of length {}",
                index, list_ref.length
            )).unwrap();
            runtime_error(msg.as_ptr());
        }

        // String pointers stored as i64
        *list_ref.data.offset(index as isize) as *const u8
    }
}

/// Push string pointer to str list
#[no_mangle]
pub extern "C" fn list_push_str(list: *mut List, value: *const u8) {
    // Store the pointer as an i64 value
    list_push_i64(list, value as i64);
}

/// Pop string from str list (returns string pointer)
#[no_mangle]
pub extern "C" fn list_pop_str(list: *mut List) -> *const u8 {
    list_pop_i64(list) as *const u8
}

/// Set element at index in str list
#[no_mangle]
pub extern "C" fn list_set_str(list: *mut List, index: i64, value: *const u8) {
    list_set_i64(list, index, value as i64);
}

/// Slice a string list and return a new list
#[no_mangle]
pub extern "C" fn list_slice_str(list: *const List, start: i64, end: i64, step: i64) -> *mut List {
    // String pointers are stored as i64, so we can reuse the i64 slice
    list_slice_i64(list, start, end, step)
}

/// Convert string list to string representation
#[no_mangle]
pub extern "C" fn list_to_string_str(list: *const List) -> *mut u8 {
    unsafe {
        if list.is_null() || (&*list).length == 0 {
            let s = "[]";
            let layout = Layout::array::<u8>(3).unwrap();
            let dest = alloc(layout) as *mut u8;
            std::ptr::copy_nonoverlapping(s.as_ptr(), dest, 2);
            *dest.add(2) = 0;
            return dest;
        }

        let list_ref = &*list;
        let mut result = String::from("[");

        for i in 0..list_ref.length {
            if i > 0 {
                result.push_str(", ");
            }
            let str_ptr = *list_ref.data.offset(i as isize) as *const i8;
            if str_ptr.is_null() {
                result.push_str("\"\"");
            } else {
                let cstr = std::ffi::CStr::from_ptr(str_ptr);
                result.push('"');
                result.push_str(cstr.to_str().unwrap_or(""));
                result.push('"');
            }
        }
        result.push(']');

        let len = result.len();
        let layout = Layout::array::<u8>(len + 1).unwrap();
        let dest = alloc(layout) as *mut u8;
        std::ptr::copy_nonoverlapping(result.as_ptr(), dest, len);
        *dest.add(len) = 0;
        dest
    }
}

/// Convert list to string representation: [elem1, elem2, ...]
#[no_mangle]
pub extern "C" fn list_to_string(list: *const List) -> *mut u8 {
    use std::alloc::Layout;

    unsafe {
        if list.is_null() {
            // Return "[]" for null list
            let s = "[]";
            let layout = Layout::array::<u8>(3).unwrap();
            let dest = alloc(layout) as *mut u8;
            std::ptr::copy_nonoverlapping(s.as_ptr(), dest, 2);
            *dest.add(2) = 0;
            return dest;
        }

        let list_ref = &*list;

        if list_ref.length == 0 {
            // Return "[]" for empty list
            let s = "[]";
            let layout = Layout::array::<u8>(3).unwrap();
            let dest = alloc(layout) as *mut u8;
            std::ptr::copy_nonoverlapping(s.as_ptr(), dest, 2);
            *dest.add(2) = 0;
            return dest;
        }

        // Build the string representation
        let mut result = String::from("[");

        for i in 0..list_ref.length {
            if i > 0 {
                result.push_str(", ");
            }
            let val = *list_ref.data.offset(i as isize);
            result.push_str(&val.to_string());
        }
        result.push(']');

        let len = result.len();
        let layout = Layout::array::<u8>(len + 1).unwrap();
        let dest = alloc(layout) as *mut u8;
        std::ptr::copy_nonoverlapping(result.as_ptr(), dest, len);
        *dest.add(len) = 0; // Null terminator
        dest
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_list() -> Box<List> {
        Box::new(List {
            data: std::ptr::null_mut(),
            length: 0,
            capacity: 0,
        })
    }

    #[test]
    fn test_list_push_and_get() {
        let mut list = create_test_list();
        let list_ptr = &mut *list as *mut List;

        // Push some values
        list_push_i64(list_ptr, 10);
        list_push_i64(list_ptr, 20);
        list_push_i64(list_ptr, 30);

        // Check values
        assert_eq!(list_get_i64(list_ptr, 0), 10);
        assert_eq!(list_get_i64(list_ptr, 1), 20);
        assert_eq!(list_get_i64(list_ptr, 2), 30);
        assert_eq!(list.length, 3);
    }

    #[test]
    fn test_list_pop() {
        let mut list = create_test_list();
        let list_ptr = &mut *list as *mut List;

        // Push values
        list_push_i64(list_ptr, 100);
        list_push_i64(list_ptr, 200);
        list_push_i64(list_ptr, 300);

        // Pop and check
        assert_eq!(list_pop_i64(list_ptr), 300);
        assert_eq!(list.length, 2);
        assert_eq!(list_pop_i64(list_ptr), 200);
        assert_eq!(list.length, 1);
        assert_eq!(list_pop_i64(list_ptr), 100);
        assert_eq!(list.length, 0);
    }

    #[test]
    fn test_list_set() {
        let mut list = create_test_list();
        let list_ptr = &mut *list as *mut List;

        // Push values
        list_push_i64(list_ptr, 1);
        list_push_i64(list_ptr, 2);
        list_push_i64(list_ptr, 3);

        // Set and verify
        list_set_i64(list_ptr, 1, 99);
        assert_eq!(list_get_i64(list_ptr, 0), 1);
        assert_eq!(list_get_i64(list_ptr, 1), 99);
        assert_eq!(list_get_i64(list_ptr, 2), 3);
    }

    #[test]
    fn test_list_capacity_growth() {
        let mut list = create_test_list();
        let list_ptr = &mut *list as *mut List;

        // Initial capacity should be 0
        assert_eq!(list.capacity, 0);

        // Push first element, should allocate capacity of 4
        list_push_i64(list_ptr, 1);
        assert_eq!(list.capacity, 4);
        assert_eq!(list.length, 1);

        // Push more elements
        list_push_i64(list_ptr, 2);
        list_push_i64(list_ptr, 3);
        list_push_i64(list_ptr, 4);
        assert_eq!(list.capacity, 4);
        assert_eq!(list.length, 4);

        // Push one more, should double capacity
        list_push_i64(list_ptr, 5);
        assert_eq!(list.capacity, 8);
        assert_eq!(list.length, 5);
    }


    #[test]
    fn test_list_large_capacity() {
        let mut list = create_test_list();
        let list_ptr = &mut *list as *mut List;

        // Push many elements to test multiple capacity doublings
        for i in 0..100 {
            list_push_i64(list_ptr, i);
        }

        assert_eq!(list.length, 100);
        assert!(list.capacity >= 100);

        // Verify all elements
        for i in 0..100 {
            assert_eq!(list_get_i64(list_ptr, i), i);
        }
    }

    #[test]
    fn test_list_to_string() {
        use std::ffi::CStr;

        // Test empty list
        let mut empty_list = create_test_list();
        let empty_ptr = &mut *empty_list as *mut List;
        let empty_str = list_to_string(empty_ptr);
        unsafe {
            let cstr = CStr::from_ptr(empty_str as *const i8);
            assert_eq!(cstr.to_str().unwrap(), "[]");
        }

        // Test list with elements
        let mut list = create_test_list();
        let list_ptr = &mut *list as *mut List;
        list_push_i64(list_ptr, 1);
        list_push_i64(list_ptr, 2);
        list_push_i64(list_ptr, 3);

        let result = list_to_string(list_ptr);
        unsafe {
            let cstr = CStr::from_ptr(result as *const i8);
            assert_eq!(cstr.to_str().unwrap(), "[1, 2, 3]");
        }

        // Test single element
        let mut single = create_test_list();
        let single_ptr = &mut *single as *mut List;
        list_push_i64(single_ptr, 42);

        let single_str = list_to_string(single_ptr);
        unsafe {
            let cstr = CStr::from_ptr(single_str as *const i8);
            assert_eq!(cstr.to_str().unwrap(), "[42]");
        }
    }

    // Slice tests

    #[test]
    fn test_list_slice_basic() {
        let mut list = create_test_list();
        let list_ptr = &mut *list as *mut List;

        for i in 0..5 {
            list_push_i64(list_ptr, i * 10); // [0, 10, 20, 30, 40]
        }

        // slice [1:4] -> [10, 20, 30]
        let sliced = list_slice_i64(list_ptr, 1, 4, 1);
        unsafe {
            assert_eq!((*sliced).length, 3);
            assert_eq!(list_get_i64(sliced, 0), 10);
            assert_eq!(list_get_i64(sliced, 1), 20);
            assert_eq!(list_get_i64(sliced, 2), 30);
        }
    }

    #[test]
    fn test_list_slice_with_step() {
        let mut list = create_test_list();
        let list_ptr = &mut *list as *mut List;

        for i in 0..6 {
            list_push_i64(list_ptr, i); // [0, 1, 2, 3, 4, 5]
        }

        // slice [0:6:2] -> [0, 2, 4]
        let sliced = list_slice_i64(list_ptr, 0, 6, 2);
        unsafe {
            assert_eq!((*sliced).length, 3);
            assert_eq!(list_get_i64(sliced, 0), 0);
            assert_eq!(list_get_i64(sliced, 1), 2);
            assert_eq!(list_get_i64(sliced, 2), 4);
        }
    }

    #[test]
    fn test_list_slice_negative_step() {
        let mut list = create_test_list();
        let list_ptr = &mut *list as *mut List;

        for i in 0..5 {
            list_push_i64(list_ptr, i); // [0, 1, 2, 3, 4]
        }

        // slice [::-1] -> [4, 3, 2, 1, 0] (reverse)
        let sliced = list_slice_i64(list_ptr, -1, -1, -1);
        unsafe {
            assert_eq!((*sliced).length, 5);
            assert_eq!(list_get_i64(sliced, 0), 4);
            assert_eq!(list_get_i64(sliced, 1), 3);
            assert_eq!(list_get_i64(sliced, 2), 2);
            assert_eq!(list_get_i64(sliced, 3), 1);
            assert_eq!(list_get_i64(sliced, 4), 0);
        }
    }

    #[test]
    fn test_list_slice_empty_result() {
        let mut list = create_test_list();
        let list_ptr = &mut *list as *mut List;

        list_push_i64(list_ptr, 1);
        list_push_i64(list_ptr, 2);

        // slice [2:1] with step 1 -> empty
        let sliced = list_slice_i64(list_ptr, 2, 1, 1);
        unsafe {
            assert_eq!((*sliced).length, 0);
        }
    }

    #[test]
    fn test_list_slice_full_copy() {
        let mut list = create_test_list();
        let list_ptr = &mut *list as *mut List;

        for i in 0..3 {
            list_push_i64(list_ptr, i + 1); // [1, 2, 3]
        }

        // slice [:] -> [1, 2, 3]  (default start=-1, end=-1, step=1)
        let sliced = list_slice_i64(list_ptr, -1, -1, 1);
        unsafe {
            assert_eq!((*sliced).length, 3);
            assert_eq!(list_get_i64(sliced, 0), 1);
            assert_eq!(list_get_i64(sliced, 1), 2);
            assert_eq!(list_get_i64(sliced, 2), 3);
        }
    }

    // Float list tests

    fn create_test_float_list() -> Box<FloatList> {
        Box::new(FloatList {
            data: std::ptr::null_mut(),
            length: 0,
            capacity: 0,
        })
    }

    #[test]
    fn test_float_list_push_and_get() {
        let mut list = create_test_float_list();
        let list_ptr = &mut *list as *mut FloatList as *mut List;

        list_push_f64(list_ptr, 1.5);
        list_push_f64(list_ptr, 2.7);
        list_push_f64(list_ptr, 3.14);

        assert_eq!(list_get_f64(list_ptr, 0), 1.5);
        assert_eq!(list_get_f64(list_ptr, 1), 2.7);
        assert_eq!(list_get_f64(list_ptr, 2), 3.14);
        assert_eq!(list.length, 3);
    }

    #[test]
    fn test_float_list_pop() {
        let mut list = create_test_float_list();
        let list_ptr = &mut *list as *mut FloatList as *mut List;

        list_push_f64(list_ptr, 1.1);
        list_push_f64(list_ptr, 2.2);

        assert_eq!(list_pop_f64(list_ptr), 2.2);
        assert_eq!(list.length, 1);
        assert_eq!(list_pop_f64(list_ptr), 1.1);
        assert_eq!(list.length, 0);
    }

    #[test]
    fn test_float_list_set() {
        let mut list = create_test_float_list();
        let list_ptr = &mut *list as *mut FloatList as *mut List;

        list_push_f64(list_ptr, 1.0);
        list_push_f64(list_ptr, 2.0);
        list_push_f64(list_ptr, 3.0);

        list_set_f64(list_ptr, 1, 99.9);
        assert_eq!(list_get_f64(list_ptr, 1), 99.9);
    }

    #[test]
    fn test_float_list_to_string() {
        use std::ffi::CStr;

        let mut list = create_test_float_list();
        let list_ptr = &mut *list as *mut FloatList as *mut List;

        list_push_f64(list_ptr, 1.5);
        list_push_f64(list_ptr, 2.0);
        list_push_f64(list_ptr, 3.14);

        let result = list_to_string_f64(list_ptr);
        unsafe {
            let cstr = CStr::from_ptr(result as *const i8);
            assert_eq!(cstr.to_str().unwrap(), "[1.5, 2.0, 3.14]");
        }
    }

    // Float list slice tests

    #[test]
    fn test_float_list_slice_basic() {
        let mut list = create_test_float_list();
        let list_ptr = &mut *list as *mut FloatList as *mut List;

        list_push_f64(list_ptr, 1.1);
        list_push_f64(list_ptr, 2.2);
        list_push_f64(list_ptr, 3.3);
        list_push_f64(list_ptr, 4.4);

        // slice [1:3] -> [2.2, 3.3]
        let sliced = list_slice_f64(list_ptr, 1, 3, 1);
        unsafe {
            assert_eq!((*sliced).length, 2);
            assert_eq!(list_get_f64(sliced, 0), 2.2);
            assert_eq!(list_get_f64(sliced, 1), 3.3);
        }
    }

    #[test]
    fn test_float_list_slice_reverse() {
        let mut list = create_test_float_list();
        let list_ptr = &mut *list as *mut FloatList as *mut List;

        list_push_f64(list_ptr, 1.0);
        list_push_f64(list_ptr, 2.0);
        list_push_f64(list_ptr, 3.0);

        // slice [::-1] -> [3.0, 2.0, 1.0]
        let sliced = list_slice_f64(list_ptr, -1, -1, -1);
        unsafe {
            assert_eq!((*sliced).length, 3);
            assert_eq!(list_get_f64(sliced, 0), 3.0);
            assert_eq!(list_get_f64(sliced, 1), 2.0);
            assert_eq!(list_get_f64(sliced, 2), 1.0);
        }
    }

    // String list tests

    #[test]
    fn test_str_list_pop() {
        let mut list = create_test_list();
        let list_ptr = &mut *list as *mut List;

        let s1 = CString::new("first").unwrap();
        let s2 = CString::new("second").unwrap();

        list_push_str(list_ptr, s1.as_ptr() as *const u8);
        list_push_str(list_ptr, s2.as_ptr() as *const u8);
        assert_eq!(list.length, 2);

        let popped = list_pop_str(list_ptr);
        unsafe {
            let cstr = std::ffi::CStr::from_ptr(popped as *const i8);
            assert_eq!(cstr.to_str().unwrap(), "second");
        }
        assert_eq!(list.length, 1);

        let popped2 = list_pop_str(list_ptr);
        unsafe {
            let cstr = std::ffi::CStr::from_ptr(popped2 as *const i8);
            assert_eq!(cstr.to_str().unwrap(), "first");
        }
        assert_eq!(list.length, 0);
    }

    #[test]
    fn test_str_list_set() {
        let mut list = create_test_list();
        let list_ptr = &mut *list as *mut List;

        let s1 = CString::new("hello").unwrap();
        let s2 = CString::new("world").unwrap();
        let s3 = CString::new("replaced").unwrap();

        list_push_str(list_ptr, s1.as_ptr() as *const u8);
        list_push_str(list_ptr, s2.as_ptr() as *const u8);

        list_set_str(list_ptr, 1, s3.as_ptr() as *const u8);

        let result = list_get_str(list_ptr, 1);
        unsafe {
            let cstr = std::ffi::CStr::from_ptr(result as *const i8);
            assert_eq!(cstr.to_str().unwrap(), "replaced");
        }
    }

    #[test]
    fn test_str_list_push_and_get() {
        let mut list = create_test_list();
        let list_ptr = &mut *list as *mut List;

        let s1 = CString::new("hello").unwrap();
        let s2 = CString::new("world").unwrap();

        list_push_str(list_ptr, s1.as_ptr() as *const u8);
        list_push_str(list_ptr, s2.as_ptr() as *const u8);

        let r1 = list_get_str(list_ptr, 0);
        let r2 = list_get_str(list_ptr, 1);

        unsafe {
            let cstr1 = std::ffi::CStr::from_ptr(r1 as *const i8);
            let cstr2 = std::ffi::CStr::from_ptr(r2 as *const i8);
            assert_eq!(cstr1.to_str().unwrap(), "hello");
            assert_eq!(cstr2.to_str().unwrap(), "world");
        }
        assert_eq!(list.length, 2);
    }

    #[test]
    fn test_str_list_to_string() {
        use std::ffi::CStr;

        let mut list = create_test_list();
        let list_ptr = &mut *list as *mut List;

        let s1 = CString::new("hello").unwrap();
        let s2 = CString::new("world").unwrap();

        list_push_str(list_ptr, s1.as_ptr() as *const u8);
        list_push_str(list_ptr, s2.as_ptr() as *const u8);

        let result = list_to_string_str(list_ptr);
        unsafe {
            let cstr = CStr::from_ptr(result as *const i8);
            assert_eq!(cstr.to_str().unwrap(), "[\"hello\", \"world\"]");
        }
    }
}
