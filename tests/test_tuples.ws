# Test tuples: creation, unpacking, and indexing

def test_tuple_creation() -> int {
    # Create a simple tuple
    point: (int, int) = (10, 20)

    # Create a mixed-type tuple
    data: (str, int, bool) = ("Alice", 30, True)

    return 0
}

def test_tuple_indexing() -> int {
    # Access tuple elements by index
    point: (int, int) = (10, 20)

    x: int = point.0
    y: int = point.1

    assert x == 10, "point.0 should be 10"
    assert y == 20, "point.1 should be 20"

    # Access mixed-type tuple
    data: (str, int, bool) = ("Alice", 30, True)
    name: str = data.0
    age: int = data.1
    active: bool = data.2

    assert age == 30, "data.1 should be 30"
    assert active == True, "data.2 should be True"

    return 0
}

def test_tuple_unpacking() -> int {
    # Basic unpacking
    point: (int, int) = (10, 20)
    x, y = point

    assert x == 10, "unpacked x should be 10"
    assert y == 20, "unpacked y should be 20"

    # Unpacking mixed types
    data: (str, int, bool) = ("Alice", 30, True)
    name, age, active = data

    assert age == 30, "unpacked age should be 30"
    assert active == True, "unpacked active should be True"

    return 0
}

def get_point() -> (int, int) {
    return (100, 200)
}

def get_person() -> (str, int) {
    return ("Bob", 25)
}

def test_tuple_return() -> int {
    # Function returning tuple
    point: (int, int) = get_point()
    x: int = point.0
    y: int = point.1

    assert x == 100, "returned x should be 100"
    assert y == 200, "returned y should be 200"

    # Unpack returned tuple directly
    px, py = get_point()
    assert px == 100, "unpacked px should be 100"
    assert py == 200, "unpacked py should be 200"

    # Mixed types from function
    person: (str, int) = get_person()
    name, age = person
    assert age == 25, "unpacked age should be 25"

    return 0
}

def test_tuple_in_expressions() -> int {
    # Use tuple values in expressions
    point: (int, int) = (5, 10)
    sum: int = point.0 + point.1
    assert sum == 15, "sum should be 15"

    product: int = point.0 * point.1
    assert product == 50, "product should be 50"

    return 0
}

def test_nested_operations() -> int {
    # Multiple tuples and operations
    p1: (int, int) = (1, 2)
    p2: (int, int) = (3, 4)

    sum_x: int = p1.0 + p2.0
    sum_y: int = p1.1 + p2.1

    assert sum_x == 4, "sum_x should be 4"
    assert sum_y == 6, "sum_y should be 6"

    return 0
}

def main() -> int {
    test_tuple_creation()
    print_str("tuple creation: PASS")

    test_tuple_indexing()
    print_str("tuple indexing: PASS")

    test_tuple_unpacking()
    print_str("tuple unpacking: PASS")

    test_tuple_return()
    print_str("tuple return: PASS")

    test_tuple_in_expressions()
    print_str("tuple expressions: PASS")

    test_nested_operations()
    print_str("nested operations: PASS")

    print_str("All tuple tests passed!")
    return 0
}
