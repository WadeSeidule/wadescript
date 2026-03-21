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

def main() -> int {
    # Basic instantiation and field access
    v: Vector2D = Vector2D(3, 4)
    assert v.x == 3
    assert v.y == 4

    # Method calls
    assert v.magnitude_squared() == 25

    # Method returning class instance
    v2: Vector2D = Vector2D(1, 2)
    v3: Vector2D = v.add(v2)
    assert v3.x == 4
    assert v3.y == 6

    # Counter pattern
    c: Counter = Counter(0)
    assert c.get() == 0
    c = c.increment()
    assert c.get() == 1
    c = c.increment()
    c = c.increment()
    assert c.get() == 3

    # String and bool fields
    n: Named = Named("test", True)
    assert n.name == "test"
    assert n.active == True

    # str() on class
    s: str = str(v)
    assert s.contains("Vector2D")
    assert s.contains("x=")
    assert s.contains("3")

    # print() on class
    print(v)
    print(c)
    print(n)

    return 0
}
