# WadeScript Language Server

WadeScript includes a built-in Language Server Protocol (LSP) implementation for IDE integration.

## Starting the Language Server

```bash
# Using the ws utility
./ws lsp

# Or directly
./target/debug/wadescript lsp
```

The server communicates over stdin/stdout using the standard LSP protocol.

## Supported Features

### Text Document Synchronization
- Full document sync on open/change/save
- Document lifecycle (open, close)

### Diagnostics
- Parse errors (syntax errors)
- Type errors (type mismatches, undefined variables)
- Real-time error reporting as you type

### Navigation
- **Go to Definition**: Jump to variable, function, or class definitions
- **Find All References**: Find all usages of a symbol
- **Document Symbols**: Outline view of functions, classes, and variables

### Code Intelligence
- **Hover**: Show type information for variables and functions
- **Completion**:
  - Variables in scope
  - Functions and classes
  - Keywords and types
  - Method/field completion after `.`

### Refactoring
- **Rename Symbol**: Rename a variable, function, or class across the file

### Formatting
- **Document Formatting**: Auto-format the entire document

## Editor Integration

### Visual Studio Code / Cursor

A VSCode extension is provided in `editors/vscode/`. See the extension README for installation instructions.

**Quick Setup:**
1. Build the extension:
   ```bash
   cd editors/vscode
   npm install
   npm run compile
   ```

2. Copy to extensions folder or use `vsce package` to create a `.vsix`

3. Configure the path to wadescript if not in PATH:
   - Settings > WadeScript > Server Path

### Other Editors

Any editor with LSP support can use the WadeScript language server. Configure your editor to:
1. Run `wadescript lsp` as the language server
2. Associate `.ws` files with the server
3. Use stdio for communication

**Neovim (nvim-lspconfig):**
```lua
require'lspconfig'.wadescript.setup{
  cmd = { "wadescript", "lsp" },
  filetypes = { "wadescript" },
  root_dir = function(fname)
    return vim.fn.getcwd()
  end,
}
```

**Emacs (lsp-mode):**
```elisp
(lsp-register-client
 (make-lsp-client :new-connection (lsp-stdio-connection '("wadescript" "lsp"))
                  :major-modes '(wadescript-mode)
                  :server-id 'wadescript-ls))
```

## Architecture

The language server reuses the existing WadeScript compiler components:
- **Lexer**: Tokenizes source code with position tracking
- **Parser**: Builds AST (with error recovery in LSP mode)
- **Type Checker**: Validates types and tracks symbols

### Source Files

```
src/lsp/
  mod.rs          - Module root
  server.rs       - LSP server implementation (tower-lsp)
  document.rs     - Document state management
  analysis.rs     - Analysis coordinator
  diagnostics.rs  - Error conversion to LSP diagnostics
  span.rs         - Span and position utilities

src/language_defs.rs - Centralized language definitions (used by LSP)
```

### Centralized Language Definitions

The LSP uses `src/language_defs.rs` as the single source of truth for:
- **Keywords**: `get_keywords()` - all language keywords
- **Type keywords**: `get_type_keywords()` - int, float, str, bool, etc.
- **Built-in functions**: `get_builtin_functions()` - print_int, range, file_*, etc.
- **List/String methods**: `get_list_methods()`, `get_string_methods()`
- **Standard library**: `get_stdlib_modules()` - io, cli, http modules with functions and classes

When adding new language features, update `language_defs.rs` to ensure the LSP provides accurate completions and hover information.

### Standard Library Completions

The LSP provides completions for all standard library modules:

| Module | Functions | Classes |
|--------|-----------|---------|
| `io` | open, read, read_line, write, close, exists | - |
| `cli` | get_args, argc, argv, parse_int, parse_bool, starts_with, str_eq | - |
| `http` | get, post, put, delete, patch, head | HttpResponse |

Completions include full signatures and documentation. Example completions:
- `io` (module)
- `io.open` with signature `(path: str, mode: str) -> int`
- `HttpResponse` (class) with fields

## Troubleshooting

### Server not starting
- Ensure the wadescript binary is built: `make`
- Check that `./target/debug/wadescript` exists

### No diagnostics appearing
- Check that the file has a `.ws` extension
- Look at the language server output in your editor

### Completion not working
- Ensure you're in a valid code context
- Check for parse errors that might prevent analysis

## Development

To test changes to the language server:

1. Build in debug mode: `cargo build`
2. Test with a simple client:
   ```bash
   echo '{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"capabilities":{}}}' | ./target/debug/wadescript lsp
   ```

Run the Rust tests:
```bash
cargo test
```

## Future Improvements

Planned features for future phases:
- Workspace-wide symbol search
- Cross-file go-to-definition
- Signature help for function calls
- Code actions (quick fixes)
- Incremental parsing for better performance
