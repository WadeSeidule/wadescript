# Test nullable/optional types

def main() -> int {
    # Test str? syntax with None (compilation check)
    x: str? = None

    # Test Optional[str] syntax with None (compilation check)
    y: Optional[str] = None

    # Test assigning value to optional type
    z: str? = "hello"

    # Test int? syntax
    a: int? = None
    b: int? = 42

    # Test Optional[int] syntax
    c: Optional[int] = None
    d: Optional[int] = 100

    # Test list[int]? syntax with None
    items: list[int]? = None

    # Test assigning a real list to an optional
    nums: list[int]? = [1, 2, 3]

    # Test that non-None optional values work with properties
    assert nums.length == 3
    assert nums[0] == 1
    assert nums[1] == 2
    assert nums[2] == 3

    # Test reassignment from None to value
    items = [10, 20, 30]
    assert items.length == 3
    assert items[0] == 10

    # Test dict optional
    scores: dict[str, int]? = None
    scores = {"alice": 100, "bob": 90}
    assert scores["alice"] == 100
    assert scores["bob"] == 90

    return 0
}
