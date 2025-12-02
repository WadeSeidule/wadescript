# Test: Integration test combining multiple features

def sum_list(numbers: list[int]) -> int {
    total: int = 0
    for num in numbers {
        total = total + num
    }
    return total
}

def main() -> int {
    # Lists and functions
    nums: list[int] = [1, 2, 3, 4, 5]
    assert sum_list(nums) == 15

    # Dictionaries and loops
    scores: dict[str, int] = {"Alice": 90, "Bob": 85, "Charlie": 95}
    assert scores["Alice"] == 90
    assert scores["Bob"] == 85
    assert scores["Charlie"] == 95

    # Complex logic
    evens: list[int] = []
    for i in range(10) {
        if i % 2 == 0 {
            evens.push(i)
        }
    }
    assert evens.length == 5
    assert sum_list(evens) == 20  # 0+2+4+6+8

    # Build a dict from list
    lookup: dict[str, int] = {}
    lookup["zero"] = 0
    lookup["one"] = 1
    lookup["two"] = 2
    assert lookup["zero"] == 0
    assert lookup["one"] == 1
    assert lookup["two"] == 2

    return 0
}
