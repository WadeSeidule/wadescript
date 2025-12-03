# Test parse error reporting

def main() -> int {
    x: int = 42
    print_int(x)
    return 0
}

# This should cause a parse error - missing parameter name
def broken_function(: int) -> void {
    print_str("broken")
}
