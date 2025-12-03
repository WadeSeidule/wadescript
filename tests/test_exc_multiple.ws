def test_value_error() -> void {
    try {
        raise ValueError("Value error test")
    } except ValueError {
        print_str("Caught ValueError")
    } except KeyError {
        print_str("Should not print")
    }
}

def test_key_error() -> void {
    try {
        raise KeyError("Key error test")
    } except ValueError {
        print_str("Should not print")
    } except KeyError {
        print_str("Caught KeyError")
    }
}

def main() -> int {
    test_value_error()
    test_key_error()
    return 0
}
