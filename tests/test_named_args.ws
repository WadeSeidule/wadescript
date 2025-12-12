# Test named arguments and default parameters

def greet(name: str = "World", excited: bool = False) -> str {
    if excited {
        return f"Hello, {name}!"
    } else {
        return f"Hello, {name}"
    }
}

def add(a: int, b: int = 10, c: int = 100) -> int {
    return a + b + c
}

def test_defaults_only() -> int {
    # Call with all defaults
    result: str = greet()
    assert result == "Hello, World", "greet() should use all defaults"
    return 0
}

def test_positional_override() -> int {
    # Override first param positionally
    result: str = greet("Alice")
    assert result == "Hello, Alice", "greet('Alice') should use positional"
    return 0
}

def test_named_first_param() -> int {
    # Use named argument for first param
    result: str = greet(name="Bob")
    assert result == "Hello, Bob", "greet(name='Bob') should work"
    return 0
}

def test_named_second_param() -> int {
    # Skip first param with default, provide second param
    result: str = greet(excited=True)
    assert result == "Hello, World!", "greet(excited=True) should use default name"
    return 0
}

def test_both_named() -> int {
    # Both params as named args
    result: str = greet(name="Charlie", excited=True)
    assert result == "Hello, Charlie!", "greet(name='Charlie', excited=True) should work"
    return 0
}

def test_named_reversed_order() -> int {
    # Named args in different order
    result: str = greet(excited=True, name="Dana")
    assert result == "Hello, Dana!", "Named args should work in any order"
    return 0
}

def test_mixed_positional_and_named() -> int {
    # Mix positional and named
    result: str = greet("Eve", excited=True)
    assert result == "Hello, Eve!", "Mixed positional and named should work"
    return 0
}

def test_add_defaults() -> int {
    # Three params with defaults
    r1: int = add(1)  # Should use b=10, c=100 -> 111
    assert r1 == 111, "add(1) should be 111"

    r2: int = add(1, 2)  # Should use c=100 -> 103
    assert r2 == 103, "add(1, 2) should be 103"

    r3: int = add(1, 2, 3)  # All provided -> 6
    assert r3 == 6, "add(1, 2, 3) should be 6"

    return 0
}

def test_add_named() -> int {
    # Use named args with add
    r1: int = add(a=5)  # b=10, c=100 -> 115
    assert r1 == 115, "add(a=5) should be 115"

    r2: int = add(a=5, c=50)  # b=10 -> 65
    assert r2 == 65, "add(a=5, c=50) should be 65"

    r3: int = add(1, c=5)  # b=10 -> 16
    assert r3 == 16, "add(1, c=5) should be 16"

    return 0
}

def main() -> int {
    test_defaults_only()
    print_str("defaults only: PASS")

    test_positional_override()
    print_str("positional override: PASS")

    test_named_first_param()
    print_str("named first param: PASS")

    test_named_second_param()
    print_str("named second param: PASS")

    test_both_named()
    print_str("both named: PASS")

    test_named_reversed_order()
    print_str("named reversed order: PASS")

    test_mixed_positional_and_named()
    print_str("mixed positional and named: PASS")

    test_add_defaults()
    print_str("add defaults: PASS")

    test_add_named()
    print_str("add named: PASS")

    print_str("All named args tests passed!")
    return 0
}
