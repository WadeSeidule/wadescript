# Test: Dictionary iteration and .length property

def main() -> int {
    # Test dict.length on empty dict
    empty: dict[str, int] = {}
    assert empty.length == 0

    # Test dict.length after adding entries
    scores: dict[str, int] = {"Math": 95, "English": 88, "Science": 92}
    assert scores.length == 3

    # Test dict iteration with for-in
    total: int = 0
    for key in scores {
        total = total + scores[key]
    }
    assert total == 275

    # Test iteration collects all keys
    count: int = 0
    for key in scores {
        count = count + 1
    }
    assert count == 3

    # Test dict with updates preserves length
    scores["Math"] = 100
    assert scores.length == 3
    assert scores["Math"] == 100

    # Test adding new key increases length
    scores["History"] = 85
    assert scores.length == 4

    # Test iteration after modification
    total2: int = 0
    for key in scores {
        total2 = total2 + scores[key]
    }
    assert total2 == 365

    return 0
}
