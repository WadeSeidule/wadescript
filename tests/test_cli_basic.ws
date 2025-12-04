# Test basic CLI functionality
import "cli"

def main() -> int {
    # Test argc function
    argc: int = cli.argc()
    print_str("argc >= 1:")
    print_bool(argc >= 1)

    # Test get_args function
    args: list[str] = cli.get_args()
    print_str("args.length >= 1:")
    print_bool(args.length >= 1)

    # Test argv function
    arg0: str = cli.argv(0)
    print_str("argv(0) not empty:")
    print_bool(arg0.length > 0)

    # Test parse_int
    print_str("parse_int 42:")
    print_int(cli.parse_int("42"))

    # Test parse_bool
    print_str("parse_bool true:")
    print_bool(cli.parse_bool("true"))
    print_str("parse_bool false:")
    print_bool(cli.parse_bool("false"))

    # Test starts_with
    print_str("starts_with --foo, --:")
    print_bool(cli.starts_with("--foo", "--"))
    print_str("starts_with -x, --:")
    print_bool(cli.starts_with("-x", "--"))

    # Test str_eq
    print_str("str_eq hello, hello:")
    print_bool(cli.str_eq("hello", "hello"))
    print_str("str_eq hello, world:")
    print_bool(cli.str_eq("hello", "world"))

    print_str("CLI module works!")
    return 0
}
