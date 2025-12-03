# Minimal dictionary showcase test

def demonstrate_dictionaries() -> void {
    print_str("=== Dictionary Features ===")

    # Create dictionary
    scores: dict[str, int] = {}

    # Add entries
    scores["Alice"] = 95
    scores["Bob"] = 87
    scores["Charlie"] = 92

    # Read entries
    alice_score: int = scores["Alice"]
    bob_score: int = scores["Bob"]

    print_str(f"Alice's score: {alice_score}")
    print_str(f"Bob's score: {bob_score}")

    print_str("")
}

def main() -> int {
    print_str("Testing dictionaries...")
    demonstrate_dictionaries()
    print_str("Done!")
    return 0
}
