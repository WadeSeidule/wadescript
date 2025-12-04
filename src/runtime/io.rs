// File I/O Runtime for WadeScript
//
// Provides basic file operations:
// - open(path, mode) -> handle
// - read(handle) -> string
// - read_line(handle) -> string
// - write(handle, content)
// - close(handle)
// - exists(path) -> bool

use std::alloc::{alloc, Layout};
use std::collections::HashMap;
use std::ffi::{CStr, CString};
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Read, Write};
use std::path::Path;
use std::ptr;
use std::sync::Mutex;

// Global file handle storage
// Maps handle IDs to file objects
lazy_static::lazy_static! {
    static ref FILE_HANDLES: Mutex<FileHandleManager> = Mutex::new(FileHandleManager::new());
}

struct FileHandleManager {
    handles: HashMap<i64, FileHandle>,
    next_id: i64,
}

enum FileHandle {
    Read(BufReader<File>),
    Write(File),
    Append(File),
}

impl FileHandleManager {
    fn new() -> Self {
        FileHandleManager {
            handles: HashMap::new(),
            next_id: 1, // Start at 1, 0 means error
        }
    }

    fn add(&mut self, handle: FileHandle) -> i64 {
        let id = self.next_id;
        self.next_id += 1;
        self.handles.insert(id, handle);
        id
    }

    fn get(&mut self, id: i64) -> Option<&mut FileHandle> {
        self.handles.get_mut(&id)
    }

    fn remove(&mut self, id: i64) -> Option<FileHandle> {
        self.handles.remove(&id)
    }
}

// Import runtime_error for error reporting
extern "C" {
    fn runtime_error(message: *const i8);
}

/// Open a file
/// mode: "r" = read, "w" = write (create/truncate), "a" = append
/// Returns: file handle (>0 on success, calls runtime_error on failure)
#[no_mangle]
pub extern "C" fn file_open(path: *const u8, mode: *const u8) -> i64 {
    unsafe {
        if path.is_null() {
            let msg = CString::new("File open error: null path").unwrap();
            runtime_error(msg.as_ptr());
            return 0;
        }
        if mode.is_null() {
            let msg = CString::new("File open error: null mode").unwrap();
            runtime_error(msg.as_ptr());
            return 0;
        }

        let path_str = match CStr::from_ptr(path as *const i8).to_str() {
            Ok(s) => s,
            Err(_) => {
                let msg = CString::new("File open error: invalid path encoding").unwrap();
                runtime_error(msg.as_ptr());
                return 0;
            }
        };

        let mode_str = match CStr::from_ptr(mode as *const i8).to_str() {
            Ok(s) => s,
            Err(_) => {
                let msg = CString::new("File open error: invalid mode encoding").unwrap();
                runtime_error(msg.as_ptr());
                return 0;
            }
        };

        let result = match mode_str {
            "r" => {
                File::open(path_str).map(|f| FileHandle::Read(BufReader::new(f)))
            }
            "w" => {
                OpenOptions::new()
                    .write(true)
                    .create(true)
                    .truncate(true)
                    .open(path_str)
                    .map(FileHandle::Write)
            }
            "a" => {
                OpenOptions::new()
                    .write(true)
                    .create(true)
                    .append(true)
                    .open(path_str)
                    .map(FileHandle::Append)
            }
            _ => {
                let msg = CString::new(format!(
                    "File open error: invalid mode '{}' (use 'r', 'w', or 'a')",
                    mode_str
                )).unwrap();
                runtime_error(msg.as_ptr());
                return 0;
            }
        };

        match result {
            Ok(handle) => {
                let mut manager = FILE_HANDLES.lock().unwrap();
                manager.add(handle)
            }
            Err(e) => {
                let msg = CString::new(format!(
                    "File open error: cannot open '{}': {}",
                    path_str, e
                )).unwrap();
                runtime_error(msg.as_ptr());
                0
            }
        }
    }
}

