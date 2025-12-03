// Reference Counting Runtime for WadeScript
//
// Memory layout: [RcHeader][Object Data]
// Header contains ref_count and size for proper deallocation

use std::alloc::{alloc, dealloc, Layout};

/// Reference counted object header
/// Placed immediately before object data in memory
#[repr(C)]
struct RcHeader {
    ref_count: i64,
    size: i64,  // Size of object data (for deallocation)
}

/// Allocate reference counted memory
/// Returns pointer to object data (header is before this)
#[no_mangle]
pub extern "C" fn rc_alloc(size: i64) -> *mut u8 {
    unsafe {
        if size <= 0 {
            return std::ptr::null_mut();
        }

        let total_size = std::mem::size_of::<RcHeader>() + size as usize;
        let layout = Layout::from_size_align_unchecked(total_size, 8);
        let ptr = alloc(layout) as *mut RcHeader;

        if ptr.is_null() {
            panic!("rc_alloc: Out of memory");
        }

        // Initialize header
        (*ptr).ref_count = 1;  // Start with count of 1
        (*ptr).size = size;

        // Return pointer to data (after header)
        ptr.add(1) as *mut u8
    }
}

/// Increment reference count
#[no_mangle]
pub extern "C" fn rc_retain(ptr: *mut u8) {
    if ptr.is_null() {
        return;
    }

    unsafe {
        let header = (ptr as *mut RcHeader).sub(1);
        (*header).ref_count += 1;
    }
}

/// Decrement reference count and free if zero
#[no_mangle]
pub extern "C" fn rc_release(ptr: *mut u8) {
    if ptr.is_null() {
        return;
    }

    unsafe {
        let header = (ptr as *mut RcHeader).sub(1);
        (*header).ref_count -= 1;

        if (*header).ref_count == 0 {
            // Free the memory
            let size = (*header).size;
            let total_size = std::mem::size_of::<RcHeader>() + size as usize;
            let layout = Layout::from_size_align_unchecked(total_size, 8);
            dealloc(header as *mut u8, layout);
        } else if (*header).ref_count < 0 {
            panic!("rc_release: ref_count went negative! Double-free detected.");
        }
    }
}

/// Get current reference count (for debugging)
#[no_mangle]
pub extern "C" fn rc_get_count(ptr: *mut u8) -> i64 {
    if ptr.is_null() {
        return 0;
    }

    unsafe {
        let header = (ptr as *mut RcHeader).sub(1);
        (*header).ref_count
    }
}

