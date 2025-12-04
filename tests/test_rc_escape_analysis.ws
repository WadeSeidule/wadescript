# Test RC Escape Analysis (Phase 3 Optimization)
# This test verifies that escape analysis correctly identifies non-escaping variables

# Non-escaping: Variable only used locally
def test_non_escaping() -> void {
    # This variable is only used locally, never escapes
    local_list: list[int] = [1, 2, 3]
    local_list.push(4)
    local_list.push(5)

    assert local_list.get(0) == 1
    assert local_list.get(4) == 5
    assert local_list.length == 5
}

# Escaping: Variable passed to function
def consume_list(items: list[int]) -> int {
    return items.get(0)
}

def test_escaping_via_call() -> void {
    # This variable escapes because it's passed to a function
    escaping_list: list[int] = [10, 20, 30]
    result: int = consume_list(escaping_list)
    assert result == 10
}

# Helper function for return test
def create_and_return() -> list[int] {
    # This variable escapes because it's returned
    items: list[int] = [100, 200, 300]
    return items
}

# Escaping: Variable returned from function
def test_escaping_via_return() -> void {
    result: list[int] = create_and_return()
    assert result.get(0) == 100
    assert result.get(2) == 300
}

# Non-escaping: Multiple local variables
def test_multiple_non_escaping() -> void {
    # All of these are non-escaping
    list1: list[int] = [1, 2]
    list2: list[int] = [3, 4]
    list3: list[int] = [5, 6]

    list1.push(10)
    list2.push(20)
    list3.push(30)

    assert list1.get(0) == 1
    assert list2.get(1) == 4
    assert list3.get(2) == 30
}

# Non-escaping: Dict operations
def test_dict_non_escaping() -> void {
    # This dict doesn't escape
    scores: dict[str, int] = {"alice": 100, "bob": 90}
    assert scores["alice"] == 100

    scores["charlie"] = 95
    assert scores["charlie"] == 95
}

# Mixed: Some escape, some don't
def test_mixed() -> void {
    # Non-escaping
    local: list[int] = [1, 2, 3]
    local.push(4)

    # Escaping (passed to function)
    escaping: list[int] = [5, 6, 7]
    result: int = consume_list(escaping)

    # Non-escaping
    another_local: list[int] = [8, 9, 10]

    assert local.length == 4
    assert result == 5
    assert another_local.get(1) == 9
}

# Control flow: Non-escaping in branches
def test_control_flow() -> void {
    condition: bool = True

    if condition {
        # Non-escaping in this branch
        branch_list: list[int] = [1, 2]
        branch_list.push(3)
        assert branch_list.get(0) == 1
    }

    # Non-escaping in loop
    total: int = 0
    for i in range(3) {
        loop_list: list[int] = [i, i * 2]
        total = total + loop_list.get(1)
    }
    assert total == 6  # 0 + 2 + 4
}

def main() -> int {
    test_non_escaping()
    test_escaping_via_call()
    test_escaping_via_return()
    test_multiple_non_escaping()
    test_dict_non_escaping()
    test_mixed()
    test_control_flow()
    return 0
}
