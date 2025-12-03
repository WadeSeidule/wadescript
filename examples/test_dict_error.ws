# Test dictionary key not found error

def main() -> int {
    scores: dict[str, int] = {}
    scores["Alice"] = 95
    scores["Bob"] = 87

    print_str("Getting Alice's score:")
    alice_score: int = scores["Alice"]
    print_int(alice_score)

    print_str("Getting non-existent key:")
    missing_score: int = scores["Charlie"]  # This should trigger error
    print_int(missing_score)

    return 0
}
