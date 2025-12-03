# Comprehensive exception handling test

def test_basic_exception() -> void {
    print_str("=== Test 1: Basic exception ===")
    try {
        raise ValueError("This is a test error")
    } except ValueError {
        print_str("Caught ValueError")
    }
    print_str("")
}

def test_multiple_except() -> void {
    print_str("=== Test 2: Multiple except clauses ===")
    error_type: int = 1

    try {
        if error_type == 1 {
            raise ValueError("Value error occurred")
        } else {
            raise KeyError("Key error occurred")
        }
    } except ValueError {
        print_str("Caught ValueError")
    } except KeyError {
        print_str("Caught KeyError")
    }
    print_str("")
}

def test_finally() -> void {
    print_str("=== Test 3: Finally block always executes ===")
    try {
        print_str("In try block")
        raise RuntimeError("Error in try")
    } except RuntimeError {
        print_str("In except block")
    } finally {
        print_str("Finally block executed")
    }
    print_str("")
}

def test_no_exception() -> void {
    print_str("=== Test 4: No exception raised ===")
    try {
        print_str("Normal execution")
    } except ValueError {
        print_str("This should not print")
    } finally {
        print_str("Finally still executes")
    }
    print_str("")
}

def divide_safe(a: int, b: int) -> void {
    try {
        if b == 0 {
            raise ValueError("Division by zero")
        }
        result: int = a / b
        print_str("Result:")
        print_int(result)
    } except ValueError {
        print_str("Cannot divide by zero!")
    }
}

def test_nested_calls() -> void {
    print_str("=== Test 5: Exception in nested call ===")
    divide_safe(10, 2)
    divide_safe(10, 0)
    print_str("")
}

def main() -> int {
    test_basic_exception()
    test_multiple_except()
    test_finally()
    test_no_exception()
    test_nested_calls()

    print_str("=== All tests completed ===")
    return 0
}
