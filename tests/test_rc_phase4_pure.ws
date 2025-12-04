# Test RC Phase 4: Pure Function Escape Analysis
# Variables passed to pure functions don't escape

# Test 1: List passed to pure functions (list.length, list.get)
def test_list_pure_functions() -> void {
    # This list is passed to pure functions only
    items: list[int] = [1, 2, 3, 4, 5]

    # All these are pure functions - don't cause escape
    assert items.length == 5
    assert items.get(0) == 1
    assert items.get(2) == 3
    assert items.get(4) == 5
}

# Test 2: Dict passed to pure functions
def test_dict_pure_functions() -> void {
    # This dict is passed to pure functions only
    scores: dict[str, int] = {"alice": 100, "bob": 90}

    # Pure functions - don't cause escape
    assert scores["alice"] == 100
    assert scores["bob"] == 90
}

# Test 3: List with mixed pure and impure
def consume_list(items: list[int]) -> int {
    return items.get(0)
}

def test_mixed_pure_impure() -> void {
    # Pure operations
    local: list[int] = [1, 2, 3]
    assert local.get(0) == 1      # Pure
    assert local.length == 3       # Pure

    # Impure (passed to user function - escapes)
    escaping: list[int] = [5, 6, 7]
    result: int = consume_list(escaping)
    assert result == 5
}

# Test 4: In-place modifications are pure for escape analysis
def test_inplace_modifications() -> void {
    # These operations modify in-place but don't cause escape
    items: list[int] = [1, 2, 3]

    items.push(4)  # Pure for escape (modifies in place)
    items.push(5)

    assert items.length == 5
    assert items.get(3) == 4
    assert items.get(4) == 5
}

# Test 5: Dict in-place modifications
def test_dict_inplace() -> void {
    data: dict[str, int] = {"x": 10}

    data["y"] = 20  # Pure for escape (modifies in place)
    data["z"] = 30

    assert data["x"] == 10
    assert data["y"] == 20
    assert data["z"] == 30
}

# Test 6: Pure functions in loops
def test_pure_in_loop() -> void {
    items: list[int] = [10, 20, 30, 40, 50]

    sum: int = 0
    for i in range(items.length) {
        # Both .length and .get are pure
        sum = sum + items.get(i)
    }
    assert sum == 150
}

# Test 7: Multiple pure calls
def test_multiple_pure_calls() -> void {
    data: list[int] = [5, 10, 15]

    # Multiple pure calls on same variable
    a: int = data.get(0)
    b: int = data.get(1)
    c: int = data.get(2)
    len: int = data.length

    assert a == 5
    assert b == 10
    assert c == 15
    assert len == 3
    assert a + b + c == 30
}

def main() -> int {
    test_list_pure_functions()
    test_dict_pure_functions()
    test_mixed_pure_impure()
    test_inplace_modifications()
    test_dict_inplace()
    test_pure_in_loop()
    test_multiple_pure_calls()
    return 0
}
