# Test dictionary updates
def main() -> int {
    # Create a dictionary with string keys and int values
    ages: dict[str, int] = {"Alice": 25, "Bob": 30}

    # Access initial values
    print_str("Initial ages:")
    print_str("Alice:")
    print_int(ages["Alice"])
    print_str("Bob:")
    print_int(ages["Bob"])

    # Update existing values
    ages["Alice"] = 26
    ages["Bob"] = 31

    # Access updated values
    print_str("After update:")
    print_str("Alice:")
    print_int(ages["Alice"])
    print_str("Bob:")
    print_int(ages["Bob"])

    # Add new key-value pair
    ages["Charlie"] = 35
    print_str("After adding Charlie:")
    print_str("Charlie:")
    print_int(ages["Charlie"])

    return 0
}
