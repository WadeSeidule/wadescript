# Test: String methods and iteration

def main() -> int {
    # Test string length property
    s: str = "hello"
    len: int = s.length
    print_int(len)
    assert len == 5

    # Test string upper() method
    upper: str = s.upper()
    print_str(upper)  # Should print "HELLO"

    # Test string lower() method
    lower: str = "WORLD".lower()
    print_str(lower)  # Should print "world"

    # Test string contains() method
    text: str = "hello world"
    has_world: bool = text.contains("world")
    has_hello: bool = text.contains("hello")
    has_foo: bool = text.contains("foo")
    assert has_world
    assert has_hello
    assert not has_foo

    # Test string iteration
    test_str: str = "abc"
    for char in test_str {
        print_str(char)
    }

    # Test iteration over string literal
    for c in "xyz" {
        print_str(c)
    }

    return 0
}
