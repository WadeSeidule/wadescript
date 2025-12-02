# Test compound assignment operators

def main() -> int {
    # Test += operator
    x: int = 10
    x += 5
    print_int(x)  # Should print 15

    # Test -= operator
    y: int = 20
    y -= 8
    print_int(y)  # Should print 12

    # Test *= operator
    z: int = 5
    z *= 4
    print_int(z)  # Should print 20

    # Test /= operator
    w: int = 100
    w /= 5
    print_int(w)  # Should print 20

    # Test with lists - TODO: requires list_set
    # numbers: list[int] = [1, 2, 3, 4, 5]
    # numbers[2] += 10
    # print_int(numbers[2])  # Should print 13

    # Multiple operations
    a: int = 5
    a += 3
    a *= 2
    a -= 4
    print_int(a)  # Should print 12

    return 0
}
