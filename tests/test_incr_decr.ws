# Test increment and decrement operators

def main() -> int {
    # Test ++ operator
    x: int = 5
    x++
    assert x == 6
    x++
    assert x == 7

    # Test -- operator
    y: int = 10
    y--
    assert y == 9
    y--
    assert y == 8

    # Test in a while loop
    count: int = 0
    while count < 5 {
        count++
    }
    assert count == 5

    # Test decrement in while loop
    z: int = 3
    sum: int = 0
    while z > 0 {
        sum = sum + z
        z--
    }
    assert sum == 6  # 3 + 2 + 1 = 6
    assert z == 0

    return 0
}
