# Test: str() with classes

class Person {
    name: str
    age: int
}

class Point {
    x: int
    y: int
}

class Settings {
    enabled: bool
    count: int
}

def main() -> int {
    # Test class string representation
    p: Person = Person("Alice", 30)
    s1: str = str(p)
    assert s1.contains("Person")
    assert s1.contains("name=")
    assert s1.contains("Alice")
    assert s1.contains("age=")
    assert s1.contains("30")

    # Test print with class
    print(p)

    # Test another class
    pt: Point = Point(10, 20)
    s2: str = str(pt)
    assert s2.contains("Point")
    assert s2.contains("x=")
    assert s2.contains("10")
    assert s2.contains("y=")
    assert s2.contains("20")

    print(pt)

    # Test class with bool field
    settings: Settings = Settings(True, 5)
    s3: str = str(settings)
    assert s3.contains("Settings")
    assert s3.contains("enabled=")
    assert s3.contains("True")
    assert s3.contains("count=")
    assert s3.contains("5")

    print(settings)

    print_str("All class str tests passed!")
    return 0
}
