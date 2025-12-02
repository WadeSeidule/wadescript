# Test assert statement

def main() -> int {
    # Test passing assertion
    x: int = 5
    assert x == 5
    print_int(1)  # Should print

    # Test with message
    assert x > 0, "x should be positive"
    print_int(2)  # Should print

    # Test with complex condition
    assert x < 10 and x > 0
    print_int(3)  # Should print

    return 0
}
