# WadeScript for Visual Studio Code

Language support for WadeScript, a statically-typed programming language that compiles to native code via LLVM.

## Features

- Syntax highlighting for `.ws` files
- Error diagnostics (parse errors, type errors)
- Code completion
- Hover information (variable types, function signatures)
- Go to definition
- Find all references
- Rename symbol
- Document symbols (outline)
- Document formatting

## Requirements

- WadeScript compiler must be installed and available in PATH
- Or configure the path in settings

## Configuration

- `wadescript.serverPath`: Path to the wadescript executable. If empty, uses 'wadescript' from PATH.

## Installation

### From Source

1. Clone the WadeScript repository
2. Navigate to `editors/vscode`
3. Run `npm install`
4. Run `npm run compile`
5. Copy the extension folder to your VS Code extensions directory, or use `vsce package` to create a `.vsix` file

### Development

1. Open the `editors/vscode` folder in VS Code
2. Run `npm install`
3. Press F5 to launch a new VS Code window with the extension loaded

## Language Features

### Syntax Highlighting

The extension provides syntax highlighting for:
- Keywords (def, class, if, while, for, etc.)
- Types (int, float, str, bool, list, dict)
- String literals and f-strings
- Numbers (integers and floats)
- Comments
- Operators
- Function and class definitions
- Decorators

### Language Server

The extension uses the WadeScript Language Server Protocol (LSP) implementation for:
- Real-time error detection
- Intelligent code completion
- Type information on hover
- Navigation features

## Example Code

```wadescript
class Person {
    name: str
    age: int

    def greet(self: Person) -> void {
        print_str(f"Hello, {self.name}!")
    }
}

def main() -> int {
    p: Person = Person("Alice", 30)
    p.greet()
    return 0
}
```

## Troubleshooting

### Language server not starting

1. Ensure `wadescript` is in your PATH or configure `wadescript.serverPath`
2. Check the Output panel (View > Output > WadeScript Language Server)

### Syntax highlighting not working

1. Ensure the file has a `.ws` extension
2. Check that the language mode in the status bar shows "WadeScript"
