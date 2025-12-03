# Test RC Escape Analysis (Phase 3 Optimization)
# This test verifies that escape analysis correctly identifies non-escaping variables

# Non-escaping: Variable only used locally
def test_non_escaping() -> void {
    print_str("Test 1: Non-escaping variable")

    # This variable is only used locally, never escapes
    local_list: list[int] = [1, 2, 3]
    local_list.push(4)
    local_list.push(5)

    # Read from it
    val: int = local_list.get(0)
    print_int(val)

    # local_list should be optimized: no RC operations needed
    print_str("Non-escaping test passed")
}

# Escaping: Variable passed to function
def consume_list(items: list[int]) -> void {
    print_int(items.get(0))
}

def test_escaping_via_call() -> void {
    print_str("Test 2: Escaping via function call")

    # This variable escapes because it's passed to a function
    escaping_list: list[int] = [10, 20, 30]
    consume_list(escaping_list)

    # escaping_list needs RC operations (not optimized)
    print_str("Escaping via call test passed")
}

# Helper function for return test
def create_and_return() -> list[int] {
    # This variable escapes because it's returned
    items: list[int] = [100, 200, 300]
    return items
}

# Escaping: Variable returned from function
def test_escaping_via_return() -> void {
    print_str("Test 3: Escaping via return")

    # This is tested by the helper function
    result: list[int] = create_and_return()
    print_int(result.get(0))

    print_str("Escaping via return test passed")
}

# Non-escaping: Multiple local variables
def test_multiple_non_escaping() -> void {
    print_str("Test 4: Multiple non-escaping variables")

    # All of these are non-escaping
    list1: list[int] = [1, 2]
    list2: list[int] = [3, 4]
    list3: list[int] = [5, 6]

    list1.push(10)
    list2.push(20)
    list3.push(30)

    print_int(list1.get(0))
    print_int(list2.get(1))
    print_int(list3.get(2))

    print_str("Multiple non-escaping test passed")
}

# Non-escaping: Dict operations
def test_dict_non_escaping() -> void {
    print_str("Test 5: Non-escaping dict")

    # This dict doesn't escape
    scores: dict[str, int] = {"alice": 100, "bob": 90}
    val: int = scores["alice"]
    print_int(val)

    scores["charlie"] = 95

    print_str("Dict non-escaping test passed")
}

# Mixed: Some escape, some don't
def test_mixed() -> void {
    print_str("Test 6: Mixed escaping/non-escaping")

    # Non-escaping
    local: list[int] = [1, 2, 3]
    local.push(4)

    # Escaping (passed to function)
    escaping: list[int] = [5, 6, 7]
    consume_list(escaping)

    # Non-escaping
    another_local: list[int] = [8, 9, 10]
    print_int(another_local.get(1))

    print_str("Mixed test passed")
}

# Control flow: Non-escaping in branches
def test_control_flow() -> void {
    print_str("Test 7: Non-escaping in control flow")

    condition: bool = True

    if condition {
        # Non-escaping in this branch
        branch_list: list[int] = [1, 2]
        branch_list.push(3)
        print_int(branch_list.get(0))
    }

    # Non-escaping in loop
    for i in range(3) {
        loop_list: list[int] = [i, i * 2]
        print_int(loop_list.get(1))
    }

    print_str("Control flow test passed")
}

def main() -> int {
    print_str("====================================")
    print_str("RC Escape Analysis Tests (Phase 3)")
    print_str("====================================")
    print_str("")

    test_non_escaping()
    test_escaping_via_call()
    test_escaping_via_return()
    test_multiple_non_escaping()
    test_dict_non_escaping()
    test_mixed()
    test_control_flow()

    print_str("")
    print_str("====================================")
    print_str("All escape analysis tests passed!")
    print_str("====================================")

    return 0
}
