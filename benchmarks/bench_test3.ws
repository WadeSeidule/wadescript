# Test with high iteration count

def test1() -> void {
    print_str("Test starting")
    iterations: int = 100000

    for i in range(iterations) {
        temp: list[int] = [1, 2, 3]
    }

    print_str("Test done")
}

def main() -> int {
    test1()
    return 0
}
