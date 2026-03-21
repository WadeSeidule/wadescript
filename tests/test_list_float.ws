def main() -> int {
    # Float list creation and literals
    nums: list[float] = [1.5, 2.7, 3.14]
    assert nums.length == 3

    # Index access
    assert nums[0] == 1.5
    assert nums[1] == 2.7
    assert nums[2] == 3.14

    # Push and pop
    nums.push(4.0)
    assert nums.length == 4
    assert nums[3] == 4.0

    val: float = nums.pop()
    assert val == 4.0
    assert nums.length == 3

    # Index assignment
    nums[1] = 99.9
    assert nums[1] == 99.9

    # Get method
    x: float = nums.get(0)
    assert x == 1.5

    # Empty float list
    empty: list[float] = []
    assert empty.length == 0
    empty.push(42.0)
    assert empty.length == 1
    assert empty[0] == 42.0

    # For loop iteration
    total: float = 0.0
    values: list[float] = [1.0, 2.0, 3.0, 4.0]
    for v in values {
        total = total + v
    }
    assert total == 10.0

    # Slicing
    all: list[float] = [10.0, 20.0, 30.0, 40.0, 50.0]
    sub: list[float] = all[1:4]
    assert sub.length == 3
    assert sub[0] == 20.0
    assert sub[2] == 40.0

    # str() conversion
    small: list[float] = [1.0, 2.5]
    s: str = str(small)
    print(s)

    return 0
}
