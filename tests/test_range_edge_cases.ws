# Test: range() function edge cases

def test_range_zero() -> int {
    empty: list[int] = range(0)
    assert empty.length == 0, "range(0) should produce empty list"

    count: int = 0
    for i in range(0) {
        count = count + 1
    }
    assert count == 0, "iterating range(0) should execute 0 times"
    return 0
}

def test_range_one() -> int {
    single: list[int] = range(1)
    assert single.length == 1, "range(1) should produce 1 element"
    assert single[0] == 0, "range(1)[0] should be 0"

    count: int = 0
    for i in range(1) {
        count = count + 1
    }
    assert count == 1, "iterating range(1) should execute once"
    return 0
}

def test_range_two() -> int {
    two: list[int] = range(2)
    assert two.length == 2, "range(2) should produce 2 elements"
    assert two[0] == 0, "range(2)[0] should be 0"
    assert two[1] == 1, "range(2)[1] should be 1"
    return 0
}

def test_range_sum() -> int {
    total: int = 0
    for i in range(10) {
        total = total + i
    }
    assert total == 45, "sum of range(10) should be 45"
    return 0
}

def test_range_nested() -> int {
    pairs: int = 0
    for i in range(3) {
        for j in range(4) {
            pairs = pairs + 1
        }
    }
    assert pairs == 12, "3x4 nested range should give 12"
    return 0
}

def test_range_for_indexing() -> int {
    nums: list[int] = [10, 20, 30, 40, 50]
    sum: int = 0
    for i in range(nums.length) {
        sum = sum + nums[i]
    }
    assert sum == 150, "indexing via range should sum to 150"
    return 0
}

def main() -> int {
    test_range_zero()
    print_str("range(0): PASS")

    test_range_one()
    print_str("range(1): PASS")

    test_range_two()
    print_str("range(2): PASS")

    test_range_sum()
    print_str("range sum: PASS")

    test_range_nested()
    print_str("range nested: PASS")

    test_range_for_indexing()
    print_str("range for indexing: PASS")

    print_str("All range edge case tests passed!")
    return 0
}
