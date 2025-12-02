# Test: Control flow (if/elif/else, while)

def main() -> int {
    # Simple if
    x: int = 10
    if x > 5 {
        x = x + 1
    }
    assert x == 11

    # If-else
    y: int = 3
    if y > 5 {
        y = 0
    } else {
        y = y + 10
    }
    assert y == 13

    # If-elif-else
    z: int = 5
    result: int = 0
    if z < 3 {
        result = 1
    } elif z < 7 {
        result = 2
    } else {
        result = 3
    }
    assert result == 2

    # While loop
    count: int = 0
    sum: int = 0
    while count < 5 {
        sum = sum + count
        count = count + 1
    }
    assert sum == 10  # 0+1+2+3+4
    assert count == 5

    # Nested if
    a: int = 10
    b: int = 20
    if a > 5 {
        if b > 15 {
            a = a + b
        }
    }
    assert a == 30

    return 0
}
