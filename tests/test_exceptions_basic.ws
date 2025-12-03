# Test basic exception handling

def main() -> int {
    # Test 1: Simple raise and catch
    try {
        raise ValueError("Test error")
    } except ValueError {
        print_str("Caught ValueError")
    }

    # Test 2: No exception
    try {
        print_str("No error")
    } except ValueError {
        print_str("Should not print")
    }

    # Test 3: Finally always runs
    try {
        print_str("Try block")
    } finally {
        print_str("Finally block")
    }

    return 0
}
