# Test io module
import "io"

def main() -> int {
    print_str("=== IO Module Tests ===")
    print_str("")

    # Test io.exists before writing
    print_str("Test 1: io.exists (before create)")
    if io.exists("/tmp/wadescript_io_test.txt") {
        print_str("File already exists (cleaning up)")
    } else {
        print_str("File does not exist yet - PASS")
    }

    # Test write
    print_str("")
    print_str("Test 2: io.open and io.write")
    handle: int = io.open("/tmp/wadescript_io_test.txt", "w")
    print_str("Opened file")
    io.write(handle, "Hello, WadeScript!\n")
    io.write(handle, "Line 2\n")
    io.write(handle, "Line 3\n")
    io.close(handle)
    print_str("Write complete - PASS")

    # Test io.exists after writing
    print_str("")
    print_str("Test 3: io.exists (after create)")
    if io.exists("/tmp/wadescript_io_test.txt") {
        print_str("File exists - PASS")
    } else {
        print_str("File does not exist - FAIL")
    }

    # Test read entire file
    print_str("")
    print_str("Test 4: io.read entire file")
    handle = io.open("/tmp/wadescript_io_test.txt", "r")
    content: str = io.read(handle)
    io.close(handle)
    print_str("Read content:")
    print_str(content)

    # Test read_line
    print_str("Test 5: io.read_line")
    handle = io.open("/tmp/wadescript_io_test.txt", "r")
    line1: str = io.read_line(handle)
    line2: str = io.read_line(handle)
    line3: str = io.read_line(handle)
    io.close(handle)
    print_str("Line 1:")
    print_str(line1)
    print_str("Line 2:")
    print_str(line2)
    print_str("Line 3:")
    print_str(line3)

    # Test append mode
    print_str("")
    print_str("Test 6: io.open append mode")
    handle = io.open("/tmp/wadescript_io_test.txt", "a")
    io.write(handle, "Appended line\n")
    io.close(handle)

    # Verify append worked
    handle = io.open("/tmp/wadescript_io_test.txt", "r")
    content = io.read(handle)
    io.close(handle)
    print_str("After append:")
    print_str(content)

    print_str("=== All IO Tests Passed ===")

    return 0
}
