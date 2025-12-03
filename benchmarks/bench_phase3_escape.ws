# Benchmark Phase 3: Escape Analysis Impact
# Compares performance of non-escaping vs escaping RC operations

# Baseline: Integer operations (no RC)
def bench_baseline() -> void {
    sum: int = 0
    for i in range(50000) {
        x: int = 42
        y: int = x
        z: int = y
        sum = sum + z
    }
    print_str("[BASELINE] Int ops: 50K iterations")
}

# Phase 3 Optimized: Non-escaping list operations
# These should have ZERO RC overhead (no retain/release)
def bench_non_escaping_lists() -> void {
    for i in range(20000) {
        # All these variables are non-escaping
        temp: list[int] = [1, 2, 3]
        temp.push(4)
        temp.push(5)
        val: int = temp.get(0)
    }
    print_str("[PHASE 3 OPTIMIZED] Non-escaping lists: 20K iterations")
}

# Phase 3 Optimized: Non-escaping dicts
# These should have ZERO RC overhead
def bench_non_escaping_dicts() -> void {
    for i in range(10000) {
        # Non-escaping dict
        data: dict[str, int] = {"key": 100}
        val: int = data["key"]
        data["key2"] = 200
    }
    print_str("[PHASE 3 OPTIMIZED] Non-escaping dicts: 10K iterations")
}

# Phase 3 Optimized: Multiple non-escaping variables
# All should have ZERO RC overhead
def bench_multiple_non_escaping() -> void {
    for i in range(15000) {
        # All non-escaping
        a: list[int] = [1, 2]
        b: list[int] = [3, 4]
        c: list[int] = [5, 6]

        a.push(10)
        b.push(20)
        c.push(30)
    }
    print_str("[PHASE 3 OPTIMIZED] Multiple non-escaping: 15K iterations")
}

# NOT Optimized: Escaping via function call
# These still need RC operations
def consume_list(items: list[int]) -> void {
    val: int = items.get(0)
}

def bench_escaping_via_call() -> void {
    for i in range(10000) {
        # Escapes via function call
        data: list[int] = [1, 2, 3]
        consume_list(data)
    }
    print_str("[NOT OPTIMIZED] Escaping via call: 10K iterations")
}

# Comparison: Non-escaping in loop body
# This pattern is very common and benefits greatly from Phase 3
def bench_loop_local_pattern() -> void {
    for i in range(20000) {
        # Classic "local temporary" pattern
        temp: list[int] = [i, i * 2, i * 3]
        sum: int = temp.get(0)
        sum = sum + temp.get(1)
        sum = sum + temp.get(2)
    }
    print_str("[PHASE 3 OPTIMIZED] Loop-local pattern: 20K iterations")
}

# Phase 3 Optimized: Nested blocks
def bench_nested_non_escaping() -> void {
    for i in range(10000) {
        condition: bool = True
        if condition {
            # Non-escaping in nested block
            block_list: list[int] = [1, 2, 3]
            block_list.push(4)
        }
    }
    print_str("[PHASE 3 OPTIMIZED] Nested blocks: 10K iterations")
}

def main() -> int {
    print_str("================================================")
    print_str("Phase 3 Escape Analysis Benchmark")
    print_str("================================================")
    print_str("")
    print_str("Measuring impact of escape analysis on RC overhead")
    print_str("")

    bench_baseline()
    print_str("")

    bench_non_escaping_lists()
    bench_non_escaping_dicts()
    bench_multiple_non_escaping()
    bench_loop_local_pattern()
    bench_nested_non_escaping()
    print_str("")

    bench_escaping_via_call()
    print_str("")

    print_str("================================================")
    print_str("Benchmark Complete")
    print_str("================================================")
    print_str("")
    print_str("Phase 3 Optimizations Applied:")
    print_str("- Non-escaping variables: ZERO RC overhead")
    print_str("- Escaping variables: Full RC (as before)")
    print_str("")
    print_str("Expected improvement: ~20-30% reduction in RC overhead")
    print_str("for code with many local temporaries")
    print_str("")
    print_str("Use: /usr/bin/time -p ./bench_phase3_escape")
    print_str("to measure wall-clock time")

    return 0
}
