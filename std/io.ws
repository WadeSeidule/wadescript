# WadeScript Standard Library: io
#
# File I/O operations
#
# Usage:
#   import "io"
#
#   handle: int = io.open("file.txt", "r")
#   content: str = io.read(handle)
#   io.close(handle)
#
# Modes:
#   "r" - read (file must exist)
#   "w" - write (creates/truncates file)
#   "a" - append (creates file if needed)

# Open a file and return a handle
# Returns a file handle (int > 0) on success
# Raises an error if file cannot be opened
def open(path: str, mode: str) -> int {
    return file_open(path, mode)
}

# Read entire file contents as a string
# Handle must be opened with "r" mode
def read(handle: int) -> str {
    return file_read(handle)
}

# Read a single line from file (without newline character)
# Returns empty string at end of file
# Handle must be opened with "r" mode
def read_line(handle: int) -> str {
    return file_read_line(handle)
}

# Write a string to file
# Handle must be opened with "w" or "a" mode
def write(handle: int, content: str) -> void {
    file_write(handle, content)
}

# Close a file handle
# Safe to call multiple times or on invalid handles
def close(handle: int) -> void {
    file_close(handle)
}

# Check if a file exists
# Returns True if file exists, False otherwise
def exists(path: str) -> bool {
    return file_exists(path) == 1
}
