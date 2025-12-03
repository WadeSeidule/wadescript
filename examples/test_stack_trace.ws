# Test stack trace functionality
# This should show nested function calls when error occurs

def level3() -> int {
    print_str("In level3, accessing out of bounds")
    numbers: list[int] = [1, 2, 3]
    # This should trigger error with stack trace showing: main -> level1 -> level2 -> level3
    bad_val: int = numbers[10]
    return bad_val
}

def level2() -> int {
    print_str("In level2, calling level3")
    return level3()
}

def level1() -> int {
    print_str("In level1, calling level2")
    return level2()
}

def main() -> int {
    print_str("In main, calling level1")
    result: int = level1()
    print_int(result)
    return 0
}
