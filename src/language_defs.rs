//! Language Definitions Registry
//!
//! Central registry of WadeScript language constructs for use by:
//! - LSP (completion, hover info)
//! - Documentation generators
//! - Syntax highlighters
//!
//! When adding new keywords, types, or built-in functions:
//! 1. Add them to the appropriate function here
//! 2. Update the lexer if adding keywords/types
//! 3. Update the typechecker if adding built-in functions
//!
//! This ensures the LSP stays in sync with language changes.

/// Built-in function with signature info for LSP
pub struct BuiltinFunction {
    pub name: &'static str,
    pub signature: &'static str,
    pub description: &'static str,
}

/// Get all WadeScript keywords
/// These must match the keywords in lexer.rs
pub fn get_keywords() -> Vec<&'static str> {
    vec![
        // Control flow
        "if", "elif", "else", "while", "for", "in",
        "break", "continue", "pass", "return",
        // Functions and classes
        "def", "class",
        // Exception handling
        "try", "except", "finally", "raise", "as",
        // Imports
        "import",
        // Testing
        "assert",
        // Logical operators
        "and", "or", "not",
        // Literals
        "True", "False", "None",
    ]
}

/// Get all WadeScript type keywords
/// These must match the type tokens in lexer.rs
pub fn get_type_keywords() -> Vec<&'static str> {
    vec![
        "int", "float", "str", "bool", "void",
        "list", "dict", "array", "Optional",
    ]
}

/// Get all built-in functions with their signatures
/// These must match the functions registered in typechecker.rs
pub fn get_builtin_functions() -> Vec<BuiltinFunction> {
    vec![
        // Print functions
        BuiltinFunction {
            name: "print_int",
            signature: "(value: int) -> void",
            description: "Print an integer to stdout",
        },
        BuiltinFunction {
            name: "print_float",
            signature: "(value: float) -> void",
            description: "Print a float to stdout",
        },
        BuiltinFunction {
            name: "print_str",
            signature: "(value: str) -> void",
            description: "Print a string to stdout",
        },
        BuiltinFunction {
            name: "print_bool",
            signature: "(value: bool) -> void",
            description: "Print a boolean to stdout",
        },
        // Utility functions
        BuiltinFunction {
            name: "range",
            signature: "(n: int) -> list[int]",
            description: "Return a list of integers from 0 to n-1",
        },
        // File I/O functions
        BuiltinFunction {
            name: "file_open",
            signature: "(path: str, mode: str) -> int",
            description: "Open a file and return a handle. Mode: \"r\", \"w\", \"a\"",
        },
        BuiltinFunction {
            name: "file_read",
            signature: "(handle: int) -> str",
            description: "Read entire contents of a file",
        },
        BuiltinFunction {
            name: "file_read_line",
            signature: "(handle: int) -> str",
            description: "Read a single line from a file",
        },
        BuiltinFunction {
            name: "file_write",
            signature: "(handle: int, content: str) -> void",
            description: "Write content to a file",
        },
        BuiltinFunction {
            name: "file_close",
            signature: "(handle: int) -> void",
            description: "Close a file handle",
        },
        BuiltinFunction {
            name: "file_exists",
            signature: "(path: str) -> int",
            description: "Check if a file exists (returns 1 or 0)",
        },
        // CLI functions
        BuiltinFunction {
            name: "cli_get_argc",
            signature: "() -> int",
            description: "Get the number of command-line arguments",
        },
        BuiltinFunction {
            name: "cli_get_argv",
            signature: "(index: int) -> str",
            description: "Get a command-line argument by index",
        },
        BuiltinFunction {
            name: "cli_get_argv_copy",
            signature: "(index: int) -> str",
            description: "Get a copy of a command-line argument by index",
        },
        BuiltinFunction {
            name: "cli_parse_int",
            signature: "(s: str) -> int",
            description: "Parse a string to an integer",
        },
        BuiltinFunction {
            name: "cli_parse_bool",
            signature: "(s: str) -> int",
            description: "Parse a string to a boolean (1 or 0)",
        },
        BuiltinFunction {
            name: "cli_starts_with",
            signature: "(s: str, prefix: str) -> int",
            description: "Check if a string starts with a prefix (1 or 0)",
        },
        BuiltinFunction {
            name: "cli_str_eq",
            signature: "(a: str, b: str) -> int",
            description: "Check if two strings are equal (1 or 0)",
        },
        BuiltinFunction {
            name: "cli_after_prefix",
            signature: "(s: str, prefix: str) -> str",
            description: "Get the part of a string after a prefix",
        },
        // HTTP functions
        BuiltinFunction {
            name: "http_get",
            signature: "(url: str) -> int",
            description: "Perform HTTP GET request, returns response handle",
        },
        BuiltinFunction {
            name: "http_get_with_headers",
            signature: "(url: str, headers: str) -> int",
            description: "HTTP GET with custom headers (newline-separated)",
        },
        BuiltinFunction {
            name: "http_post",
            signature: "(url: str, body: str, headers: str) -> int",
            description: "Perform HTTP POST request",
        },
        BuiltinFunction {
            name: "http_put",
            signature: "(url: str, body: str, headers: str) -> int",
            description: "Perform HTTP PUT request",
        },
        BuiltinFunction {
            name: "http_delete",
            signature: "(url: str, headers: str) -> int",
            description: "Perform HTTP DELETE request",
        },
        BuiltinFunction {
            name: "http_patch",
            signature: "(url: str, body: str, headers: str) -> int",
            description: "Perform HTTP PATCH request",
        },
        BuiltinFunction {
            name: "http_head",
            signature: "(url: str, headers: str) -> int",
            description: "Perform HTTP HEAD request",
        },
        BuiltinFunction {
            name: "http_response_status",
            signature: "(handle: int) -> int",
            description: "Get HTTP response status code",
        },
        BuiltinFunction {
            name: "http_response_body",
            signature: "(handle: int) -> str",
            description: "Get HTTP response body",
        },
        BuiltinFunction {
            name: "http_response_headers",
            signature: "(handle: int) -> str",
            description: "Get all HTTP response headers",
        },
        BuiltinFunction {
            name: "http_response_get_header",
            signature: "(handle: int, name: str) -> str",
            description: "Get a specific HTTP response header",
        },
        BuiltinFunction {
            name: "http_response_free",
            signature: "(handle: int) -> void",
            description: "Free an HTTP response handle",
        },
    ]
}

