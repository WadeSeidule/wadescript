# Test: String methods and operations

def main() -> int {
    # Test .upper()
    hello: str = "hello world"
    assert hello.upper() == "HELLO WORLD"

    # Test .lower()
    upper: str = "HELLO WORLD"
    assert upper.lower() == "hello world"

    # Test .contains()
    sentence: str = "the quick brown fox"
    assert sentence.contains("quick") == True
    assert sentence.contains("slow") == False
    assert sentence.contains("") == True
    assert sentence.contains("the quick brown fox") == True

    # Test .length property
    assert hello.length == 11
    empty: str = ""
    assert empty.length == 0
    single: str = "x"
    assert single.length == 1

    # Test string concatenation
    first: str = "Hello"
    second: str = " World"
    combined: str = first + second
    assert combined == "Hello World"

    # Test multiple concatenation
    a: str = "a"
    b: str = "b"
    c: str = "c"
    abc: str = a + b + c
    assert abc == "abc"

    # Test string comparison
    assert "abc" == "abc"
    assert "abc" != "def"

    # Test string indexing (char_at via [])
    word: str = "hello"
    assert word[0] == "h"
    assert word[4] == "o"

    # Test chained string methods
    mixed: str = "Hello World"
    assert mixed.lower() == "hello world"
    assert mixed.upper() == "HELLO WORLD"

    # Test .upper() / .lower() with numbers and symbols
    special: str = "abc123!@#"
    assert special.upper() == "ABC123!@#"
    upper_special: str = "ABC123!@#"
    assert upper_special.lower() == "abc123!@#"

    return 0
}
