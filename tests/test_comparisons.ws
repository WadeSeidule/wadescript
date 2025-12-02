# Test: Comparison operators

def main() -> int {
    # Integer comparisons
    assert 5 == 5
    assert not (5 == 6)
    assert 5 != 6
    assert not (5 != 5)
    assert 3 < 5
    assert not (5 < 3)
    assert 5 > 3
    assert not (3 > 5)
    assert 5 <= 5
    assert 3 <= 5
    assert not (6 <= 5)
    assert 5 >= 5
    assert 5 >= 3
    assert not (3 >= 5)

    # Float comparisons
    assert 3.5 > 2.0
    assert 2.0 < 3.5
    assert 3.5 >= 3.5
    assert 3.5 <= 3.5

    # Boolean comparisons
    assert True == True
    assert False == False
    assert True != False

    # Logical operators
    assert True and True
    assert not (True and False)
    assert not (False and True)
    assert not (False and False)
    assert True or False
    assert False or True
    assert True or True
    assert not (False or False)

    # Complex expressions
    assert (5 > 3) and (10 < 20)
    assert (5 < 3) or (10 < 20)
    assert not (5 < 3)

    return 0
}
