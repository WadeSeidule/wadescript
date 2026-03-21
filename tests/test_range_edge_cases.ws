# Test: range() function edge cases

def main() -> int {
    # range(0) should produce empty list
    empty: list[int] = range(0)
    assert empty.length == 0

    # range(1) should produce [0]
    single: list[int] = range(1)
    assert single.length == 1
    assert single[0] == 0

    # range(2) should produce [0, 1]
    two: list[int] = range(2)
    assert two.length == 2
    assert two[0] == 0
    assert two[1] == 1

    # Iterate over range(0) - should execute 0 times
    count: int = 0
    for i in range(0) {
        count = count + 1
    }
    assert count == 0

    # Iterate over range(1) - should execute once
    count = 0
    for i in range(1) {
        count = count + 1
    }
    assert count == 1

    # Sum of range
    total: int = 0
    for i in range(10) {
        total = total + i
    }
    assert total == 45

    # range in nested loops
    pairs: int = 0
    for i in range(3) {
        for j in range(4) {
            pairs = pairs + 1
        }
    }
    assert pairs == 12

    # Use range for indexing
    nums: list[int] = [10, 20, 30, 40, 50]
    sum: int = 0
    for i in range(nums.length) {
        sum = sum + nums[i]
    }
    assert sum == 150

    return 0
}
