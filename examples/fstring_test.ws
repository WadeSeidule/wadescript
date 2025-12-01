# Test f-strings
def main() -> int {
    name: str = "Alice"
    age: int = 30
    height: float = 5.8

    # Basic f-string
    message: str = f"Hello {name}!"
    print_str(message)

    # Multiple interpolations
    info: str = f"Name: {name}, Age: {age}"
    print_str(info)

    # With float
    stats: str = f"{name} is {height} feet tall"
    print_str(stats)

    # Expressions in f-strings
    result: str = f"Next year {name} will be {age + 1}"
    print_str(result)

    # Math expressions
    calc: str = f"10 + 20 = {10 + 20}"
    print_str(calc)

    return 0
}
