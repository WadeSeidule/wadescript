# Test: str() and print() functions

def main() -> int {
    # Test str() with int
    s1: str = str(42)
    assert s1 == "42"

    # Test str() with negative int
    s2: str = str(-123)
    assert s2 == "-123"

    # Test str() with zero
    s3: str = str(0)
    assert s3 == "0"

    # Test str() with bool
    s4: str = str(True)
    assert s4 == "True"
    s5: str = str(False)
    assert s5 == "False"

    # Test str() with str (identity)
    s6: str = str("hello")
    assert s6 == "hello"

    # Test str() with list
    nums: list[int] = [1, 2, 3]
    s7: str = str(nums)
    assert s7 == "[1, 2, 3]"

    # Test str() with empty list
    empty_list: list[int] = []
    s8: str = str(empty_list)
    assert s8 == "[]"

    # Test str() with single element list
    single: list[int] = [42]
    s9: str = str(single)
    assert s9 == "[42]"

    # Test print() with various types (visual verification)
    print(42)
    print(-1)
    print(True)
    print(False)
    print("hello world")
    print(nums)
    print(empty_list)

    print_str("All str/print tests passed!")
    return 0
}
