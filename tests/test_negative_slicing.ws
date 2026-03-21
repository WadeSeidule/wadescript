# Test: Negative indices and step slicing

def test_list_reverse() -> int {
    nums: list[int] = [10, 20, 30, 40, 50]
    rev: list[int] = nums[::-1]
    assert rev.length == 5, "reversed list should have same length"
    assert rev[0] == 50, "first element of reversed should be last"
    assert rev[1] == 40, "second element of reversed"
    assert rev[4] == 10, "last element of reversed should be first"
    return 0
}

def test_list_step() -> int {
    nums: list[int] = [10, 20, 30, 40, 50]
    evens: list[int] = nums[::2]
    assert evens.length == 3, "step 2 on 5 elements gives 3"
    assert evens[0] == 10, "first stepped element"
    assert evens[1] == 30, "second stepped element"
    assert evens[2] == 50, "third stepped element"
    return 0
}

def test_list_subslice() -> int {
    nums: list[int] = [10, 20, 30, 40, 50]
    sub: list[int] = nums[1:4]
    assert sub.length == 3, "subslice [1:4] has 3 elements"
    assert sub[0] == 20, "subslice first"
    assert sub[2] == 40, "subslice last"
    return 0
}

def test_list_empty_slice() -> int {
    nums: list[int] = [10, 20, 30, 40, 50]
    empty: list[int] = nums[3:1]
    assert empty.length == 0, "start > end with positive step gives empty"
    return 0
}

def test_string_reverse() -> int {
    hello: str = "hello world"
    rev: str = hello[::-1]
    assert rev == "dlrow olleh", "reversed string"
    return 0
}

def test_string_step() -> int {
    hello: str = "hello world"
    skip: str = hello[::2]
    assert skip == "hlowrd", "string with step 2"
    return 0
}

def test_string_subslice() -> int {
    hello: str = "hello world"
    sub: str = hello[0:5]
    assert sub == "hello", "string subslice [0:5]"

    world: str = hello[6:11]
    assert world == "world", "string subslice [6:11]"
    return 0
}

def main() -> int {
    test_list_reverse()
    print_str("list reverse: PASS")

    test_list_step()
    print_str("list step: PASS")

    test_list_subslice()
    print_str("list subslice: PASS")

    test_list_empty_slice()
    print_str("list empty slice: PASS")

    test_string_reverse()
    print_str("string reverse: PASS")

    test_string_step()
    print_str("string step: PASS")

    test_string_subslice()
    print_str("string subslice: PASS")

    print_str("All negative slicing tests passed!")
    return 0
}