/// Read entire file contents as string
/// Returns: pointer to null-terminated string (caller should not free - managed by WadeScript)
#[no_mangle]
pub extern "C" fn file_read(handle: i64) -> *mut u8 {
    unsafe {
        let mut manager = FILE_HANDLES.lock().unwrap();

        let file_handle = match manager.get(handle) {
            Some(h) => h,
            None => {
                let msg = CString::new(format!(
                    "File read error: invalid handle {}",
                    handle
                )).unwrap();
                runtime_error(msg.as_ptr());
                return ptr::null_mut();
            }
        };

        let contents = match file_handle {
            FileHandle::Read(reader) => {
                let mut contents = String::new();
                if let Err(e) = reader.get_mut().read_to_string(&mut contents) {
                    let msg = CString::new(format!("File read error: {}", e)).unwrap();
                    runtime_error(msg.as_ptr());
                    return ptr::null_mut();
                }
                contents
            }
            _ => {
                let msg = CString::new("File read error: file not opened for reading").unwrap();
                runtime_error(msg.as_ptr());
                return ptr::null_mut();
            }
        };

        // Allocate and copy string
        let len = contents.len();
        let layout = Layout::array::<u8>(len + 1).unwrap();
        let dest = alloc(layout);

        ptr::copy_nonoverlapping(contents.as_ptr(), dest, len);
        *dest.add(len) = 0; // Null terminator

        dest
    }
}

/// Read a single line from file
/// Returns: pointer to null-terminated string (without newline)
#[no_mangle]
pub extern "C" fn file_read_line(handle: i64) -> *mut u8 {
    unsafe {
        let mut manager = FILE_HANDLES.lock().unwrap();

        let file_handle = match manager.get(handle) {
            Some(h) => h,
            None => {
                let msg = CString::new(format!(
                    "File read_line error: invalid handle {}",
                    handle
                )).unwrap();
                runtime_error(msg.as_ptr());
                return ptr::null_mut();
            }
        };

        let line = match file_handle {
            FileHandle::Read(reader) => {
                let mut line = String::new();
                match reader.read_line(&mut line) {
                    Ok(0) => String::new(), // EOF
                    Ok(_) => {
                        // Remove trailing newline
                        if line.ends_with('\n') {
                            line.pop();
                            if line.ends_with('\r') {
                                line.pop();
                            }
                        }
                        line
                    }
                    Err(e) => {
                        let msg = CString::new(format!("File read_line error: {}", e)).unwrap();
                        runtime_error(msg.as_ptr());
                        return ptr::null_mut();
                    }
                }
            }
            _ => {
                let msg = CString::new("File read_line error: file not opened for reading").unwrap();
                runtime_error(msg.as_ptr());
                return ptr::null_mut();
            }
        };

        // Allocate and copy string
        let len = line.len();
        let layout = Layout::array::<u8>(len + 1).unwrap();
        let dest = alloc(layout);

        ptr::copy_nonoverlapping(line.as_ptr(), dest, len);
        *dest.add(len) = 0; // Null terminator

        dest
    }
}

/// Write string to file
#[no_mangle]
pub extern "C" fn file_write(handle: i64, content: *const u8) {
    unsafe {
        if content.is_null() {
            let msg = CString::new("File write error: null content").unwrap();
            runtime_error(msg.as_ptr());
            return;
        }

        let content_str = match CStr::from_ptr(content as *const i8).to_str() {
            Ok(s) => s,
            Err(_) => {
                let msg = CString::new("File write error: invalid content encoding").unwrap();
                runtime_error(msg.as_ptr());
                return;
            }
        };

        let mut manager = FILE_HANDLES.lock().unwrap();

        let file_handle = match manager.get(handle) {
            Some(h) => h,
            None => {
                let msg = CString::new(format!(
                    "File write error: invalid handle {}",
                    handle
                )).unwrap();
                runtime_error(msg.as_ptr());
                return;
            }
        };

        let result = match file_handle {
            FileHandle::Write(file) | FileHandle::Append(file) => {
                file.write_all(content_str.as_bytes())
            }
            FileHandle::Read(_) => {
                let msg = CString::new("File write error: file not opened for writing").unwrap();
                runtime_error(msg.as_ptr());
                return;
            }
        };

        if let Err(e) = result {
            let msg = CString::new(format!("File write error: {}", e)).unwrap();
            runtime_error(msg.as_ptr());
        }
    }
}

/// Close a file handle
#[no_mangle]
pub extern "C" fn file_close(handle: i64) {
    let mut manager = FILE_HANDLES.lock().unwrap();

    if manager.remove(handle).is_none() {
        // Silently ignore closing an invalid handle
        // (common pattern: close in finally block even if open failed)
    }
    // File is automatically closed when dropped
}

