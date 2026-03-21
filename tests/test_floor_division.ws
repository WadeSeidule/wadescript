# Test: Floor division (//) operator

def main() -> int {
    # Basic floor division
    assert 7 // 2 == 3
    assert 10 // 3 == 3
    assert 9 // 3 == 3
    assert 1 // 1 == 1
    assert 0 // 5 == 0

    # Larger values
    assert 100 // 7 == 14
    assert 999 // 10 == 99

    # Floor division in expressions
    x: int = 17
    y: int = 5
    assert x // y == 3

    # Floor division with compound assignment
    z: int = 20
    z = z // 3
    assert z == 6

    # Floor division vs regular division
    a: int = 7
    b: int = 2
    assert a // b == 3
    # Regular division would give 3.5 for floats

    # Chained arithmetic with floor division
    result: int = 100 // 3 // 2
    assert result == 16

    return 0
}
