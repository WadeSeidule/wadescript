# Test: Import system

import "helpers"

def main() -> int {
    # Test imported functions
    assert helpers.double(5) == 10
    assert helpers.triple(4) == 12

    # Test with variables
    x: int = 7
    assert helpers.double(x) == 14
    assert helpers.triple(x) == 21

    # Test with expressions
    assert helpers.double(3 + 2) == 10
    assert helpers.triple(2 * 2) == 12

    return 0
}
