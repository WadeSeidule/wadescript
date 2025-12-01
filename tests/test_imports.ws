# Test: Import system

import "test_helpers"

def main() -> int {
    # Test imported functions
    print_int(test_helpers.double(5))   # 10
    print_int(test_helpers.triple(4))   # 12

    # Test with variables
    x: int = 7
    print_int(test_helpers.double(x))   # 14
    print_int(test_helpers.triple(x))   # 21

    return 0
}