/// Get list method signatures for LSP
pub fn get_list_methods() -> Vec<(&'static str, &'static str, &'static str)> {
    vec![
        ("push", "(item: T) -> void", "Add an item to the end of the list"),
        ("pop", "() -> T", "Remove and return the last item"),
        ("get", "(index: int) -> T", "Get item at index"),
        ("length", "int", "Number of items in the list (property)"),
    ]
}

/// Get string method signatures for LSP
pub fn get_string_methods() -> Vec<(&'static str, &'static str, &'static str)> {
    vec![
        ("upper", "() -> str", "Convert to uppercase"),
        ("lower", "() -> str", "Convert to lowercase"),
        ("contains", "(substr: str) -> bool", "Check if contains substring"),
        ("split", "(delimiter: str) -> list[str]", "Split string by delimiter"),
        ("length", "int", "Length of the string (property)"),
    ]
}

/// Standard library module with its functions
pub struct StdLibModule {
    pub name: &'static str,
    pub description: &'static str,
    pub functions: Vec<StdLibFunction>,
    pub classes: Vec<StdLibClass>,
}

/// Standard library function
pub struct StdLibFunction {
    pub name: &'static str,
    pub signature: &'static str,
    pub description: &'static str,
}

/// Standard library class
pub struct StdLibClass {
    pub name: &'static str,
    pub fields: Vec<(&'static str, &'static str)>, // (name, type)
    pub description: &'static str,
}

