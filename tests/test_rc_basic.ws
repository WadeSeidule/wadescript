# Test that RC-allocated lists work
def main() -> int {
    items: list[int] = [1, 2, 3]
    items.push(4)
    assert items.get(0) == 1
    assert items.get(1) == 2
    assert items.get(2) == 3
    assert items.get(3) == 4
    assert items.length == 4
    return 0
}
