# Comprehensive example showing various WadeScript features

def is_prime(n: int) -> bool {
    if n <= 1 {
        return False
    }
    if n <= 3 {
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

def count_primes(limit: int) -> int {
    count: int = 0
    i: int = 2

    while i < limit {
        if is_prime(i) {
            count = count + 1
        }
        i = i + 1
    }

    return count
}

def power(base: int, exp: int) -> int {
    if exp == 0 {
        return 1
    }
    result: int = base
    i: int = 1
    while i < exp {
        result = result * base
        i = i + 1
    }
    return result
}

def main() -> int {
    # Count primes less than 20
    prime_count: int = count_primes(20)

    # Calculate 2^5
    pow_result: int = power(2, 5)

    # Return the sum
    return prime_count + pow_result
}
