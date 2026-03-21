# Test: Floor division (//) operator

def test_basic_floor_div() -> int {
    assert 7 // 2 == 3, "7 // 2 should be 3"
    assert 10 // 3 == 3, "10 // 3 should be 3"
    assert 9 // 3 == 3, "9 // 3 should be 3"
    assert 1 // 1 == 1, "1 // 1 should be 1"
    assert 0 // 5 == 0, "0 // 5 should be 0"
    return 0
}

def test_larger_values() -> int {
    assert 100 // 7 == 14, "100 // 7 should be 14"
    assert 999 // 10 == 99, "999 // 10 should be 99"
    assert 1000 // 1000 == 1, "1000 // 1000 should be 1"
    return 0
}

def test_floor_div_variables() -> int {
    x: int = 17
    y: int = 5
    assert x // y == 3, "17 // 5 should be 3"

    z: int = 20
    z = z // 3
    assert z == 6, "20 // 3 should be 6"
    return 0
}

def test_floor_div_chained() -> int {
    result: int = 100 // 3 // 2
    assert result == 16, "100 // 3 // 2 should be 16"
    return 0
}

def test_floor_div_in_expression() -> int {
    a: int = 10
    b: int = 3
    result: int = (a // b) + 1
    assert result == 4, "(10 // 3) + 1 should be 4"
    return 0
}

def main() -> int {
    test_basic_floor_div()
    print_str("basic floor division: PASS")

    test_larger_values()
    print_str("larger values: PASS")

    test_floor_div_variables()
    print_str("floor div with variables: PASS")

    test_floor_div_chained()
    print_str("chained floor division: PASS")

    test_floor_div_in_expression()
    print_str("floor div in expression: PASS")

    print_str("All floor division tests passed!")
    return 0
}
