# Test: Functions - definition, calls, recursion

def add(a: int, b: int) -> int {
    return a + b
}

def multiply(x: int, y: int) -> int {
    return x * y
}

def factorial(n: int) -> int {
    if n <= 1 {
        return 1
    } else {
        return n * factorial(n - 1)
    }
}

def fibonacci(n: int) -> int {
    if n <= 1 {
        return n
    } else {
        return fibonacci(n - 1) + fibonacci(n - 2)
    }
}

def no_return_value() -> void {
    x: int = 5
}

def main() -> int {
    # Basic function calls
    assert add(3, 4) == 7
    assert add(10, 20) == 30
    assert multiply(5, 6) == 30
    assert multiply(7, 8) == 56

    # Recursive functions
    assert factorial(5) == 120
    assert factorial(0) == 1
    assert factorial(3) == 6

    assert fibonacci(0) == 0
    assert fibonacci(1) == 1
    assert fibonacci(5) == 5
    assert fibonacci(7) == 13

    # Function with no return value
    no_return_value()

    # Using function results
    sum: int = add(add(1, 2), add(3, 4))
    assert sum == 10

    return 0
}
