# Test break and continue statements

def main() -> int {
    # Test break in while loop
    i: int = 0
    while i < 10 {
        if i == 5 {
            break
        }
        print_int(i)
        i = i + 1
    }

    # Test continue in while loop
    j: int = 0
    while j < 5 {
        j = j + 1
        if j == 3 {
            continue
        }
        print_int(j)
    }

    # Test break in for loop
    numbers: list[int] = [1, 2, 3, 4, 5, 6, 7, 8, 9, 10]
    for num in numbers {
        if num == 6 {
            break
        }
        print_int(num)
    }

    # Test continue in for loop
    for num in numbers {
        if num == 3 {
            continue
        }
        if num > 5 {
            break
        }
        print_int(num)
    }

    return 0
}
