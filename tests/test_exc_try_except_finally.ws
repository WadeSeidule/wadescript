def main() -> int {
    print_str("Test 1: Exception caught")
    try {
        print_str("In try")
        raise ValueError("Test error")
    } except ValueError {
        print_str("In except")
    } finally {
        print_str("In finally")
    }
    print_str("After try-except-finally")

    print_str("Test 2: No exception")
    try {
        print_str("In try 2")
    } except ValueError {
        print_str("Should not print")
    } finally {
        print_str("In finally 2")
    }
    print_str("Done")

    return 0
}
