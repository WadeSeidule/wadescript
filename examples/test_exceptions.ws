# Test exception handling syntax

def divide(a: int, b: int) -> int {
    if b == 0 {
        raise ValueError("Cannot divide by zero")
    }
    return a / b
}

def main() -> int {
    try {
        result: int = divide(10, 0)
        print_int(result)
    } except ValueError as e {
        print_str("Caught ValueError!")
    } finally {
        print_str("Cleanup in finally block")
    }

    print_str("Program continues after exception")
    return 0
}
