# Comprehensive test of string features
def main() -> int {
    # String concatenation
    first: str = "Hello"
    last: str = "World"
    full: str = first + " " + last
    print_str(full)

    # F-strings with variables
    name: str = "Bob"
    age: int = 25
    greeting: str = f"Hi, I'm {name} and I'm {age} years old"
    print_str(greeting)

    # Combining concatenation and f-strings
    prefix: str = "User:"
    suffix: str = "!"
    combined: str = prefix + " " + f"{name} ({age})" + suffix
    print_str(combined)

    # F-strings with expressions
    x: int = 10
    y: int = 20
    math_str: str = f"Sum of {x} and {y} is {x + y}"
    print_str(math_str)

    # Multiple concatenations
    a: str = "A"
    b: str = "B"
    c: str = "C"
    abc: str = a + b + c
    print_str(abc)

    # F-strings with floats
    pi: float = 3.14159
    circle_info: str = f"Pi is approximately {pi}"
    print_str(circle_info)

    # Nested in expressions
    result: str = "Result: " + f"{x * y}"
    print_str(result)

    return 0
}
