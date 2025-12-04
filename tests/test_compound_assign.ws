# Test compound assignment operators

def main() -> int {
    # Test += operator
    x: int = 10
    x += 5
    assert x == 15

    # Test -= operator
    y: int = 20
    y -= 8
    assert y == 12

    # Test *= operator
    z: int = 5
    z *= 4
    assert z == 20

    # Test /= operator
    w: int = 100
    w /= 5
    assert w == 20

    # Multiple operations
    a: int = 5
    a += 3
    assert a == 8
    a *= 2
    assert a == 16
    a -= 4
    assert a == 12

    return 0
}
