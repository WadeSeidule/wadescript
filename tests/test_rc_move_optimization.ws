# Test move semantics optimization for function returns
# When returning a local RC variable, ownership is transferred
# No release needed = optimization!

def create_list() -> list[int] {
    items: list[int] = [1, 2, 3, 4, 5]
    return items  # OPTIMIZED: Move semantics - no RC release
}

def create_dict() -> dict[str, int] {
    data: dict[str, int] = {"count": 100}
    return data  # OPTIMIZED: Move semantics - no RC release
}

def chain_returns() -> list[int] {
    a: list[int] = create_list()  # Receives ownership
    return a  # OPTIMIZED: Transfer ownership again
}

def main() -> int {
    # Each of these functions uses move semantics
    # No unnecessary retain/release pairs
    list1: list[int] = create_list()
    print_int(list1.get(0))
    print_int(list1.length)

    dict1: dict[str, int] = create_dict()
    print_int(dict1["count"])

    list2: list[int] = chain_returns()
    print_int(list2.get(4))

    print_str("Move optimization test passed")
    return 0
}
