# Loop examples
def sum_to_n(n: int) -> int {
    sum: int = 0
    i: int = 1

    while i <= n {
        sum = sum + i
        i = i + 1
    }

    return sum
}

def main() -> int {
    result: int = sum_to_n(100)
    return result
}
