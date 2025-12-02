# Test: List operations - literals, methods, indexing

def main() -> int {
    # Empty list
    empty: list[int] = []
    assert empty.length == 0

    # List literal with elements
    numbers: list[int] = [10, 20, 30, 40, 50]
    assert numbers.length == 5

    # Index access
    assert numbers[0] == 10
    assert numbers[2] == 30
    assert numbers[4] == 50

    # Push method
    numbers.push(60)
    assert numbers.length == 6
    assert numbers[5] == 60

    # Get method
    val: int = numbers.get(3)
    assert val == 40

    # Pop method
    last: int = numbers.pop()
    assert last == 60
    assert numbers.length == 5

    # Building list dynamically
    squares: list[int] = []
    for i in range(5) {
        sq: int = i * i
        squares.push(sq)
    }
    assert squares.length == 5
    assert squares[0] == 0
    assert squares[1] == 1
    assert squares[2] == 4
    assert squares[3] == 9
    assert squares[4] == 16

    # Iterate and sum
    sum: int = 0
    for s in squares {
        sum = sum + s
    }
    assert sum == 30  # 0+1+4+9+16

    return 0
}
