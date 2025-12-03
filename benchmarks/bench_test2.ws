# Test with function parameters

def pass_through(items: list[int]) -> list[int] {
    return items
}

def test1() -> void {
    print_str("Test 1")
    for i in range(10) {
        base: list[int] = [1, 2, 3]
        result: list[int] = pass_through(base)
    }
    print_str("Test 1 done")
}

def main() -> int {
    print_str("Starting")
    test1()
    print_str("Done")
    return 0
}
