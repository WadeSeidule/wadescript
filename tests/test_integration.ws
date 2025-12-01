# Test: Integration - Multiple features working together

def is_prime(n: int) -> bool {
    if n <= 1 {
        return False
    }
    if n == 2 {
        return True
    }

    i: int = 2
    while i * i <= n {
        if n % i == 0 {
            return False
        }
        i = i + 1
    }
    return True
}

def main() -> int {
    # Build list of primes using multiple features
    primes: list[int] = []

    # Use for loop with range to check numbers
    for num in range(20) {
        if is_prime(num) {
            primes.push(num)
        }
    }

    # Print count
    print_int(primes.length)  # 8

    # Print all primes
    for p in primes {
        print_int(p)  # 2 3 5 7 11 13 17 19
    }

    # Sum the primes
    sum: int = 0
    for p in primes {
        sum = sum + p
    }
    print_int(sum)  # 77

    # Test list operations
    last_prime: int = primes.pop()
    print_int(last_prime)  # 19
    print_int(primes.length)  # 7

    # Test indexing
    first: int = primes[0]
    second: int = primes[1]
    print_int(first + second)  # 5

    return 0
}
