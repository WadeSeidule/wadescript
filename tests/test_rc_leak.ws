# Test RC memory management - no leaks
# This test allocates many lists and they should all be freed

def create_and_discard() -> int {
    # Create a list that goes out of scope immediately
    temp: list[int] = [1, 2, 3, 4, 5]
    temp.push(6)
    return temp.length
}

def reassign_test() -> int {
    # Test that reassignment releases the old value
    x: list[int] = [1, 2, 3]
    len1: int = x.length
    x = [4, 5, 6]  # Old list should be freed
    len2: int = x.length
    x = [7, 8, 9]  # Previous list should be freed
    return len1 + len2 + x.length
}

def main() -> int {
    # Allocate many lists in a loop
    # All should be freed when they go out of scope
    for i in range(100) {
        result: int = create_and_discard()
        assert result == 6
    }

    for i in range(100) {
        result: int = reassign_test()
        assert result == 9  # 3 + 3 + 3
    }

    # Test assignment chain
    for i in range(100) {
        a: list[int] = [1, 2, 3]
        b: list[int] = a  # a ref_count = 2
        c: list[int] = b  # a ref_count = 3
        assert c.length == 3
    }  # All released properly

    return 0
}
