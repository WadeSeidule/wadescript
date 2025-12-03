# Test simple raise without try/except

def main() -> int {
    print_str("Before raise")
    raise ValueError("This is a test error")
    print_str("After raise (should not execute)")
    return 0
}
