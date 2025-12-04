# Test RC Phase 4b: Loop-Invariant Variable Detection
# Variables defined outside loops and only read inside are loop-invariant

# Test 1: Simple loop-invariant variable
def test_simple_loop_invariant() -> void {
    # Variable defined outside loop
    items: list[int] = [1, 2, 3, 4, 5]

    sum: int = 0
    for i in range(5) {
        # items is loop-invariant: only read, not assigned
        sum = sum + items.get(i)
    }
    assert sum == 15  # 1+2+3+4+5
}

# Test 2: Loop-invariant with nested loops
def test_nested_loop_invariant() -> void {
    # Outer scope variable
    data: list[int] = [10, 20, 30]

    total: int = 0
    for i in range(3) {
        for j in range(2) {
            # data is loop-invariant in both loops
            total = total + data.get(i)
        }
    }
    assert total == 120  # (10*2 + 20*2 + 30*2)
}

# Test 3: Mixed invariant and non-invariant
def test_mixed_invariant() -> void {
    # Loop-invariant
    multiplier: list[int] = [2, 3]

    # Not loop-invariant (assigned in loop)
    accumulator: list[int] = []

    for i in range(3) {
        # multiplier is loop-invariant (only read)
        # accumulator is not loop-invariant (modified)
        accumulator.push(i * multiplier.get(0))
    }

    assert accumulator.length == 3
    assert accumulator.get(0) == 0
    assert accumulator.get(1) == 2
    assert accumulator.get(2) == 4
}

# Test 4: Loop-invariant with dict
def test_dict_loop_invariant() -> void {
    # Dict defined outside loop
    cache: dict[str, int] = {"a": 100, "b": 200}

    sum: int = 0
    for i in range(3) {
        # cache is loop-invariant
        sum = sum + cache["a"]
    }
    assert sum == 300
}

# Test 5: Loop-invariant with method calls
def test_method_call_invariant() -> void {
    # Variable defined outside
    numbers: list[int] = [5, 10, 15, 20, 25]

    # Only pure method calls (get, length) - loop-invariant
    total: int = 0
    for i in range(numbers.length) {
        total = total + numbers.get(i)
    }
    assert total == 75  # 5+10+15+20+25
}

# Test 6: Non-invariant (assigned in loop)
def test_non_invariant() -> void {
    temp: list[int] = [1, 2, 3]

    for i in range(3) {
        # temp is NOT loop-invariant (reassigned)
        temp = [i, i * 2, i * 3]
    }

    assert temp.get(0) == 2
    assert temp.get(1) == 4
    assert temp.get(2) == 6
}

def main() -> int {
    test_simple_loop_invariant()
    test_nested_loop_invariant()
    test_mixed_invariant()
    test_dict_loop_invariant()
    test_method_call_invariant()
    test_non_invariant()
    return 0
}
