# Test last-use optimization for RC variables
# When assigning x = y and y is never used again, move instead of retain

def test_last_use_simple() -> void {
    a: list[int] = [1, 2, 3]
    b: list[int] = a  # OPTIMIZED: Last use of 'a', move instead of retain
    # 'a' is never used after this point
    assert b.get(0) == 1
    assert b.get(1) == 2
    assert b.get(2) == 3
}

def test_last_use_chain() -> void {
    x: list[int] = [10, 20, 30]
    y: list[int] = x  # OPTIMIZED: Last use of 'x'
    z: list[int] = y  # OPTIMIZED: Last use of 'y'
    # Only 'z' is used from here
    assert z.get(0) == 10
    assert z.get(1) == 20
    assert z.get(2) == 30
}

def test_no_optimization_reuse() -> void {
    items: list[int] = [100, 200]
    copy: list[int] = items  # NOT optimized - 'items' used again
    assert items.get(0) == 100  # 'items' still used here
    assert copy.get(1) == 200
}

def test_last_use_with_dict() -> void {
    data: dict[str, int] = {"value": 42}
    moved: dict[str, int] = data  # OPTIMIZED: Last use of 'data'
    assert moved["value"] == 42
}

def main() -> int {
    test_last_use_simple()
    test_last_use_chain()
    test_no_optimization_reuse()
    test_last_use_with_dict()
    return 0
}
