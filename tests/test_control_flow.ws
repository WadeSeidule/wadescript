# Test: Control flow - if/elif/else and while loops

def main() -> int {
    # If statements
    x: int = 10
    if x > 5 {
        print_str("greater")  # greater
    }

    if x < 5 {
        print_str("less")
    } else {
        print_str("not_less")  # not_less
    }

    # Elif
    score: int = 75
    if score >= 90 {
        print_str("A")
    } elif score >= 80 {
        print_str("B")
    } elif score >= 70 {
        print_str("C")  # C
    } else {
        print_str("F")
    }

    # While loop
    i: int = 0
    while i < 5 {
        print_int(i)  # 0 1 2 3 4
        i = i + 1
    }

    # While with break condition (manual break via conditional)
    counter: int = 0
    sum: int = 0
    while counter < 10 {
        sum = sum + counter
        counter = counter + 1
    }
    print_int(sum)  # 45

    return 0
}
