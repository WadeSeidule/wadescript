# Test: F-string formatting

def main() -> int {
    # Basic variable interpolation
    name: str = "Alice"
    greeting: str = f"Hello, {name}!"
    assert greeting == "Hello, Alice!"

    # Integer interpolation
    age: int = 30
    info: str = f"{name} is {age} years old"
    assert info == "Alice is 30 years old"

    # Expression interpolation
    x: int = 10
    y: int = 20
    result: str = f"Sum of {x} and {y} is {x + y}"
    assert result == "Sum of 10 and 20 is 30"

    # Float interpolation
    pi: float = 3.14
    circle: str = f"Pi is {pi}"
    assert circle == "Pi is 3.14"

    # Boolean interpolation
    flag: bool = True
    bool_str: str = f"Value is {flag}"
    assert bool_str == "Value is True"

    # Empty f-string with no interpolation
    plain: str = f"no interpolation here"
    assert plain == "no interpolation here"

    # Multiple interpolations in one string
    a: int = 1
    b: int = 2
    c: int = 3
    multi: str = f"{a}, {b}, {c}"
    assert multi == "1, 2, 3"

    # F-string concatenation with regular strings
    prefix: str = "Result: "
    val: int = 42
    combined: str = prefix + f"{val}"
    assert combined == "Result: 42"

    return 0
}
