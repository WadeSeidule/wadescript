# Test dictionary key not found error with line number

def main() -> int {
    scores: dict[str, int] = {}
    scores["Alice"] = 95

    # This should error on line 7 with key name and line number
    val: int = scores["Missing"]

    return 0
}
