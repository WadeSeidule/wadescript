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
