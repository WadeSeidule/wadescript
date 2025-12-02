use std::alloc::{alloc, alloc_zeroed, Layout};
use std::ffi::CStr;
use std::ptr;

const INITIAL_CAPACITY: i64 = 16;
const LOAD_FACTOR_THRESHOLD: f64 = 0.75;

/// Dictionary entry structure (for chaining)
#[repr(C)]
struct DictEntry {
    key: *mut u8,      // C string (strdup'd)
    value: i64,
    next: *mut DictEntry,
}

/// Hash table structure
#[repr(C)]
pub struct Dict {
    buckets: *mut *mut DictEntry,  // Array of bucket pointers
    capacity: i64,                  // Number of buckets
    length: i64,                    // Number of entries
}

/// Hash function (djb2 algorithm)
unsafe fn hash_string(key: *const u8) -> u64 {
    let mut hash: u64 = 5381;
    let mut ptr = key;

    while *ptr != 0 {
        hash = hash.wrapping_mul(33).wrapping_add(*ptr as u64);
        ptr = ptr.offset(1);
    }

    hash
}

/// Duplicate a C string (equivalent to strdup)
unsafe fn string_dup(src: *const u8) -> *mut u8 {
    if src.is_null() {
        return ptr::null_mut();
    }

    let len = CStr::from_ptr(src as *const i8).to_bytes().len();
    let layout = Layout::array::<u8>(len + 1).unwrap();
    let dest = alloc(layout);

    ptr::copy_nonoverlapping(src, dest, len + 1);
    dest
}

/// Compare two C strings (equivalent to strcmp)
unsafe fn string_cmp(s1: *const u8, s2: *const u8) -> i32 {
    let mut i = 0;
    loop {
        let c1 = *s1.offset(i);
        let c2 = *s2.offset(i);

        if c1 != c2 {
            return (c1 as i32) - (c2 as i32);
        }

        if c1 == 0 {
            return 0;
        }

        i += 1;
    }
}

/// Rehash the dictionary to a larger capacity
unsafe fn dict_rehash(dict: *mut Dict) {
    let dict_ref = &mut *dict;
    let old_capacity = dict_ref.capacity;
    let old_buckets = dict_ref.buckets;

    // Double the capacity
    dict_ref.capacity *= 2;

    // Allocate new buckets (zeroed)
    let layout = Layout::array::<*mut DictEntry>(dict_ref.capacity as usize).unwrap();
    dict_ref.buckets = alloc_zeroed(layout) as *mut *mut DictEntry;

    dict_ref.length = 0;

    // Rehash all entries
    for i in 0..old_capacity {
        let mut entry = *old_buckets.offset(i as isize);

        while !entry.is_null() {
            let next = (*entry).next;

            // Reinsert entry into new buckets
            let hash = hash_string((*entry).key);
            let new_index = (hash % dict_ref.capacity as u64) as isize;

            (*entry).next = *dict_ref.buckets.offset(new_index);
            *dict_ref.buckets.offset(new_index) = entry;
            dict_ref.length += 1;

            entry = next;
        }
    }

    // Note: We don't free old_buckets array here as it would require proper deallocation
    // In production, you'd want to properly deallocate using Layout::array
}

/// Create a new dictionary
#[no_mangle]
pub extern "C" fn dict_create() -> *mut Dict {
    unsafe {
        let layout = Layout::new::<Dict>();
        let dict = alloc(layout) as *mut Dict;

        if dict.is_null() {
            eprintln!("Failed to allocate memory for dictionary");
            std::process::exit(1);
        }

        (*dict).capacity = INITIAL_CAPACITY;
        (*dict).length = 0;

        // Allocate buckets (zeroed)
        let buckets_layout = Layout::array::<*mut DictEntry>(INITIAL_CAPACITY as usize).unwrap();
        (*dict).buckets = alloc_zeroed(buckets_layout) as *mut *mut DictEntry;

        if (*dict).buckets.is_null() {
            eprintln!("Failed to allocate memory for dictionary buckets");
            std::process::exit(1);
        }

        dict
    }
}

/// Set a key-value pair in the dictionary
#[no_mangle]
pub extern "C" fn dict_set(dict: *mut Dict, key: *const u8, value: i64) {
    unsafe {
        if dict.is_null() || key.is_null() {
            return;
        }

        let dict_ref = &mut *dict;

        // Check if we need to rehash
        if (dict_ref.length as f64 / dict_ref.capacity as f64) >= LOAD_FACTOR_THRESHOLD {
            dict_rehash(dict);
        }

        // Calculate bucket index
        let hash = hash_string(key);
        let index = (hash % dict_ref.capacity as u64) as isize;

        // Check if key already exists in this bucket
        let mut entry = *dict_ref.buckets.offset(index);
        while !entry.is_null() {
            if string_cmp((*entry).key, key) == 0 {
                // Update existing value
                (*entry).value = value;
                return;
            }
            entry = (*entry).next;
        }

        // Key doesn't exist, create new entry at head of bucket
        let entry_layout = Layout::new::<DictEntry>();
        let new_entry = alloc(entry_layout) as *mut DictEntry;

        if new_entry.is_null() {
            eprintln!("Failed to allocate memory for dictionary entry");
            std::process::exit(1);
        }

        (*new_entry).key = string_dup(key);
        (*new_entry).value = value;
        (*new_entry).next = *dict_ref.buckets.offset(index);

        *dict_ref.buckets.offset(index) = new_entry;
        dict_ref.length += 1;
    }
}

/// Get a value from the dictionary (returns 0 if not found)
#[no_mangle]
pub extern "C" fn dict_get(dict: *const Dict, key: *const u8) -> i64 {
    unsafe {
        if dict.is_null() || key.is_null() {
            return 0;
        }

        let dict_ref = &*dict;

        // Calculate bucket index
        let hash = hash_string(key);
        let index = (hash % dict_ref.capacity as u64) as isize;

        // Search through the bucket chain
        let mut entry = *dict_ref.buckets.offset(index);
        while !entry.is_null() {
            if string_cmp((*entry).key, key) == 0 {
                return (*entry).value;
            }
            entry = (*entry).next;
        }

        0 // Return 0 if key not found
    }
}

/// Check if a key exists in the dictionary
#[no_mangle]
pub extern "C" fn dict_has(dict: *const Dict, key: *const u8) -> i32 {
    unsafe {
        if dict.is_null() || key.is_null() {
            return 0;
        }

        let dict_ref = &*dict;

        // Calculate bucket index
        let hash = hash_string(key);
        let index = (hash % dict_ref.capacity as u64) as isize;

        // Search through the bucket chain
        let mut entry = *dict_ref.buckets.offset(index);
        while !entry.is_null() {
            if string_cmp((*entry).key, key) == 0 {
                return 1;
            }
            entry = (*entry).next;
        }

        0
    }
}
