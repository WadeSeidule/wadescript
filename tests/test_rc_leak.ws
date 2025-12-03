# Test RC memory management - no leaks
# This test allocates many lists and they should all be freed

def create_and_discard() -> void {
    # Create a list that goes out of scope immediately
    temp: list[int] = [1, 2, 3, 4, 5]
    temp.push(6)
}

def reassign_test() -> void {
    # Test that reassignment releases the old value
    x: list[int] = [1, 2, 3]
    x = [4, 5, 6]  # Old list should be freed
    x = [7, 8, 9]  # Previous list should be freed
}

def main() -> int {
    # Allocate many lists in a loop
    # All should be freed when they go out of scope
    for i in range(1000) {
        create_and_discard()
    }

    for i in range(1000) {
        reassign_test()
    }

    # Test assignment chain
    for i in range(1000) {
        a: list[int] = [1, 2, 3]
        b: list[int] = a  # a ref_count = 2
        c: list[int] = b  # a ref_count = 3
    }  # All released: c released (count=2), b released (count=1), a released (count=0, freed)

    print_str("RC test completed - check for leaks with memory tools")
    return 0
}