/// Check if pointer is valid RC object (for debugging)
#[no_mangle]
pub extern "C" fn rc_is_valid(ptr: *mut u8) -> i32 {
    if ptr.is_null() {
        return 0;
    }

    unsafe {
        let header = (ptr as *mut RcHeader).sub(1);
        if (*header).ref_count > 0 && (*header).ref_count < 1000000 {
            1
        } else {
            0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rc_alloc_and_free() {
        unsafe {
            let ptr = rc_alloc(100);
            assert!(!ptr.is_null());
            assert_eq!(rc_get_count(ptr), 1);
            rc_release(ptr);
        }
    }

    #[test]
    fn test_rc_retain_release() {
        unsafe {
            let ptr = rc_alloc(100);
            assert_eq!(rc_get_count(ptr), 1);

            rc_retain(ptr);
            assert_eq!(rc_get_count(ptr), 2);

            rc_retain(ptr);
            assert_eq!(rc_get_count(ptr), 3);

            rc_release(ptr);
            assert_eq!(rc_get_count(ptr), 2);

            rc_release(ptr);
            assert_eq!(rc_get_count(ptr), 1);

            rc_release(ptr);
            // Memory freed, can't check count
        }
    }

    #[test]
    fn test_rc_null_safe() {
        unsafe {
            rc_retain(std::ptr::null_mut());
            rc_release(std::ptr::null_mut());
            assert_eq!(rc_get_count(std::ptr::null_mut()), 0);
        }
    }

    #[test]
    fn test_rc_is_valid() {
        unsafe {
            let ptr = rc_alloc(100);
            assert_eq!(rc_is_valid(ptr), 1);
            assert_eq!(rc_is_valid(std::ptr::null_mut()), 0);
            rc_release(ptr);
        }
    }

    #[test]
    fn test_rc_zero_size_alloc() {
        unsafe {
            // Zero size should return null
            let ptr = rc_alloc(0);
            assert!(ptr.is_null());

            // Negative size should return null
            let ptr2 = rc_alloc(-10);
            assert!(ptr2.is_null());
        }
    }

    #[test]
    fn test_rc_large_allocation() {
        unsafe {
            // Test large but reasonable allocation (1MB)
            let ptr = rc_alloc(1024 * 1024);
            assert!(!ptr.is_null());
            assert_eq!(rc_get_count(ptr), 1);

            // Write some data to verify allocation worked
            *ptr = 42;
            *ptr.add(1024 * 1024 - 1) = 99;

            assert_eq!(*ptr, 42);
            assert_eq!(*ptr.add(1024 * 1024 - 1), 99);

            rc_release(ptr);
        }
    }

    #[test]
    fn test_rc_multiple_objects() {
        unsafe {
            // Create multiple RC objects simultaneously
            let ptr1 = rc_alloc(50);
            let ptr2 = rc_alloc(100);
            let ptr3 = rc_alloc(150);

            assert!(!ptr1.is_null());
            assert!(!ptr2.is_null());
            assert!(!ptr3.is_null());

            // Each should have independent ref count
            assert_eq!(rc_get_count(ptr1), 1);
            assert_eq!(rc_get_count(ptr2), 1);
            assert_eq!(rc_get_count(ptr3), 1);

            // Retain one multiple times
            rc_retain(ptr2);
            rc_retain(ptr2);
            assert_eq!(rc_get_count(ptr1), 1);
            assert_eq!(rc_get_count(ptr2), 3);
            assert_eq!(rc_get_count(ptr3), 1);

            // Release in different order
            rc_release(ptr1);
            rc_release(ptr2);
            assert_eq!(rc_get_count(ptr2), 2);
            rc_release(ptr3);
            rc_release(ptr2);
            assert_eq!(rc_get_count(ptr2), 1);
            rc_release(ptr2);
        }
    }


    #[test]
    fn test_rc_valid_range() {
        unsafe {
            let ptr = rc_alloc(100);

            // Normal ref count should be valid
            assert_eq!(rc_is_valid(ptr), 1);

            // Retain many times - should still be valid
            for _ in 0..100 {
                rc_retain(ptr);
            }
            assert_eq!(rc_get_count(ptr), 101);
            assert_eq!(rc_is_valid(ptr), 1);

            // Release back down
            for _ in 0..100 {
                rc_release(ptr);
            }
            assert_eq!(rc_get_count(ptr), 1);

            rc_release(ptr);
        }
    }

    #[test]
    fn test_rc_get_count_edge_cases() {
        unsafe {
            // Null pointer
            assert_eq!(rc_get_count(std::ptr::null_mut()), 0);

            // Fresh allocation
            let ptr = rc_alloc(50);
            assert_eq!(rc_get_count(ptr), 1);

            // After retains
            for i in 2..=10 {
                rc_retain(ptr);
                assert_eq!(rc_get_count(ptr), i);
            }

            // After releases
            for i in (1..10).rev() {
                rc_release(ptr);
                assert_eq!(rc_get_count(ptr), i);
            }

            rc_release(ptr);
        }
    }

    #[test]
    fn test_rc_size_preservation() {
        unsafe {
            // Allocate different sizes
            let sizes = vec![1, 8, 64, 256, 1024, 4096];

            for size in sizes {
                let ptr = rc_alloc(size);
                assert!(!ptr.is_null());

                // Get header and verify size was stored
                let header = (ptr as *mut RcHeader).sub(1);
                assert_eq!((*header).size, size);
                assert_eq!((*header).ref_count, 1);

                rc_release(ptr);
            }
        }
    }
}
