# Fibonacci calculator
def fibonacci(n: int) -> int {
    if n <= 1 {
        return n
    }
    return fibonacci(n - 1) + fibonacci(n - 2)
}

def main() -> int {
    result: int = fibonacci(10)
    return result
}
