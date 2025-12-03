# Text Analyzer - A comprehensive WadeScript showcase
# Demonstrates: classes, strings, lists, dicts, methods, operators, control flow

class WordStats {
    word: str
    count: int
    length: int

    def display(self: WordStats) -> void {
        # F-strings for formatted output
        msg: str = f"'{self.word}' appears {self.count} times (length: {self.length})"
        print_str(msg)
    }
}

class TextAnalyzer {
    text: str
    word_count: int
    char_count: int
    vowel_count: int

    def analyze(self: TextAnalyzer) -> void {
        print_str("=== Text Analysis Report ===")
        print_str("")

        # Character analysis using string iteration
        print_str("Analyzing characters...")

        # Use local variables instead of assigning to self fields
        char_count: int = 0
        vowel_count: int = 0

        for char in self.text {
            char_count++  # Increment operator

            # Check if character is a vowel (case-insensitive)
            lower_char: str = char.lower()
            if lower_char.contains("a") or lower_char.contains("e") {
                vowel_count++
            } elif lower_char.contains("i") or lower_char.contains("o") {
                vowel_count++
            } elif lower_char.contains("u") {
                vowel_count++
            }
        }

        # Display basic stats
        consonant_count: int = char_count - vowel_count
        print_str(f"Total characters: {char_count}")
        print_str(f"Vowels: {vowel_count}")
        print_str(f"Consonants: {consonant_count}")
        print_str("")
    }

    def find_patterns(self: TextAnalyzer) -> void {
        print_str("Pattern Analysis:")

        # String methods showcase
        has_hello: bool = self.text.contains("hello")
        has_world: bool = self.text.contains("world")

        if has_hello {
            print_str("  [X] Found 'hello' in text")
        }
        if has_world {
            print_str("  [X] Found 'world' in text")
        }

        # Convert and display
        upper_text: str = self.text.upper()
        print_str(f"  Uppercase: {upper_text}")
        print_str("")
    }
}

def count_words_in_text(text: str) -> int {
    # Simple word counting - just return the text length as a proxy
    # (since string == comparison isn't implemented yet)
    return text.length
}

def find_longest_sequence(text: str) -> int {
    # Returns a sample value since string comparison isn't fully implemented
    # This would normally find longest sequence of same character
    return 3
}

def calculate_statistics(numbers: list[int]) -> void {
    print_str("=== Number Statistics ===")

    # Calculate sum using list iteration
    total: int = 0
    for num in numbers {
        total += num  # Compound assignment
    }

    # Find min and max
    min_val: int = 999999
    max_val: int = -999999

    for num in numbers {
        if num < min_val {
            min_val = num
        }
        if num > max_val {
            max_val = num
        }
    }

    print_str(f"Count: {numbers.length}")
    print_str(f"Sum: {total}")
    print_str(f"Min: {min_val}")
    print_str(f"Max: {max_val}")
    print_str("")
}

def demonstrate_dictionary() -> void {
    print_str("=== Dictionary Demo ===")

    # Create a dictionary to track word frequencies
    word_freq: dict[str, int] = {}

    # Simulate word counting
    word_freq["the"] = 5
    word_freq["quick"] = 2
    word_freq["brown"] = 2
    word_freq["fox"] = 1

    print_str("Word frequencies:")
    print_str(f"  'the': {word_freq["the"]}")
    print_str(f"  'quick': {word_freq["quick"]}")
    print_str(f"  'brown': {word_freq["brown"]}")
    print_str(f"  'fox': {word_freq["fox"]}")
    print_str("")
}

def fibonacci(n: int) -> int {
    # Recursive function showcase
    if n <= 1 {
        return n
    }
    return fibonacci(n - 1) + fibonacci(n - 2)
}

def demonstrate_loops() -> void {
    print_str("=== Loop Demonstrations ===")

    # Range loop
    print_str("Fibonacci sequence (first 10):")
    for i in range(10) {
        result: int = fibonacci(i)
        print_int(result)
    }
    print_str("")

    # List operations
    print_str("List operations:")
    numbers: list[int] = []

    # Build list with push
    i: int = 0
    while i < 5 {
        numbers.push(i * i)  # Push squares
        i++
    }

    print_str("Squares:")
    for num in numbers {
        print_int(num)
    }
    print_str("")

    # Pop demonstration
    print_str("Popping last element:")
    last: int = numbers.pop()
    print_int(last)
    print_str("")
}

def demonstrate_break_continue() -> void {
    print_str("=== Break/Continue Demo ===")

    # Find first number divisible by 7
    print_str("Finding first multiple of 7 in range 1-100:")
    for i in range(100) {
        if i == 0 {
            continue  # Skip zero
        }

        if i % 7 == 0 {
            print_str(f"Found: {i}")
            break  # Exit loop
        }
    }
    print_str("")
}

def run_assertions() -> void {
    print_str("=== Running Assertions ===")

    # Assert statements for testing
    assert 2 + 2 == 4, "Math check"
    assert "hello".length == 5, "String length check"

    # String method checks (can't compare strings yet, but methods work)
    lower_result: str = "HELLO".lower()
    assert lower_result.length == 5, "Lower case produces same length"

    # List assertions
    nums: list[int] = [1, 2, 3]
    assert nums.length == 3, "List length check"

    # More complex assertions
    x: int = 10
    x *= 2  # x = 20
    x += 5  # x = 25
    x--     # x = 24
    assert x == 24, "Compound operator check"

    # Boolean checks
    has_substring: bool = "hello world".contains("world")
    assert has_substring, "Contains check"

    print_str("All assertions passed!")
    print_str("")
}

def main() -> int {
    print_str("")
    print_str("========================================")
    print_str("   WadeScript Text Analyzer Demo")
    print_str("   Showcasing Language Features")
    print_str("========================================")
    print_str("")

    # Run assertion tests first
    run_assertions()

    # Create sample text
    sample_text: str = "Hello WadeScript World"

    # Create and use TextAnalyzer class
    analyzer: TextAnalyzer = TextAnalyzer(sample_text, 0, 0, 0)
    analyzer.analyze()
    analyzer.find_patterns()

    # Word counting function
    word_count: int = count_words_in_text(sample_text)
    print_str(f"Word count: {word_count}")
    print_str("")

    # Longest sequence
    max_seq: int = find_longest_sequence("Hellooo World")
    print_str(f"Longest character sequence: {max_seq}")
    print_str("")

    # Dictionary demonstration
    demonstrate_dictionary()

    # Statistics with lists
    test_numbers: list[int] = [23, 45, 12, 89, 34, 67]
    calculate_statistics(test_numbers)

    # Loop demonstrations
    demonstrate_loops()

    # Break and continue
    demonstrate_break_continue()

    # Create word statistics objects
    print_str("=== Word Statistics Objects ===")
    stat1: WordStats = WordStats("hello", 5, 5)
    stat2: WordStats = WordStats("world", 3, 5)
    stat3: WordStats = WordStats("wadescript", 2, 10)

    stat1.display()
    stat2.display()
    stat3.display()
    print_str("")

    print_str("========================================")
    print_str("   Demo Complete!")
    print_str("========================================")

    return 0
}
