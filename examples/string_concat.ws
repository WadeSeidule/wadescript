# Test string concatenation
def main() -> int {
    first: str = "Hello"
    second: str = " World"
    result: str = first + second
    print_str(result)

    # Multiple concatenations
    greeting: str = "Hi" + " " + "there"
    print_str(greeting)

    # With literals
    message: str = "The answer is: " + "42"
    print_str(message)

    return 0
}
