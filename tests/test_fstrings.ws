# Test: F-string formatting

def test_fstring_variable() -> int {
    name: str = "Alice"
    greeting: str = f"Hello, {name}!"
    assert greeting == "Hello, Alice!", "f-string should interpolate variable"
    return 0
}

def test_fstring_int() -> int {
    age: int = 30
    s: str = f"Age: {age}"
    assert s == "Age: 30", "f-string should interpolate int"
    return 0
}

def test_fstring_expression() -> int {
    x: int = 10
    y: int = 20
    result: str = f"Sum of {x} and {y} is {x + y}"
    assert result == "Sum of 10 and 20 is 30", "f-string should evaluate expressions"
    return 0
}

def test_fstring_float() -> int {
    pi: float = 3.14
    s: str = f"Pi is {pi}"
    assert s == "Pi is 3.14", "f-string should interpolate float"
    return 0
}

def test_fstring_bool() -> int {
    flag: bool = True
    s: str = f"Value is {flag}"
    assert s == "Value is True", "f-string should interpolate bool"
    return 0
}

def test_fstring_multiple() -> int {
    a: int = 1
    b: int = 2
    c: int = 3
    s: str = f"{a}, {b}, {c}"
    assert s == "1, 2, 3", "f-string should handle multiple interpolations"
    return 0
}

def test_fstring_no_interpolation() -> int {
    s: str = f"no interpolation here"
    assert s == "no interpolation here", "f-string with no braces should work"
    return 0
}

def test_fstring_concat() -> int {
    prefix: str = "Result: "
    val: int = 42
    combined: str = prefix + f"{val}"
    assert combined == "Result: 42", "f-string concatenated with regular string"
    return 0
}

def main() -> int {
    test_fstring_variable()
    print_str("fstring variable: PASS")

    test_fstring_int()
    print_str("fstring int: PASS")

    test_fstring_expression()
    print_str("fstring expression: PASS")

    test_fstring_float()
    print_str("fstring float: PASS")

    test_fstring_bool()
    print_str("fstring bool: PASS")

    test_fstring_multiple()
    print_str("fstring multiple: PASS")

    test_fstring_no_interpolation()
    print_str("fstring no interpolation: PASS")

    test_fstring_concat()
    print_str("fstring concat: PASS")

    print_str("All f-string tests passed!")
    return 0
}
