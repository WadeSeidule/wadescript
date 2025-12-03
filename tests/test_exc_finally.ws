def main() -> int {
    try {
        print_str("Try block")
    } finally {
        print_str("Finally block")
    }
    return 0
}
