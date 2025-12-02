# Test increment and decrement operators

def main() -> int {
    # Test ++ operator
    x: int = 5
    x++
    print_int(x)  # Should print 6
    x++
    print_int(x)  # Should print 7

    # Test -- operator
    y: int = 10
    y--
    print_int(y)  # Should print 9
    y--
    print_int(y)  # Should print 8

    # Test in a loop
    count: int = 0
    while count < 5 {
        print_int(count)
        count++
    }

    # Test with for loop using --
    z: int = 3
    while z > 0 {
        print_int(z)
        z--
    }

    return 0
}
