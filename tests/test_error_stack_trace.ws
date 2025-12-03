# Test stack trace in error messages

def inner() -> int {
    nums: list[int] = [1, 2, 3]
    # Error on line 5
    return nums[99]
}

def middle() -> int {
    return inner()
}

def outer() -> int {
    return middle()
}

def main() -> int {
    return outer()
}
