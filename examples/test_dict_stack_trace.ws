# Test dictionary error with stack trace

def lookup_score(scores: dict[str, int], name: str) -> int {
    print_str(f"Looking up score for {name}")
    return scores[name]  # Will error if name not found
}

def process_student(scores: dict[str, int], name: str) -> void {
    print_str(f"Processing student: {name}")
    score: int = lookup_score(scores, name)
    print_int(score)
}

def main() -> int {
    scores: dict[str, int] = {}
    scores["Alice"] = 95
    scores["Bob"] = 87

    # This works
    process_student(scores, "Alice")

    # This should trigger error with stack trace: main -> process_student -> lookup_score
    process_student(scores, "Charlie")

    return 0
}
