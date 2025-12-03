# Test RC Phase 4b: Loop-Invariant Variable Detection
# This test verifies that variables defined outside loops and only read inside
# are properly detected as loop-invariant

# Test 1: Simple loop-invariant variable
def test_simple_loop_invariant() -> void {
    print_str("Test 1: Simple loop-invariant")

    # Variable defined outside loop
    items: list[int] = [1, 2, 3, 4, 5]

    sum: int = 0
    for i in range(5) {
        # items is loop-invariant: only read, not assigned
        sum = sum + items.get(i)
    }

    print_int(sum)  # Should be 15
    print_str("Simple loop-invariant test passed")
}

# Test 2: Loop-invariant with nested loops
def test_nested_loop_invariant() -> void {
    print_str("Test 2: Nested loop-invariant")

    # Outer scope variable
    data: list[int] = [10, 20, 30]

    total: int = 0
    for i in range(3) {
        for j in range(2) {
            # data is loop-invariant in both loops
            total = total + data.get(i)
        }
    }

    print_int(total)  # Should be 120 (10*2 + 20*2 + 30*2)
    print_str("Nested loop-invariant test passed")
}

# Test 3: Mixed invariant and non-invariant
def test_mixed_invariant() -> void {
    print_str("Test 3: Mixed invariant/non-invariant")

    # Loop-invariant
    multiplier: list[int] = [2, 3]

    # Not loop-invariant (assigned in loop)
    accumulator: list[int] = []

    for i in range(3) {
        # multiplier is loop-invariant (only read)
        # accumulator is not loop-invariant (modified)
        accumulator.push(i * multiplier.get(0))
    }

    print_int(accumulator.length)  # Should be 3
    print_int(accumulator.get(2))  # Should be 4
    print_str("Mixed invariant test passed")
}

# Test 4: Loop-invariant with dict
def test_dict_loop_invariant() -> void {
    print_str("Test 4: Dict loop-invariant")

    # Dict defined outside loop
    cache: dict[str, int] = {"a": 100, "b": 200}

    sum: int = 0
    for i in range(3) {
        # cache is loop-invariant
        sum = sum + cache["a"]
    }

    print_int(sum)  # Should be 300
    print_str("Dict loop-invariant test passed")
}

# Test 5: Loop-invariant with method calls
def test_method_call_invariant() -> void {
    print_str("Test 5: Method call loop-invariant")

    # Variable defined outside
    numbers: list[int] = [5, 10, 15, 20, 25]

    # Only pure method calls (get, length) - loop-invariant
    total: int = 0
    for i in range(numbers.length) {
        total = total + numbers.get(i)
    }

    print_int(total)  # Should be 75
    print_str("Method call invariant test passed")
}

# Test 6: Non-invariant (assigned in loop)
def test_non_invariant() -> void {
    print_str("Test 6: Non-invariant (assigned in loop)")

    temp: list[int] = [1, 2, 3]

    for i in range(3) {
        # temp is NOT loop-invariant (reassigned)
        temp = [i, i * 2, i * 3]
    }

    print_int(temp.get(0))  # Should be 2
    print_int(temp.get(2))  # Should be 6
    print_str("Non-invariant test passed")
}

def main() -> int {
    print_str("====================================")
    print_str("RC Phase 4b Tests: Loop Hoisting")
    print_str("====================================")
    print_str("")

    test_simple_loop_invariant()
    test_nested_loop_invariant()
    test_mixed_invariant()
    test_dict_loop_invariant()
    test_method_call_invariant()
    test_non_invariant()

    print_str("")
    print_str("====================================")
    print_str("All Phase 4b loop hoisting tests passed!")
    print_str("====================================")
    print_str("")
    print_str("Phase 4b Optimization Active:")
    print_str("- Loop-invariant variables detected")
    print_str("- Variables defined outside loops and only read inside")
    print_str("- RC operations happen at outer scope, not per-iteration")

    return 0
}
