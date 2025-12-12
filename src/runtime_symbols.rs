//! Runtime Symbol Registry
//!
//! Central registry of all runtime functions that must be available
//! in both compiled mode (linked from libwadescript_runtime.a) and
//! REPL mode (registered with JIT via LLVMAddSymbol).
//!
//! When adding a new runtime function:
//! 1. Add it to the appropriate category in RUNTIME_SYMBOLS
//! 2. Declare it in codegen.rs (declare_*_functions)
//! 3. Export it from src/runtime/*.rs with #[no_mangle] pub extern "C"
//!
//! The test_all_symbols_registered test will fail if JIT is missing symbols.

/// Runtime symbol with its function pointer for JIT registration
pub struct RuntimeSymbol {
    pub name: &'static str,
    pub addr: usize,
}

/// Get all runtime symbols that need JIT registration
/// This is the single source of truth for runtime functions
pub fn get_runtime_symbols() -> Vec<RuntimeSymbol> {
    // Import from each submodule explicitly
    use crate::runtime::list::{list_get_i64, list_push_i64, list_pop_i64, list_set_i64, list_slice_i64};
    use crate::runtime::dict::{dict_create, dict_set, dict_get, dict_has};
    use crate::runtime::string::{str_length, str_upper, str_lower, str_contains, str_char_at, str_slice};
    use crate::runtime::rc::{rc_alloc, rc_retain, rc_release, rc_get_count, rc_is_valid};
    use crate::runtime::io::{file_open, file_read, file_read_line, file_write, file_close, file_exists};
    use crate::runtime::cli::{
        cli_get_argc, cli_get_argv, cli_get_argv_copy, cli_parse_int, cli_parse_bool,
        cli_starts_with, cli_str_eq, cli_after_prefix
    };
    use crate::runtime::exceptions::{
        exception_create, exception_get_current, exception_set_current, exception_clear,
        exception_get_type, exception_get_message, exception_matches,
        exception_push_handler, exception_pop_handler, exception_raise
    };
    use crate::runtime::http::{
        http_get, http_get_with_headers, http_post, http_put, http_delete,
        http_patch, http_head, http_response_status, http_response_body,
        http_response_headers, http_response_get_header, http_response_free
    };
    use crate::runtime::{push_call_stack, pop_call_stack, runtime_error};

    vec![
        // List operations
        RuntimeSymbol { name: "list_get_i64", addr: list_get_i64 as usize },
        RuntimeSymbol { name: "list_push_i64", addr: list_push_i64 as usize },
        RuntimeSymbol { name: "list_pop_i64", addr: list_pop_i64 as usize },
        RuntimeSymbol { name: "list_set_i64", addr: list_set_i64 as usize },
        RuntimeSymbol { name: "list_slice_i64", addr: list_slice_i64 as usize },

        // Dict operations
        RuntimeSymbol { name: "dict_create", addr: dict_create as usize },
        RuntimeSymbol { name: "dict_set", addr: dict_set as usize },
        RuntimeSymbol { name: "dict_get", addr: dict_get as usize },
        RuntimeSymbol { name: "dict_has", addr: dict_has as usize },

        // String operations
        RuntimeSymbol { name: "str_length", addr: str_length as usize },
        RuntimeSymbol { name: "str_upper", addr: str_upper as usize },
        RuntimeSymbol { name: "str_lower", addr: str_lower as usize },
        RuntimeSymbol { name: "str_contains", addr: str_contains as usize },
        RuntimeSymbol { name: "str_char_at", addr: str_char_at as usize },
        RuntimeSymbol { name: "str_slice", addr: str_slice as usize },

        // RC operations
        RuntimeSymbol { name: "rc_alloc", addr: rc_alloc as usize },
        RuntimeSymbol { name: "rc_retain", addr: rc_retain as usize },
        RuntimeSymbol { name: "rc_release", addr: rc_release as usize },
        RuntimeSymbol { name: "rc_get_count", addr: rc_get_count as usize },
        RuntimeSymbol { name: "rc_is_valid", addr: rc_is_valid as usize },

        // File I/O operations
        RuntimeSymbol { name: "file_open", addr: file_open as usize },
        RuntimeSymbol { name: "file_read", addr: file_read as usize },
        RuntimeSymbol { name: "file_read_line", addr: file_read_line as usize },
        RuntimeSymbol { name: "file_write", addr: file_write as usize },
        RuntimeSymbol { name: "file_close", addr: file_close as usize },
        RuntimeSymbol { name: "file_exists", addr: file_exists as usize },

        // CLI operations
        RuntimeSymbol { name: "cli_get_argc", addr: cli_get_argc as usize },
        RuntimeSymbol { name: "cli_get_argv", addr: cli_get_argv as usize },
        RuntimeSymbol { name: "cli_get_argv_copy", addr: cli_get_argv_copy as usize },
        RuntimeSymbol { name: "cli_parse_int", addr: cli_parse_int as usize },
        RuntimeSymbol { name: "cli_parse_bool", addr: cli_parse_bool as usize },
        RuntimeSymbol { name: "cli_starts_with", addr: cli_starts_with as usize },
        RuntimeSymbol { name: "cli_str_eq", addr: cli_str_eq as usize },
        RuntimeSymbol { name: "cli_after_prefix", addr: cli_after_prefix as usize },

        // Exception handling
        RuntimeSymbol { name: "exception_create", addr: exception_create as usize },
        RuntimeSymbol { name: "exception_get_current", addr: exception_get_current as usize },
        RuntimeSymbol { name: "exception_set_current", addr: exception_set_current as usize },
        RuntimeSymbol { name: "exception_clear", addr: exception_clear as usize },
        RuntimeSymbol { name: "exception_get_type", addr: exception_get_type as usize },
        RuntimeSymbol { name: "exception_get_message", addr: exception_get_message as usize },
        RuntimeSymbol { name: "exception_matches", addr: exception_matches as usize },
        RuntimeSymbol { name: "exception_push_handler", addr: exception_push_handler as usize },
        RuntimeSymbol { name: "exception_pop_handler", addr: exception_pop_handler as usize },
        RuntimeSymbol { name: "exception_raise", addr: exception_raise as usize },

        // Call stack functions
        RuntimeSymbol { name: "push_call_stack", addr: push_call_stack as usize },
        RuntimeSymbol { name: "pop_call_stack", addr: pop_call_stack as usize },
        RuntimeSymbol { name: "runtime_error", addr: runtime_error as usize },

        // HTTP functions
        RuntimeSymbol { name: "http_get", addr: http_get as usize },
        RuntimeSymbol { name: "http_get_with_headers", addr: http_get_with_headers as usize },
        RuntimeSymbol { name: "http_post", addr: http_post as usize },
        RuntimeSymbol { name: "http_put", addr: http_put as usize },
        RuntimeSymbol { name: "http_delete", addr: http_delete as usize },
        RuntimeSymbol { name: "http_patch", addr: http_patch as usize },
        RuntimeSymbol { name: "http_head", addr: http_head as usize },
        RuntimeSymbol { name: "http_response_status", addr: http_response_status as usize },
        RuntimeSymbol { name: "http_response_body", addr: http_response_body as usize },
        RuntimeSymbol { name: "http_response_headers", addr: http_response_headers as usize },
        RuntimeSymbol { name: "http_response_get_header", addr: http_response_get_header as usize },
        RuntimeSymbol { name: "http_response_free", addr: http_response_free as usize },

        // Standard C library functions
        RuntimeSymbol { name: "printf", addr: libc::printf as usize },
        RuntimeSymbol { name: "malloc", addr: libc::malloc as usize },
        RuntimeSymbol { name: "free", addr: libc::free as usize },
        RuntimeSymbol { name: "memcpy", addr: libc::memcpy as usize },
        RuntimeSymbol { name: "strlen", addr: libc::strlen as usize },
        RuntimeSymbol { name: "exit", addr: libc::exit as usize },
    ]
}

/// Get just the names of all runtime symbols (for validation)
pub fn get_runtime_symbol_names() -> Vec<&'static str> {
    get_runtime_symbols().into_iter().map(|s| s.name).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_all_symbols_have_valid_addresses() {
        let symbols = get_runtime_symbols();
        for symbol in symbols {
            assert!(symbol.addr != 0, "Symbol '{}' has null address", symbol.name);
        }
    }

    #[test]
    fn test_no_duplicate_symbols() {
        let names = get_runtime_symbol_names();
        let mut seen = std::collections::HashSet::new();
        for name in names {
            assert!(seen.insert(name), "Duplicate symbol: {}", name);
        }
    }
}
