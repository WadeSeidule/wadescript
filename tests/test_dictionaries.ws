# Test: Dictionary operations with hash table

def main() -> int {
    # Create dictionary with initial values
    ages: dict[str, int] = {"Alice": 25, "Bob": 30, "Charlie": 35}

    # Test basic access
    print_int(ages["Alice"])
    print_int(ages["Bob"])
    print_int(ages["Charlie"])

    # Create empty dictionary
    scores: dict[str, int] = {}

    # Add values
    scores["Math"] = 95
    scores["English"] = 88
    scores["Science"] = 92

    # Test access
    print_int(scores["Math"])
    print_int(scores["English"])
    print_int(scores["Science"])

    # Update existing values
    scores["Math"] = 98
    print_int(scores["Math"])

    # Test with many entries (triggers rehashing)
    data: dict[str, int] = {}
    data["k1"] = 1
    data["k2"] = 2
    data["k3"] = 3
    data["k4"] = 4
    data["k5"] = 5
    data["k6"] = 6
    data["k7"] = 7
    data["k8"] = 8
    data["k9"] = 9
    data["k10"] = 10
    data["k11"] = 11
    data["k12"] = 12
    data["k13"] = 13
    data["k14"] = 14
    data["k15"] = 15

    # Verify all values after rehashing
    print_int(data["k1"])
    print_int(data["k5"])
    print_int(data["k10"])
    print_int(data["k15"])

    return 0
}
