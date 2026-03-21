# Test: String methods and operations

def test_upper() -> int {
    assert "hello world".upper() == "HELLO WORLD", "upper should capitalize all"
    assert "abc123!@#".upper() == "ABC123!@#", "upper should only affect letters"
    assert "".upper() == "", "upper on empty string"
    assert "ALREADY".upper() == "ALREADY", "upper on already upper"
    return 0
}

def test_lower() -> int {
    assert "HELLO WORLD".lower() == "hello world", "lower should lowercase all"
    assert "ABC123!@#".lower() == "abc123!@#", "lower should only affect letters"
    assert "".lower() == "", "lower on empty string"
    assert "already".lower() == "already", "lower on already lower"
    return 0
}

def test_contains() -> int {
    sentence: str = "the quick brown fox"
    assert sentence.contains("quick") == True, "should find substring"
    assert sentence.contains("slow") == False, "should not find missing substring"
    assert sentence.contains("") == True, "empty substring always contained"
    assert sentence.contains("the quick brown fox") == True, "exact match"
    assert sentence.contains("THE") == False, "contains should be case-sensitive"
    return 0
}

def test_length() -> int {
    assert "hello".length == 5, "hello has length 5"
    assert "".length == 0, "empty string has length 0"
    assert "x".length == 1, "single char has length 1"
    assert "hello world".length == 11, "hello world has length 11"
    return 0
}

def test_concatenation() -> int {
    first: str = "Hello"
    second: str = " World"
    combined: str = first + second
    assert combined == "Hello World", "string concatenation"

    a: str = "a"
    b: str = "b"
    c: str = "c"
    assert a + b + c == "abc", "triple concatenation"
    return 0
}

def test_comparison() -> int {
    assert "abc" == "abc", "equal strings"
    assert "abc" != "def", "unequal strings"
    assert "hello" == "hello", "longer equal strings"
    assert "hello" != "Hello", "case differs"
    return 0
}

def test_indexing() -> int {
    word: str = "hello"
    assert word[0] == "h", "first char"
    assert word[4] == "o", "last char"
    assert word[2] == "l", "middle char"
    return 0
}

def main() -> int {
    test_upper()
    print_str("string upper: PASS")

    test_lower()
    print_str("string lower: PASS")

    test_contains()
    print_str("string contains: PASS")

    test_length()
    print_str("string length: PASS")

    test_concatenation()
    print_str("string concatenation: PASS")

    test_comparison()
    print_str("string comparison: PASS")

    test_indexing()
    print_str("string indexing: PASS")

    print_str("All string method tests passed!")
    return 0
}
