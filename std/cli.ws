# WadeScript Standard Library: cli
#
# Command-line argument parsing utilities
#
# Usage:
#   import "cli"
#
#   def main() -> int {
#       args: list[str] = cli.get_args()
#       print_int(args.length)
#       for arg in args {
#           print_str(arg)
#       }
#       return 0
#   }

# Get all command-line arguments as a list of strings
# Returns list including program name at index 0
def get_args() -> list[str] {
    args: list[str] = []
    argc: int = cli_get_argc()
    i: int = 0
    while i < argc {
        args.push(cli_get_argv_copy(i))
        i = i + 1
    }
    return args
}

# Get the number of command-line arguments (including program name)
def argc() -> int {
    return cli_get_argc()
}

# Get a specific argument by index
# Returns empty string if index is out of bounds
def argv(index: int) -> str {
    if index < 0 {
        return ""
    }
    if index >= cli_get_argc() {
        return ""
    }
    return cli_get_argv_copy(index)
}

# Parse an integer from a string argument
# Returns 0 on parse error
def parse_int(s: str) -> int {
    return cli_parse_int(s)
}

# Parse a boolean from a string argument
# Accepts: "true", "false", "1", "0", "yes", "no"
# Returns False on parse error
def parse_bool(s: str) -> bool {
    return cli_parse_bool(s) == 1
}

# Check if a string starts with a prefix
def starts_with(s: str, prefix: str) -> bool {
    return cli_starts_with(s, prefix) == 1
}

# Compare two strings for equality
def str_eq(a: str, b: str) -> bool {
    return cli_str_eq(a, b) == 1
}
