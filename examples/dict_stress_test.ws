# Stress test for dictionary hash table
# Tests multiple insertions and lookups
def main() -> int {
    # Create empty dictionary
    data: dict[str, int] = {}

    # Add many entries
    data["key1"] = 100
    data["key2"] = 200
    data["key3"] = 300
    data["key4"] = 400
    data["key5"] = 500
    data["key6"] = 600
    data["key7"] = 700
    data["key8"] = 800
    data["key9"] = 900
    data["key10"] = 1000
    data["key11"] = 1100
    data["key12"] = 1200
    data["key13"] = 1300
    data["key14"] = 1400
    data["key15"] = 1500

    print_str("Testing hash table with 15 entries")

    # Update some values
    data["key5"] = 555
    data["key10"] = 1010

    # Test lookups
    print_str("key1:")
    print_int(data["key1"])

    print_str("key5 (updated):")
    print_int(data["key5"])

    print_str("key10 (updated):")
    print_int(data["key10"])

    print_str("key15:")
    print_int(data["key15"])

    print_str("All tests passed!")
    return 0
}
