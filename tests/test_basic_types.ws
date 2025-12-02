# Test: Basic types and operations

def main() -> int {
    # Integer operations
    a: int = 10
    b: int = 5
    assert a + b == 15
    assert a - b == 5
    assert a * b == 50
    assert a / b == 2
    assert a % b == 0

    # Float operations
    x: float = 3.5
    y: float = 2.0
    assert x + y == 5.5
    assert x * y == 7.0

    # Boolean operations
    assert True == True
    assert False == False
    assert 10 > 5
    assert not (10 < 5)

    # More integer tests
    assert 100 / 10 == 10
    assert 17 % 5 == 2

    return 0
}
