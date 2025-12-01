# Factorial calculator
def factorial(n: int) -> int {
    if n <= 1 {
        return 1
    }
    return n * factorial(n - 1)
}

def main() -> int {
    result: int = factorial(5)
    return result
}
