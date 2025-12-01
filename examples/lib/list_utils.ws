# List utilities

def sum_list(numbers: list[int]) -> int {
    total: int = 0
    for num in numbers {
        total = total + num
    }
    return total
}

def max_in_list(numbers: list[int]) -> int {
    max: int = 0
    for num in numbers {
        if num > max {
            max = num
        }
    }
    return max
}

def count_evens(numbers: list[int]) -> int {
    count: int = 0
    for num in numbers {
        if num % 2 == 0 {
            count = count + 1
        }
    }
    return count
}
