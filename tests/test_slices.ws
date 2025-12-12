# Test slice syntax for lists and strings

def test_list_slice_basic() -> int {
    nums: list[int] = [0, 1, 2, 3, 4, 5]

    # Slice from index 1 to 4
    sub: list[int] = nums[1:4]
    assert sub.length == 3, "sub should have 3 elements"
    assert sub[0] == 1, "sub[0] should be 1"
    assert sub[1] == 2, "sub[1] should be 2"
    assert sub[2] == 3, "sub[2] should be 3"

    return 0
}

def test_list_slice_from_start() -> int {
    nums: list[int] = [0, 1, 2, 3, 4, 5]

    # Slice from beginning to index 3
    first3: list[int] = nums[:3]
    assert first3.length == 3, "first3 should have 3 elements"
    assert first3[0] == 0, "first3[0] should be 0"
    assert first3[1] == 1, "first3[1] should be 1"
    assert first3[2] == 2, "first3[2] should be 2"

    return 0
}

def test_list_slice_to_end() -> int {
    nums: list[int] = [0, 1, 2, 3, 4, 5]

    # Slice from index 3 to end
    last3: list[int] = nums[3:]
    assert last3.length == 3, "last3 should have 3 elements"
    assert last3[0] == 3, "last3[0] should be 3"
    assert last3[1] == 4, "last3[1] should be 4"
    assert last3[2] == 5, "last3[2] should be 5"

    return 0
}

def test_list_slice_with_step() -> int {
    nums: list[int] = [0, 1, 2, 3, 4, 5]

    # Every second element
    every2: list[int] = nums[::2]
    assert every2.length == 3, "every2 should have 3 elements"
    assert every2[0] == 0, "every2[0] should be 0"
    assert every2[1] == 2, "every2[1] should be 2"
    assert every2[2] == 4, "every2[2] should be 4"

    return 0
}

def test_list_slice_copy() -> int {
    nums: list[int] = [0, 1, 2, 3, 4, 5]

    # Full copy
    copy: list[int] = nums[:]
    assert copy.length == 6, "copy should have 6 elements"

    return 0
}

def test_string_slice_basic() -> int {
    s: str = "hello world"

    # Slice first 5 characters
    hello: str = s[:5]
    assert hello == "hello", "should be 'hello'"

    # Slice from index 6
    world: str = s[6:]
    assert world == "world", "should be 'world'"

    return 0
}

def test_string_slice_middle() -> int {
    s: str = "hello world"

    # Slice middle
    lo_wo: str = s[3:8]
    assert lo_wo == "lo wo", "should be 'lo wo'"

    return 0
}

def main() -> int {
    test_list_slice_basic()
    print_str("list slice basic: PASS")

    test_list_slice_from_start()
    print_str("list slice from start: PASS")

    test_list_slice_to_end()
    print_str("list slice to end: PASS")

    test_list_slice_with_step()
    print_str("list slice with step: PASS")

    test_list_slice_copy()
    print_str("list slice copy: PASS")

    test_string_slice_basic()
    print_str("string slice basic: PASS")

    test_string_slice_middle()
    print_str("string slice middle: PASS")

    print_str("All slice tests passed!")
    return 0
}
