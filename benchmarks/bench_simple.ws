# Simple benchmark to test RC overhead

def test_baseline() -> void {
    iterations: int = 1000
    sum: int = 0

    for i in range(iterations) {
        x: int = 42
        y: int = x
        sum = sum + y
    }

    print_str("Baseline complete")
}

def test_lists() -> void {
    iterations: int = 1000

    for i in range(iterations) {
        temp: list[int] = [1, 2, 3]
    }

    print_str("Lists complete")
}

def main() -> int {
    print_str("Simple benchmark starting")
    test_baseline()
    test_lists()
    print_str("Benchmark done")
    return 0
}
