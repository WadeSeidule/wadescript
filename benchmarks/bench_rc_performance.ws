# Benchmark RC overhead with realistic iterations

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

# RC Test 1: List creation and destruction
def bench_list_create() -> void {
    for i in range(10000) {
        temp: list[int] = [1, 2, 3, 4, 5]
    }  # Released at end of scope

    print_str("[RC] List create/destroy: 10K iterations")
}

# RC Test 2: List assignments with move optimization
def bench_list_move() -> void {
    for i in range(10000) {
        a: list[int] = [1, 2, 3]
        b: list[int] = a  # OPTIMIZED: Last use, move
    }

    print_str("[RC OPTIMIZED] List move: 10K iterations")
}

# RC Test 3: Function returns with move semantics
def create_list() -> list[int] {
    items: list[int] = [1, 2, 3]
    return items  # OPTIMIZED: Move semantics
}

def bench_function_returns() -> void {
    for i in range(10000) {
        temp: list[int] = create_list()
    }

    print_str("[RC OPTIMIZED] Function returns: 10K iterations")
}

# RC Test 4: Dict operations
def bench_dict_create() -> void {
    for i in range(5000) {
        data: dict[str, int] = {"key": 100}
        copy: dict[str, int] = data  # OPTIMIZED: Last use
    }

    print_str("[RC OPTIMIZED] Dict create/move: 5K iterations")
}

# RC Test 5: List with operations
def bench_list_ops() -> void {
    for i in range(5000) {
        items: list[int] = [1, 2, 3]
        items.push(4)
        items.push(5)
        val: int = items.get(0)
    }

    print_str("[RC] List with operations: 5K iterations")
}

# RC Test 6: Reassignment pattern
def bench_reassignment() -> void {
    for i in range(10000) {
        x: list[int] = [1, 2, 3]
        x = [4, 5, 6]  # Release old, assign new
    }

    print_str("[RC] Reassignments: 10K iterations")
}

def main() -> int {
    print_str("====================================")
    print_str("WadeScript RC Performance Benchmark")
    print_str("====================================")
    print_str("")

    bench_baseline()
    bench_list_create()
    bench_list_move()
    bench_function_returns()
    bench_dict_create()
    bench_list_ops()
    bench_reassignment()

    print_str("")
    print_str("====================================")
    print_str("Benchmark Complete")
    print_str("====================================")
    print_str("")
    print_str("Optimizations Active:")
    print_str("- Move semantics for function returns")
    print_str("- Last-use analysis for assignments")
    print_str("- Inline RC operations (no function calls)")

    return 0
}
