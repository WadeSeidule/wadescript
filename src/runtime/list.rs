use std::alloc::{alloc, realloc, Layout};

/// List structure: { ptr data, i64 length, i64 capacity }
#[repr(C)]
pub struct List {
    data: *mut i64,
    length: i64,
    capacity: i64,
}

/// Get element at index from i64 list
#[no_mangle]
pub extern "C" fn list_get_i64(list: *const List, index: i64) -> i64 {
    unsafe {
        if list.is_null() {
            return 0;
        }

        let list_ref = &*list;

        if index < 0 || index >= list_ref.length {
            return 0; // Out of bounds
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
            return 0;
        }

        let list_ref = &mut *list;

        if list_ref.length == 0 {
            return 0;
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
            return;
        }

        let list_ref = &mut *list;

        if index < 0 || index >= list_ref.length {
            return; // Out of bounds
        }

        *list_ref.data.offset(index as isize) = value;
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
    fn test_list_out_of_bounds() {
        let mut list = create_test_list();
        let list_ptr = &mut *list as *mut List;

        list_push_i64(list_ptr, 10);

        // Out of bounds access should return 0
        assert_eq!(list_get_i64(list_ptr, -1), 0);
        assert_eq!(list_get_i64(list_ptr, 5), 0);

        // Out of bounds set should be no-op
        list_set_i64(list_ptr, -1, 99);
        list_set_i64(list_ptr, 5, 99);
        assert_eq!(list.length, 1); // Should not have changed
    }

    #[test]
    fn test_list_pop_empty() {
        let mut list = create_test_list();
        let list_ptr = &mut *list as *mut List;

        // Popping from empty list should return 0
        assert_eq!(list_pop_i64(list_ptr), 0);
        assert_eq!(list.length, 0);
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
