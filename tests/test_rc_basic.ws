# Test that RC-allocated lists work
def main() -> int {
    items: list[int] = [1, 2, 3]
    items.push(4)
    print_int(items.get(0))
    print_int(items.get(3))
    return 0
}
