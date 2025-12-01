# Test: For loops with lists and range

def main() -> int {
    # For loop with list literal
    numbers: list[int] = [1, 2, 3, 4, 5]
    for num in numbers {
        print_int(num)  # 1 2 3 4 5
    }

    # For loop with range
    for i in range(5) {
        print_int(i)  # 0 1 2 3 4
    }

    # Computing sum with for loop
    sum: int = 0
    for n in numbers {
        sum = sum + n
    }
    print_int(sum)  # 15

    # For loop with range computation
    total: int = 0
    for i in range(10) {
        total = total + i
    }
    print_int(total)  # 45

    # Empty list iteration
    empty: list[int] = []
    for item in empty {
        print_int(item)  # Should not print
    }
    print_str("done")  # done

    return 0
}
