# Benchmark Phase 4: Pure Function Optimization
# Measures impact of recognizing pure functions don't cause escape

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

# Phase 4 Optimized: Lists with pure function calls
# Variables should NOT escape because they're only passed to pure functions
def bench_list_pure_ops() -> void {
    for i in range(25000) {
        # Phase 4: All operations are pure - no escape!
        items: list[int] = [1, 2, 3]
        val: int = items.get(0)
        val = items.get(1)
        len: int = items.length
    }
    print_str("[PHASE 4 OPTIMIZED] List + pure ops: 25K iterations")
}

# Phase 4 Optimized: Dict with pure operations
def bench_dict_pure_ops() -> void {
    for i in range(15000) {
        # Phase 4: Only pure operations - no escape!
        data: dict[str, int] = {"key": 100}
        val: int = data["key"]
    }
    print_str("[PHASE 4 OPTIMIZED] Dict + pure ops: 15K iterations")
}

# Phase 4 Optimized: Lists with method calls (pure methods)
def bench_list_method_chain() -> void {
    for i in range(20000) {
        # Phase 4: Method calls on pure methods don't cause escape
        numbers: list[int] = [10, 20, 30]
        a: int = numbers.get(0)
        b: int = numbers.get(1)
        c: int = numbers.get(2)
        sum: int = a + b + c
    }
    print_str("[PHASE 4 OPTIMIZED] List method chain: 20K iterations")
}

# Phase 4 Optimized: Complex pure operations
def bench_complex_pure() -> void {
    for i in range(15000) {
        # Multiple lists, all with pure operations
        a: list[int] = [1, 2]
        b: list[int] = [3, 4]
        c: list[int] = [5, 6]

        # All pure operations
        sum: int = 0
        sum = sum + a.get(0)
        sum = sum + b.get(1)
        sum = sum + c.get(0)
        len: int = a.length
    }
    print_str("[PHASE 4 OPTIMIZED] Complex pure: 15K iterations")
}

# Comparison: Variables that DO escape (not optimized)
def consumer(items: list[int]) -> void {
    val: int = items.get(0)
}

def bench_escaping_calls() -> void {
    for i in range(10000) {
        # This escapes - passed to user function
        items: list[int] = [1, 2, 3]
        consumer(items)  # Not pure - causes escape
    }
    print_str("[NOT OPTIMIZED] Escaping calls: 10K iterations")
}

# Phase 4 Optimized: Print with pure operations
def bench_print_with_pure() -> void {
    sum: int = 0
    for i in range(20000) {
        # Phase 4: Print is pure, list ops are pure
        data: list[int] = [i, i * 2]
        sum = sum + data.length  # Pure operation - no escape!
    }
    print_str("[PHASE 4 OPTIMIZED] Print + pure: 20K iterations")
}

def main() -> int {
    print_str("================================================")
    print_str("Phase 4 Pure Function Optimization Benchmark")
    print_str("================================================")
    print_str("")
    print_str("Measuring impact of pure function analysis")
    print_str("")

    bench_baseline()
    print_str("")

    bench_list_pure_ops()
    bench_dict_pure_ops()
    bench_list_method_chain()
    bench_complex_pure()
    bench_print_with_pure()
    print_str("")

    bench_escaping_calls()
    print_str("")

    print_str("================================================")
    print_str("Benchmark Complete")
    print_str("================================================")
    print_str("")
    print_str("Phase 4a Optimizations Applied:")
    print_str("- Pure functions (list.get, dict.get, print, etc.)")
    print_str("- Variables passed only to pure functions: no escape")
    print_str("- Zero RC overhead for pure function patterns")
    print_str("")
    print_str("Expected improvement: Additional 10-20% reduction")
    print_str("for code with many pure function calls")
    print_str("")
    print_str("Use: /usr/bin/time -p ./bench_phase4_pure")
    print_str("to measure wall-clock time")

    return 0
}
