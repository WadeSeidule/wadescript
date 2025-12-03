# Benchmark Phase 4b: Loop Hoisting for RC Operations
# Measures impact of loop-invariant variable detection

# Baseline: Integer operations (no RC)
def bench_baseline() -> void {
    sum: int = 0
    for i in range(50000) {
        x: int = 42
        y: int = x
        sum = sum + y
    }
    print_str("[BASELINE] Int ops: 50K iterations")
}

# Phase 4b Optimized: Loop-invariant list (read-only in loop)
def bench_loop_invariant_list() -> void {
    items: list[int] = [1, 2, 3, 4, 5]

    sum: int = 0
    # items is loop-invariant: defined outside, only read inside
    # RC operations happen at outer scope, not per-iteration
    for i in range(10000) {
        sum = sum + items.get(0)
    }
    print_str("[PHASE 4B OPTIMIZED] Loop-invariant list: 10K iterations")
}

# Phase 4b Optimized: Loop-invariant dict
def bench_loop_invariant_dict() -> void {
    cache: dict[str, int] = {"key": 100}

    sum: int = 0
    # cache is loop-invariant: defined outside, only read inside
    for i in range(8000) {
        sum = sum + cache["key"]
    }
    print_str("[PHASE 4B OPTIMIZED] Loop-invariant dict: 8K iterations")
}

# Phase 4b Optimized: Nested loops with invariant variable
def bench_nested_loop_invariant() -> void {
    data: list[int] = [10, 20, 30, 40, 50]

    total: int = 0
    # data is loop-invariant in both nested loops
    for i in range(100) {
        for j in range(5) {
            total = total + data.get(j)
        }
    }
    print_str("[PHASE 4B OPTIMIZED] Nested loop-invariant: 100x5 iterations")
}

# Phase 4b Optimized: Loop-invariant with pure methods
def bench_loop_invariant_methods() -> void {
    numbers: list[int] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]

    sum: int = 0
    len: int = numbers.length  # Pure method - loop-invariant
    # numbers is loop-invariant, only pure methods called
    for i in range(5000) {
        if len > 0 {
            sum = sum + numbers.get(0)
        }
    }
    print_str("[PHASE 4B OPTIMIZED] Loop-invariant with methods: 5K iterations")
}

# Phase 4b Optimized: Multiple loop-invariant variables
def bench_multiple_invariants() -> void {
    a: list[int] = [1, 2]
    b: list[int] = [3, 4]
    c: list[int] = [5, 6]

    sum: int = 0
    # All three are loop-invariant
    for i in range(5000) {
        sum = sum + a.get(0)
        sum = sum + b.get(0)
        sum = sum + c.get(0)
    }
    print_str("[PHASE 4B OPTIMIZED] Multiple invariants: 5K iterations")
}

# Comparison: Variable defined in loop (not invariant)
def bench_in_loop_variable() -> void {
    sum: int = 0

    # Variable created each iteration - not loop-invariant
    for i in range(5000) {
        temp: list[int] = [i, i * 2]
        sum = sum + temp.get(0)
    }
    print_str("[NOT OPTIMIZED] In-loop variable: 5K iterations")
}

# Comparison: Variable modified in loop (not invariant)
def bench_modified_in_loop() -> void {
    temp: list[int] = [1, 2, 3]

    # temp is modified (reassigned) in loop - not loop-invariant
    for i in range(3000) {
        if i % 2 == 0 {
            temp = [i, i + 1, i + 2]
        }
    }
    print_str("[NOT OPTIMIZED] Modified in loop: 3K iterations")
}

def main() -> int {
    print_str("================================================")
    print_str("Phase 4b Loop Hoisting Optimization Benchmark")
    print_str("================================================")
    print_str("")
    print_str("Measuring impact of loop-invariant detection")
    print_str("")

    bench_baseline()
    print_str("")

    bench_loop_invariant_list()
    bench_loop_invariant_dict()
    bench_nested_loop_invariant()
    bench_loop_invariant_methods()
    bench_multiple_invariants()
    print_str("")

    bench_in_loop_variable()
    bench_modified_in_loop()
    print_str("")

    print_str("================================================")
    print_str("Benchmark Complete")
    print_str("================================================")
    print_str("")
    print_str("Phase 4b Optimizations Applied:")
    print_str("- Loop-invariant variable detection")
    print_str("- Variables defined outside loops + only read inside")
    print_str("- RC operations at outer scope, not per-iteration")
    print_str("")
    print_str("Combined with Phase 3 (escape analysis) and")
    print_str("Phase 4a (pure functions), loop-invariant variables")
    print_str("achieve optimal RC performance")
    print_str("")
    print_str("Expected improvement: 5-15% in loop-heavy code")
    print_str("")
    print_str("Use: /usr/bin/time -p ./bench_phase4b_loop_hoisting")
    print_str("to measure wall-clock time")

    return 0
}