/// Get all standard library modules
/// These must match the modules in std/*.ws
pub fn get_stdlib_modules() -> Vec<StdLibModule> {
    vec![
        // io module
        StdLibModule {
            name: "io",
            description: "File I/O operations",
            functions: vec![
                StdLibFunction {
                    name: "open",
                    signature: "(path: str, mode: str) -> int",
                    description: "Open a file. Mode: \"r\", \"w\", \"a\". Returns file handle.",
                },
                StdLibFunction {
                    name: "read",
                    signature: "(handle: int) -> str",
                    description: "Read entire file contents as a string",
                },
                StdLibFunction {
                    name: "read_line",
                    signature: "(handle: int) -> str",
                    description: "Read a single line from file",
                },
                StdLibFunction {
                    name: "write",
                    signature: "(handle: int, content: str) -> void",
                    description: "Write a string to file",
                },
                StdLibFunction {
                    name: "close",
                    signature: "(handle: int) -> void",
                    description: "Close a file handle",
                },
                StdLibFunction {
                    name: "exists",
                    signature: "(path: str) -> bool",
                    description: "Check if a file exists",
                },
            ],
            classes: vec![],
        },
        // cli module
        StdLibModule {
            name: "cli",
            description: "Command-line argument parsing",
            functions: vec![
                StdLibFunction {
                    name: "get_args",
                    signature: "() -> list[str]",
                    description: "Get all command-line arguments as a list",
                },
                StdLibFunction {
                    name: "argc",
                    signature: "() -> int",
                    description: "Get the number of command-line arguments",
                },
                StdLibFunction {
                    name: "argv",
                    signature: "(index: int) -> str",
                    description: "Get a specific argument by index",
                },
                StdLibFunction {
                    name: "parse_int",
                    signature: "(s: str) -> int",
                    description: "Parse an integer from a string",
                },
                StdLibFunction {
                    name: "parse_bool",
                    signature: "(s: str) -> bool",
                    description: "Parse a boolean from a string",
                },
                StdLibFunction {
                    name: "starts_with",
                    signature: "(s: str, prefix: str) -> bool",
                    description: "Check if a string starts with a prefix",
                },
                StdLibFunction {
                    name: "str_eq",
                    signature: "(a: str, b: str) -> bool",
                    description: "Compare two strings for equality",
                },
            ],
            classes: vec![],
        },
        // http module
        StdLibModule {
            name: "http",
            description: "HTTP client for web requests",
            functions: vec![
                StdLibFunction {
                    name: "get",
                    signature: "(url: str, headers: dict[str, str] = {}) -> HttpResponse",
                    description: "Perform a GET request with optional headers dict",
                },
                StdLibFunction {
                    name: "post",
                    signature: "(url: str, body: str, headers: dict[str, str] = {}) -> HttpResponse",
                    description: "Perform a POST request with optional headers dict",
                },
                StdLibFunction {
                    name: "put",
                    signature: "(url: str, body: str, headers: dict[str, str] = {}) -> HttpResponse",
                    description: "Perform a PUT request with optional headers dict",
                },
                StdLibFunction {
                    name: "delete",
                    signature: "(url: str, headers: dict[str, str] = {}) -> HttpResponse",
                    description: "Perform a DELETE request with optional headers dict",
                },
                StdLibFunction {
                    name: "patch",
                    signature: "(url: str, body: str, headers: dict[str, str] = {}) -> HttpResponse",
                    description: "Perform a PATCH request with optional headers dict",
                },
                StdLibFunction {
                    name: "head",
                    signature: "(url: str, headers: dict[str, str] = {}) -> HttpResponse",
                    description: "Perform a HEAD request with optional headers dict",
                },
            ],
            classes: vec![
                StdLibClass {
                    name: "HttpResponse",
                    fields: vec![
                        ("status", "int"),
                        ("body", "str"),
                        ("headers", "str"),
                    ],
                    description: "HTTP response containing status, body, and headers",
                },
            ],
        },
    ]
}

/// Get stdlib module names for import completion
pub fn get_stdlib_module_names() -> Vec<&'static str> {
    vec!["io", "cli", "http"]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_keywords_not_empty() {
        assert!(!get_keywords().is_empty());
    }

    #[test]
    fn test_type_keywords_not_empty() {
        assert!(!get_type_keywords().is_empty());
    }

    #[test]
    fn test_builtin_functions_not_empty() {
        assert!(!get_builtin_functions().is_empty());
    }

    #[test]
    fn test_no_duplicate_keywords() {
        let keywords = get_keywords();
        let mut seen = std::collections::HashSet::new();
        for kw in keywords {
            assert!(seen.insert(kw), "Duplicate keyword: {}", kw);
        }
    }

    #[test]
    fn test_no_duplicate_builtins() {
        let builtins = get_builtin_functions();
        let mut seen = std::collections::HashSet::new();
        for f in builtins {
            assert!(seen.insert(f.name), "Duplicate builtin: {}", f.name);
        }
    }

    #[test]
    fn test_stdlib_modules_not_empty() {
        assert!(!get_stdlib_modules().is_empty());
    }

    #[test]
    fn test_stdlib_module_names_match() {
        let modules = get_stdlib_modules();
        let names = get_stdlib_module_names();
        assert_eq!(modules.len(), names.len());
        for module in modules {
            assert!(names.contains(&module.name), "Module {} not in names list", module.name);
        }
    }

    #[test]
    fn test_stdlib_io_module() {
        let modules = get_stdlib_modules();
        let io = modules.iter().find(|m| m.name == "io").expect("io module not found");
        assert!(!io.functions.is_empty());
        assert!(io.functions.iter().any(|f| f.name == "open"));
        assert!(io.functions.iter().any(|f| f.name == "read"));
        assert!(io.functions.iter().any(|f| f.name == "write"));
        assert!(io.functions.iter().any(|f| f.name == "close"));
    }

    #[test]
    fn test_stdlib_http_module() {
        let modules = get_stdlib_modules();
        let http = modules.iter().find(|m| m.name == "http").expect("http module not found");
        assert!(!http.functions.is_empty());
        assert!(http.functions.iter().any(|f| f.name == "get"));
        assert!(http.functions.iter().any(|f| f.name == "post"));
        assert!(http.functions.iter().any(|f| f.name == "put"));
        assert!(http.functions.iter().any(|f| f.name == "delete"));
        assert!(http.functions.iter().any(|f| f.name == "patch"));
        assert!(http.functions.iter().any(|f| f.name == "head"));
        // Verify no _with_headers variants (simplified API)
        assert!(!http.functions.iter().any(|f| f.name.contains("_with_headers")));
        assert!(!http.classes.is_empty());
        assert!(http.classes.iter().any(|c| c.name == "HttpResponse"));
    }
}
