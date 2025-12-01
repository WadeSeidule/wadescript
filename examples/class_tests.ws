# Test 1: Multiple classes
class Vector2D {
    x: int
    y: int

    def init(self: Vector2D) {
        print_str("Vector2D created")
    }

    def magnitude_squared(self: Vector2D) -> int {
        return self.x * self.x + self.y * self.y
    }

    def print_coords(self: Vector2D) {
        print_str("Coordinates:")
        print_int(self.x)
        print_int(self.y)
    }
}

class Rectangle {
    width: int
    height: int

    def init(self: Rectangle) {
        print_str("Rectangle created")
    }

    def area(self: Rectangle) -> int {
        return self.width * self.height
    }

    def perimeter(self: Rectangle) -> int {
        return 2 * (self.width + self.height)
    }
}

# Test 2: Class types in function parameters and return types
def create_vector(x: int, y: int) -> Vector2D {
    return Vector2D(x, y)
}

def print_vector_info(v: Vector2D) {
    print_str("Vector info:")
    v.print_coords()
    print_str("Magnitude squared:")
    print_int(v.magnitude_squared())
}

# Test 3: Multiple instances
def test_multiple_instances() {
    r1: Rectangle = Rectangle(5, 10)
    r2: Rectangle = Rectangle(3, 7)

    print_str("Rectangle 1 area:")
    print_int(r1.area())

    print_str("Rectangle 1 perimeter:")
    print_int(r1.perimeter())

    print_str("Rectangle 2 area:")
    print_int(r2.area())

    print_str("Rectangle 2 perimeter:")
    print_int(r2.perimeter())
}

# Test 4: Methods with parameters and return values
def test_vector_operations() {
    v1: Vector2D = create_vector(3, 4)
    v2: Vector2D = create_vector(5, 12)

    print_str("First vector:")
    print_vector_info(v1)

    print_str("Second vector:")
    print_vector_info(v2)
}

def main() -> int {
    test_multiple_instances()
    test_vector_operations()
    return 0
}
