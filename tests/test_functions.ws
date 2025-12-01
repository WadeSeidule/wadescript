# Test: Functions and recursion

def add(a: int, b: int) -> int {
    return a + b
}

def multiply(a: int, b: int) -> int {
    return a * b
}

def factorial(n: int) -> int {
    if n <= 1 {
        return 1
    }
    return n * factorial(n - 1)
}

def fibonacci(n: int) -> int {
    if n <= 1 {
        return n
    }
    return fibonacci(n - 1) + fibonacci(n - 2)
}

def main() -> int {
    # Simple function calls
    print_int(add(10, 5))         # 15
    print_int(multiply(6, 7))     # 42

    # Recursion
    print_int(factorial(5))       # 120
    print_int(fibonacci(7))       # 13

    return 0
}
