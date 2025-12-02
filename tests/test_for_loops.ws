# Test: For loops with range()

def main() -> int {
    # Basic for loop with range
    sum: int = 0
    for i in range(5) {
        sum = sum + i
    }
    assert sum == 10  # 0+1+2+3+4

    # For loop with list
    numbers: list[int] = [10, 20, 30]
    total: int = 0
    for num in numbers {
        total = total + num
    }
    assert total == 60

    # Nested for loops
    product: int = 0
    for i in range(3) {
        for j in range(3) {
            product = product + 1
        }
    }
    assert product == 9  # 3 * 3

    # Building a list in a loop
    squares: list[int] = []
    for i in range(4) {
        squares.push(i * i)
    }
    assert squares.length == 4
    assert squares[0] == 0
    assert squares[1] == 1
    assert squares[2] == 4
    assert squares[3] == 9

    return 0
}
