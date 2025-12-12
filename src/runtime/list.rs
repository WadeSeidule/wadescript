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
}
