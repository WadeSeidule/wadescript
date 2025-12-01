# Test: List operations - literals, methods, indexing

def main() -> int {
    # Empty list
    empty: list[int] = []
    print_int(empty.length)  # 0

    # List literal with elements
    numbers: list[int] = [10, 20, 30, 40, 50]
    print_int(numbers.length)  # 5

    # Index access
    print_int(numbers[0])  # 10
    print_int(numbers[2])  # 30
    print_int(numbers[4])  # 50

    # Push method
    numbers.push(60)
    print_int(numbers.length)  # 6
    print_int(numbers[5])  # 60

    # Get method
    val: int = numbers.get(3)
    print_int(val)  # 40

    # Pop method
    last: int = numbers.pop()
    print_int(last)  # 60
    print_int(numbers.length)  # 5

    # Building list dynamically
    squares: list[int] = []
    for i in range(5) {
        sq: int = i * i
        squares.push(sq)
    }
    print_int(squares.length)  # 5
    print_int(squares[0])  # 0
    print_int(squares[4])  # 16

    # Iterate and sum
    sum: int = 0
    for s in squares {
        sum = sum + s
    }
    print_int(sum)  # 30 (0+1+4+9+16)

    return 0
}
