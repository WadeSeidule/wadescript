def main() -> int {
    # Array creation with literal
    nums: int[5] = [10, 20, 30, 40, 50]

    # Index access
    assert nums[0] == 10
    assert nums[1] == 20
    assert nums[4] == 50

    # Index assignment
    nums[2] = 99
    assert nums[2] == 99

    # Float array
    vals: float[3] = [1.5, 2.5, 3.5]
    assert vals[0] == 1.5
    assert vals[2] == 3.5

    vals[1] = 9.9
    assert vals[1] == 9.9

    return 0
}
