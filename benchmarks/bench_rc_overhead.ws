# Benchmark RC overhead across different operations
# Tests various scenarios to measure performance impact

# Baseline: Integer operations (no RC)
def bench_int_ops() -> void {
    iterations: int = 1000000
    sum: int = 0

    for i in range(iterations) {
        x: int = 42
        y: int = x
        z: int = y
        sum = sum + z
    }

    print_str("Baseline int ops: 1M iterations")
}

# RC Test 1: List creation and destruction
def bench_list_create_destroy() -> void {
    iterations: int = 100000

    for i in range(iterations) {
        temp: list[int] = [1, 2, 3, 4, 5]
    }  # Released at end of scope

    print_str("List create/destroy: 100K iterations")
}

# RC Test 2: List assignments with move optimization
def bench_list_assignments_optimized() -> void {
    iterations: int = 100000

    for i in range(iterations) {
        a: list[int] = [1, 2, 3]
        b: list[int] = a  # OPTIMIZED: Last use, move
    }

    print_str("List assignments (optimized): 100K iterations")
}

# RC Test 3: List assignments without optimization (reuse)
def bench_list_assignments_unoptimized() -> void {
    iterations: int = 100000

    for i in range(iterations) {
        a: list[int] = [1, 2, 3]
        b: list[int] = a  # NOT optimized - 'a' used again
        c: int = a.get(0)  # 'a' reused, so retain/release needed
    }

    print_str("List assignments (with reuse): 100K iterations")
}

# RC Test 4: Function returns with move semantics
def create_small_list() -> list[int] {
    items: list[int] = [1, 2, 3]
    return items  # OPTIMIZED: Move semantics
}

def bench_function_returns() -> void {
    iterations: int = 100000

    for i in range(iterations) {
        temp: list[int] = create_small_list()
    }

    print_str("Function returns (move): 100K iterations")
}

# RC Test 5: Nested function calls
def pass_through_a(items: list[int]) -> list[int] {
    return items
}

def pass_through_b(items: list[int]) -> list[int] {
    return pass_through_a(items)
}

def bench_nested_calls() -> void {
    iterations: int = 50000

    for i in range(iterations) {
        base: list[int] = [10, 20, 30]
        result: list[int] = pass_through_b(base)
    }

    print_str("Nested function calls: 50K iterations")
}

# RC Test 6: Dict operations
def bench_dict_ops() -> void {
    iterations: int = 50000

    for i in range(iterations) {
        data: dict[str, int] = {"key": 100}
        copy: dict[str, int] = data  # OPTIMIZED: Last use
    }

    print_str("Dict create/assign: 50K iterations")
}

# RC Test 7: List with operations
def bench_list_with_ops() -> void {
    iterations: int = 50000

    for i in range(iterations) {
        items: list[int] = [1, 2, 3]
        items.push(4)
        items.push(5)
        val: int = items.get(0)
    }

    print_str("List with operations: 50K iterations")
}

# RC Test 8: Reassignment pattern
def bench_reassignments() -> void {
    iterations: int = 100000

    for i in range(iterations) {
        x: list[int] = [1, 2, 3]
        x = [4, 5, 6]  # Release old, assign new
        x = [7, 8, 9]  # Release old, assign new
    }

    print_str("Reassignments: 100K iterations")
}

def main() -> int {
    print_str("=== WadeScript RC Performance Benchmark ===")
    print_str("")

    print_str("[1] Baseline (no RC):")
    bench_int_ops()
    print_str("")

    print_str("[2] List Create/Destroy:")
    bench_list_create_destroy()
    print_str("")

    print_str("[3] Assignments (Optimized - Move):")
    bench_list_assignments_optimized()
    print_str("")

    print_str("[4] Assignments (Unoptimized - Reuse):")
    bench_list_assignments_unoptimized()
    print_str("")

    print_str("[5] Function Returns (Move Semantics):")
    bench_function_returns()
    print_str("")

    print_str("[6] Nested Function Calls:")
    bench_nested_calls()
    print_str("")

    print_str("[7] Dict Operations:")
    bench_dict_ops()
    print_str("")

    print_str("[8] List with Operations:")
    bench_list_with_ops()
    print_str("")

    print_str("[9] Reassignments:")
    bench_reassignments()
    print_str("")

    print_str("=== Benchmark Complete ===")
    print_str("")
    print_str("Notes:")
    print_str("- Tests 3 & 5 use move optimizations (minimal RC ops)")
    print_str("- Test 4 shows unoptimized case (full retain/release)")
    print_str("- Compare runtime vs baseline to measure overhead")

    return 0
}
