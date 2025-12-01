# Complete Lists Demo - All features working!

def main() -> int {
    print_str("=== WadeScript Lists - Complete Demo ===")
    print_str("")

    # Part 1: List literals
    print_str("Part 1: List Literals")
    numbers: list[int] = [10, 20, 30, 40, 50]
    print_str("Created list with 5 elements")
    print_int(numbers.length)
    print_str("")

    # Part 2: Index access
    print_str("Part 2: Index Access")
    print_str("First element (index 0):")
    print_int(numbers[0])
    print_str("Third element (index 2):")
    print_int(numbers[2])
    print_str("")

    # Part 3: For loop iteration
    print_str("Part 3: For Loop Iteration")
    print_str("All elements:")
    for num in numbers {
        print_int(num)
    }
    print_str("")

    # Part 4: List methods - push
    print_str("Part 4: Push Method")
    numbers.push(60)
    numbers.push(70)
    print_str("After pushing 60 and 70:")
    print_int(numbers.length)
    for n in numbers {
        print_int(n)
    }
    print_str("")

    # Part 5: Pop method
    print_str("Part 5: Pop Method")
    last: int = numbers.pop()
    print_str("Popped value:")
    print_int(last)
    print_str("Remaining length:")
    print_int(numbers.length)
    print_str("")

    # Part 6: Get method
    print_str("Part 6: Get Method")
    value: int = numbers.get(3)
    print_str("Element at index 3:")
    print_int(value)
    print_str("")

    # Part 7: range() function
    print_str("Part 7: range() Function")
    print_str("Numbers 0-4:")
    for i in range(5) {
        print_int(i)
    }
    print_str("")

    # Part 8: Building lists dynamically
    print_str("Part 8: Dynamic List Building")
    squares: list[int] = []
    for i in range(10) {
        sq: int = i * i
        squares.push(sq)
    }
    print_str("Squares of 0-9:")
    for s in squares {
        print_int(s)
    }
    print_str("")

    # Part 9: Computing with lists
    print_str("Part 9: Computing Sum")
    data: list[int] = [5, 10, 15, 20, 25]
    sum: int = 0
    for val in data {
        sum = sum + val
    }
    print_str("Sum of [5,10,15,20,25]:")
    print_int(sum)
    print_str("")

    # Part 10: Filtering
    print_str("Part 10: Filtering Values")
    evens: list[int] = []
    for i in range(20) {
        remainder: int = i % 2
        if remainder == 0 {
            evens.push(i)
        }
    }
    print_str("Even numbers 0-19:")
    for e in evens {
        print_int(e)
    }
    print_str("")

    print_str("=== All Features Working! ===")
    return 0
}
