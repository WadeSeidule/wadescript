# Conditional logic
def max(a: int, b: int) -> int {
    if a > b {
        return a
    } else {
        return b
    }
}

def classify_number(n: int) -> int {
    if n < 0 {
        return -1
    } elif n == 0 {
        return 0
    } else {
        return 1
    }
}

def main() -> int {
    maximum: int = max(42, 17)
    classification: int = classify_number(-5)
    return maximum + classification
}
