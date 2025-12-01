# Test dictionaries
def main() -> int {
    # Create a dictionary with string keys and int values
    ages: dict[str, int] = {"Alice": 25, "Bob": 30, "Charlie": 35}

    # Access values by key
    alice_age: int = ages["Alice"]
    print_str("Alice's age:")
    print_int(alice_age)

    bob_age: int = ages["Bob"]
    print_str("Bob's age:")
    print_int(bob_age)

    charlie_age: int = ages["Charlie"]
    print_str("Charlie's age:")
    print_int(charlie_age)

    # Create an empty dict and add values
    scores: dict[str, int] = {}
    print_str("Created empty dict")

    return 0
}
