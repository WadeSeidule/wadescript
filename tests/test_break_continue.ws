# Test break and continue statements

def main() -> int {
    # Test break in while loop
    i: int = 0
    while i < 10 {
        if i == 5 {
            break
        }
        i = i + 1
    }
    assert i == 5

    # Test continue in while loop
    j: int = 0
    sum: int = 0
    while j < 5 {
        j = j + 1
        if j == 3 {
            continue
        }
        sum = sum + j
    }
    assert sum == 12  # 1 + 2 + 4 + 5 = 12

    # Test break in for loop
    numbers: list[int] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
    last: int = 0
    for num in numbers {
        if num == 6 {
            break
        }
        last = num
    }
    assert last == 5

    # Test continue in for loop
    total: int = 0
    for num in numbers {
        if num == 3 {
            continue
        }
        if num > 5 {
            break
        }
        total = total + num
    }
    assert total == 12  # 1 + 2 + 4 + 5 = 12

    return 0
}
