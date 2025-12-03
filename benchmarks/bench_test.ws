# Test to isolate segfault

def create_small_list() -> list[int] {
    items: list[int] = [1, 2, 3]
    return items
}

def test1() -> void {
    print_str("Test 1")
    for i in range(10) {
        temp: list[int] = create_small_list()
    }
    print_str("Test 1 done")
}

def main() -> int {
    print_str("Starting")
    test1()
    print_str("Done")
    return 0
}
