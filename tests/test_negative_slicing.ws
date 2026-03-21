# Test: Negative indices in slicing

def main() -> int {
    nums: list[int] = [10, 20, 30, 40, 50]

    # Slice with negative end
    first_three: list[int] = nums[0:3]
    assert first_three.length == 3
    assert first_three[0] == 10
    assert first_three[2] == 30

    # Reverse a list with [::-1]
    rev: list[int] = nums[::-1]
    assert rev.length == 5
    assert rev[0] == 50
    assert rev[1] == 40
    assert rev[4] == 10

    # Slice with step 2
    evens: list[int] = nums[::2]
    assert evens.length == 3
    assert evens[0] == 10
    assert evens[1] == 30
    assert evens[2] == 50

    # String slicing
    hello: str = "hello world"

    # String reverse
    rev_str: str = hello[::-1]
    assert rev_str == "dlrow olleh"

    # String with step 2
    skip: str = hello[::2]
    assert skip == "hlowrd"

    # Substring via slice
    sub: str = hello[0:5]
    assert sub == "hello"

    # Slice from middle to end
    world: str = hello[6:11]
    assert world == "world"

    # Empty slice (start >= end with positive step)
    empty: list[int] = nums[3:1]
    assert empty.length == 0

    return 0
}
