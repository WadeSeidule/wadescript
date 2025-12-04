# Test that decorators are parsed correctly
# This file just verifies parsing works - no runtime use of decorators yet

class Args {
    @arg(help="Input file path")
    input_file: str

    @option(short="o", long="output", help="Output file")
    output: str

    @option(short="v", long="verbose")
    verbose: bool
}

def main() -> int {
    print_str("Decorators parsed successfully!")
    return 0
}
