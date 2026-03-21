# Test: Dictionary iteration and .length property

def test_dict_length_empty() -> int {
    empty: dict[str, int] = {}
    assert empty.length == 0, "empty dict should have length 0"
    return 0
}

def test_dict_length_populated() -> int {
    scores: dict[str, int] = {"Math": 95, "English": 88, "Science": 92}
    assert scores.length == 3, "dict with 3 entries should have length 3"

    # Update existing key should not change length
    scores["Math"] = 100
    assert scores.length == 3, "updating key should not change length"

    # Adding new key increases length
    scores["History"] = 85
    assert scores.length == 4, "adding key should increase length"
    return 0
}

def test_dict_iteration_sum() -> int {
    scores: dict[str, int] = {"Math": 95, "English": 88, "Science": 92}
    total: int = 0
    for key in scores {
        total = total + scores[key]
    }
    assert total == 275, "sum of dict values should be 275"
    return 0
}

def test_dict_iteration_count() -> int {
    scores: dict[str, int] = {"a": 1, "b": 2, "c": 3}
    count: int = 0
    for key in scores {
        count = count + 1
    }
    assert count == 3, "iterating 3-key dict should yield 3 iterations"
    return 0
}

def test_dict_iteration_after_modify() -> int {
    data: dict[str, int] = {"x": 10, "y": 20}
    data["z"] = 30

    total: int = 0
    for key in data {
        total = total + data[key]
    }
    assert total == 60, "sum after adding key should be 60"
    return 0
}

def main() -> int {
    test_dict_length_empty()
    print_str("dict length empty: PASS")

    test_dict_length_populated()
    print_str("dict length populated: PASS")

    test_dict_iteration_sum()
    print_str("dict iteration sum: PASS")

    test_dict_iteration_count()
    print_str("dict iteration count: PASS")

    test_dict_iteration_after_modify()
    print_str("dict iteration after modify: PASS")

    print_str("All dict iteration tests passed!")
    return 0
}