/// Check if a file exists
/// Returns: 1 if exists, 0 if not
#[no_mangle]
pub extern "C" fn file_exists(path: *const u8) -> i64 {
    unsafe {
        if path.is_null() {
            return 0;
        }

        let path_str = match CStr::from_ptr(path as *const i8).to_str() {
            Ok(s) => s,
            Err(_) => return 0,
        };

        if Path::new(path_str).exists() { 1 } else { 0 }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_file_exists() {
        // Create a temp file
        let test_path = "/tmp/wadescript_test_exists.txt";
        fs::write(test_path, "test").unwrap();

        let path = CString::new(test_path).unwrap();
        assert_eq!(file_exists(path.as_ptr() as *const u8), 1);

        // Remove and check again
        fs::remove_file(test_path).unwrap();
        assert_eq!(file_exists(path.as_ptr() as *const u8), 0);
    }

    #[test]
    fn test_file_exists_null() {
        assert_eq!(file_exists(ptr::null()), 0);
    }

    #[test]
    fn test_file_write_and_read() {
        let test_path = "/tmp/wadescript_test_rw.txt";
        let path = CString::new(test_path).unwrap();
        let mode_w = CString::new("w").unwrap();
        let mode_r = CString::new("r").unwrap();

        // Write to file
        let handle = file_open(path.as_ptr() as *const u8, mode_w.as_ptr() as *const u8);
        assert!(handle > 0);

        let content = CString::new("Hello, WadeScript!").unwrap();
        file_write(handle, content.as_ptr() as *const u8);
        file_close(handle);

        // Read from file
        let handle = file_open(path.as_ptr() as *const u8, mode_r.as_ptr() as *const u8);
        assert!(handle > 0);

        let result = file_read(handle);
        unsafe {
            let result_str = CStr::from_ptr(result as *const i8).to_str().unwrap();
            assert_eq!(result_str, "Hello, WadeScript!");
        }
        file_close(handle);

        // Cleanup
        fs::remove_file(test_path).ok();
    }

    #[test]
    fn test_file_read_line() {
        let test_path = "/tmp/wadescript_test_lines.txt";
        let path = CString::new(test_path).unwrap();
        let mode_w = CString::new("w").unwrap();
        let mode_r = CString::new("r").unwrap();

        // Write multiple lines
        let handle = file_open(path.as_ptr() as *const u8, mode_w.as_ptr() as *const u8);
        let content = CString::new("Line 1\nLine 2\nLine 3\n").unwrap();
        file_write(handle, content.as_ptr() as *const u8);
        file_close(handle);

        // Read lines one by one
        let handle = file_open(path.as_ptr() as *const u8, mode_r.as_ptr() as *const u8);

        unsafe {
            let line1 = file_read_line(handle);
            assert_eq!(CStr::from_ptr(line1 as *const i8).to_str().unwrap(), "Line 1");

            let line2 = file_read_line(handle);
            assert_eq!(CStr::from_ptr(line2 as *const i8).to_str().unwrap(), "Line 2");

            let line3 = file_read_line(handle);
            assert_eq!(CStr::from_ptr(line3 as *const i8).to_str().unwrap(), "Line 3");
        }

        file_close(handle);
        fs::remove_file(test_path).ok();
    }

    #[test]
    fn test_file_append() {
        let test_path = "/tmp/wadescript_test_append.txt";
        let path = CString::new(test_path).unwrap();
        let mode_w = CString::new("w").unwrap();
        let mode_a = CString::new("a").unwrap();
        let mode_r = CString::new("r").unwrap();

        // Write initial content
        let handle = file_open(path.as_ptr() as *const u8, mode_w.as_ptr() as *const u8);
        let content = CString::new("First").unwrap();
        file_write(handle, content.as_ptr() as *const u8);
        file_close(handle);

        // Append more content
        let handle = file_open(path.as_ptr() as *const u8, mode_a.as_ptr() as *const u8);
        let content = CString::new("Second").unwrap();
        file_write(handle, content.as_ptr() as *const u8);
        file_close(handle);

        // Read and verify
        let handle = file_open(path.as_ptr() as *const u8, mode_r.as_ptr() as *const u8);
        let result = file_read(handle);
        unsafe {
            let result_str = CStr::from_ptr(result as *const i8).to_str().unwrap();
            assert_eq!(result_str, "FirstSecond");
        }
        file_close(handle);

        fs::remove_file(test_path).ok();
    }
}
