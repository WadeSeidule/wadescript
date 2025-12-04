# Test: String methods and iteration

def main() -> int {
    # Test string length property
    s: str = "hello"
    assert s.length == 5

    # Test empty string length
    empty: str = ""
    assert empty.length == 0

    # Test longer string
    long: str = "hello world"
    assert long.length == 11

    # Test string upper() method
    upper: str = s.upper()
    assert upper.length == 5

    # Test string lower() method
    lower: str = "WORLD".lower()
    assert lower.length == 5

    # Test string contains() method
    text: str = "hello world"
    assert text.contains("world")
    assert text.contains("hello")
    assert text.contains("lo wo")
    assert not text.contains("foo")
    assert not text.contains("xyz")

    # Test string iteration count
    test_str: str = "abc"
    count: int = 0
    for char in test_str {
        count = count + 1
    }
    assert count == 3

    # Test iteration over string literal
    count = 0
    for c in "xyz" {
        count = count + 1
    }
    assert count == 3

    return 0
}
