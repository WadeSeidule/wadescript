def main() -> int {
    try {
        print_str("In try")
        raise ValueError("Test")
    } except ValueError {
        print_str("In except")
    }
    print_str("After try")
    return 0
}
