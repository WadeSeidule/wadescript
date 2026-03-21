# Test: Class definition, instantiation, fields, and methods

class Vector2D {
    x: int
    y: int

    def magnitude_squared(self: Vector2D) -> int {
        return self.x * self.x + self.y * self.y
    }

    def add(self: Vector2D, other: Vector2D) -> Vector2D {
        return Vector2D(self.x + other.x, self.y + other.y)
    }
}

class Counter {
    value: int

    def increment(self: Counter) -> Counter {
        return Counter(self.value + 1)
    }

    def get(self: Counter) -> int {
        return self.value
    }
}

class Named {
    name: str
    active: bool
}

def test_field_access() -> int {
    v: Vector2D = Vector2D(3, 4)
    assert v.x == 3, "v.x should be 3"
    assert v.y == 4, "v.y should be 4"
    return 0
}

def test_method_call() -> int {
    v: Vector2D = Vector2D(3, 4)
    assert v.magnitude_squared() == 25, "3^2 + 4^2 should be 25"
    return 0
}

def test_method_returning_class() -> int {
    v1: Vector2D = Vector2D(3, 4)
    v2: Vector2D = Vector2D(1, 2)
    v3: Vector2D = v1.add(v2)
    assert v3.x == 4, "added x should be 4"
    assert v3.y == 6, "added y should be 6"
    return 0
}

def test_method_chaining() -> int {
    c: Counter = Counter(0)
    assert c.get() == 0, "initial counter should be 0"
    c = c.increment()
    assert c.get() == 1, "after one increment"
    c = c.increment()
    c = c.increment()
    assert c.get() == 3, "after three increments"
    return 0
}

def test_string_bool_fields() -> int {
    n: Named = Named("test", True)
    assert n.name == "test", "name field"
    assert n.active == True, "bool field"
    return 0
}

def test_class_str() -> int {
    v: Vector2D = Vector2D(3, 4)
    s: str = str(v)
    assert s.contains("Vector2D"), "str() should contain class name"
    assert s.contains("x="), "str() should contain field name x"
    assert s.contains("3"), "str() should contain field value 3"
    return 0
}

def main() -> int {
    test_field_access()
    print_str("field access: PASS")

    test_method_call()
    print_str("method call: PASS")

    test_method_returning_class()
    print_str("method returning class: PASS")

    test_method_chaining()
    print_str("method chaining: PASS")

    test_string_bool_fields()
    print_str("string and bool fields: PASS")

    test_class_str()
    print_str("class str(): PASS")

    print_str("All class tests passed!")
    return 0
}
