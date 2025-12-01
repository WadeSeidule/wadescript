# Test: Import system

import "test_helpers"

def main() -> int {
    # Test imported functions
    print_int(double(5))   # 10
    print_int(triple(4))   # 12

    # Test with variables
    x: int = 7
    print_int(double(x))   # 14
    print_int(triple(x))   # 21

    return 0
}
