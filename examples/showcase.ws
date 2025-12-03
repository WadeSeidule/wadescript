# WadeScript Feature Showcase
# Demonstrates all major language features in a working program

class Person {
    name: str
    age: int
    score: int

    def display(self: Person) -> void {
        # F-strings for formatted output
        print_str(f"Person: {self.name}, Age: {self.age}, Score: {self.score}")
    }

    def is_adult(self: Person) -> bool {
        return self.age >= 18
    }
}

def fibonacci(n: int) -> int {
    # Recursive function
    if n <= 1 {
        return n
    }
    return fibonacci(n - 1) + fibonacci(n - 2)
}

def demonstrate_strings() -> void {
    print_str("=== String Features ===")

    # String properties
    text: str = "Hello WadeScript"
    len: int = text.length
    print_str(f"Text: {text}")
    print_str(f"Length: {len}")

    # String methods
    upper: str = text.upper()
    lower: str = text.lower()
    print_str(f"Uppercase: {upper}")
    print_str(f"Lowercase: {lower}")

    # String contains
    has_wade: bool = text.contains("Wade")
    has_python: bool = text.contains("Python")

    if has_wade {
        print_str("Found 'Wade' in text")
    }
    if not has_python {
        print_str("'Python' not found in text")
    }

    # String iteration
    print_str("Iterating over 'ABC':")
    for char in "ABC" {
        print_str(char)
    }

    print_str("")
}

def demonstrate_lists() -> void {
    print_str("=== List Features ===")

    # Create list with literal
    numbers: list[int] = [10, 20, 30, 40, 50]
    len: int = numbers.length
    print_str(f"Initial list length: {len}")

    # List iteration
    print_str("List elements:")
    for num in numbers {
        print_int(num)
    }

    print_str("")
}

def demonstrate_dictionaries() -> void {
    print_str("=== Dictionary Features ===")

    # Create dictionary
    scores: dict[str, int] = {}

    # Add entries
    scores["Alice"] = 95
    scores["Bob"] = 87
    scores["Charlie"] = 92

    # Read entries
    alice_score: int = scores["Alice"]
    bob_score: int = scores["Bob"]

    print_str(f"Alice's score: {alice_score}")
    print_str(f"Bob's score: {bob_score}")

    print_str("")
}

def demonstrate_loops() -> void {
    print_str("=== Loop Features ===")

    # Range loop
    print_str("Range 0 to 4:")
    for i in range(5) {
        print_int(i)
    }

    # While loop with break
    print_str("While with break at 3:")
    counter: int = 0
    while counter < 10 {
        if counter == 3 {
            break
        }
        print_int(counter)
        counter++  # Increment operator
    }

    # While with continue
    print_str("While with continue (skip even):")
    counter = 0
    while counter < 6 {
        counter++
        if counter % 2 == 0 {
            continue
        }
        print_int(counter)
    }

    print_str("")
}

def demonstrate_operators() -> void {
    print_str("=== Operator Features ===")

    # Compound assignment operators
    x: int = 10
    print_str(f"x = {x}")

    x += 5
    print_str(f"After x += 5: {x}")

    x -= 3
    print_str(f"After x -= 3: {x}")

    x *= 2
    print_str(f"After x *= 2: {x}")

    x /= 4
    print_str(f"After x /= 4: {x}")

    # Increment and decrement
    x++
    print_str(f"After x++: {x}")

    x--
    print_str(f"After x--: {x}")

    print_str("")
}

def demonstrate_control_flow() -> void {
    print_str("=== Control Flow ===")

    age: int = 25

    # If/elif/else
    if age < 13 {
        print_str("Child")
    } elif age < 18 {
        print_str("Teenager")
    } elif age < 65 {
        print_str("Adult")
    } else {
        print_str("Senior")
    }

    # Logical operators
    has_license: bool = True
    can_drive: bool = age >= 16 and has_license

    if can_drive {
        print_str("Can drive")
    }

    print_str("")
}

def calculate_stats(nums: list[int]) -> int {
    # Calculate sum
    total: int = 0
    for n in nums {
        total += n
    }
    return total
}

def run_tests() -> void {
    print_str("=== Running Tests (Assertions) ===")

    # Basic assertions
    assert 2 + 2 == 4, "Basic math"
    assert 10 > 5, "Comparison"
    assert True or False, "Logic"

    # String assertions
    s: str = "test"
    assert s.length == 4, "String length"

    # List assertions
    nums: list[int] = [1, 2, 3]
    assert nums.length == 3, "List length"

    # Function call assertion
    sum: int = calculate_stats(nums)
    assert sum == 6, "Sum calculation"  # 1 + 2 + 3 = 6

    print_str("All tests passed!")
    print_str("")
}

def main() -> int {
    print_str("")
    print_str("=========================================")
    print_str("  WadeScript Comprehensive Feature Demo")
    print_str("=========================================")
    print_str("")

    # Run assertions first
    run_tests()

    # Demonstrate each feature category
    demonstrate_strings()
    demonstrate_lists()
    demonstrate_dictionaries()
    demonstrate_loops()
    demonstrate_operators()
    demonstrate_control_flow()

    # Classes
    print_str("=== Class Features ===")
    person1: Person = Person("Alice", 25, 95)
    person2: Person = Person("Bob", 17, 87)
    person3: Person = Person("Charlie", 30, 92)

    person1.display()
    person2.display()
    person3.display()

    if person1.is_adult() {
        print_str(f"{person1.name} is an adult")
    }
    if not person2.is_adult() {
        print_str(f"{person2.name} is not an adult")
    }

    print_str("")

    # Recursion
    print_str("=== Recursion (Fibonacci) ===")
    print_str("First 10 Fibonacci numbers:")
    for i in range(10) {
        fib: int = fibonacci(i)
        print_int(fib)
    }

    print_str("")
    print_str("=========================================")
    print_str("  Demo Complete!")
    print_str("=========================================")

    return 0
}
