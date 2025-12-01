# WadeScript Build Guide

## Quick Start

WadeScript provides convenient build utilities similar to Go's `go build` and `go run` commands.

### Prerequisites

First, build the WadeScript compiler:

```bash
cargo build
```

### Using the `ws` Command

The `ws` command provides two main subcommands:

#### `ws run` - Compile and Run

Compile and immediately run a WadeScript program:

```bash
./ws run examples/class_demo.ws
```

The compiled executable is automatically cleaned up after running.

**With Arguments:**

```bash
./ws run examples/program.ws arg1 arg2
```

#### `ws build` - Compile Only

Compile a WadeScript program to an executable:

```bash
./ws build examples/class_demo.ws
```

This creates an executable with the same name as the source file (without the `.ws` extension).

**Custom Output Name:**

```bash
./ws build examples/class_demo.ws -o myapp
```

This creates an executable named `myapp`.

## Examples

### Example 1: Quick Test

```bash
./ws run examples/class_demo.ws
```

Output:
```
Compiling examples/class_demo.ws...
✓ Compiled successfully
Running ./class_demo
---
Point created!
10
20
---
Program exited with code 0
```

### Example 2: Build for Distribution

```bash
./ws build examples/class_tests.ws -o tests
./tests
```

### Example 3: Development Workflow

```bash
# Quick iteration during development
./ws run examples/test.ws

# When ready to distribute
./ws build examples/test.ws -o my-program
```

## Manual Compilation

If you prefer to use the compiler directly:

```bash
./target/debug/wadescript examples/hello.ws
./hello
```

## Adding `ws` to PATH

To use `ws` from anywhere, add it to your PATH:

```bash
# Add to ~/.bashrc, ~/.zshrc, or equivalent
export PATH="$PATH:/path/to/wadescript"
```

Or create a symlink:

```bash
sudo ln -s /path/to/wadescript/ws /usr/local/bin/ws
```

Then you can use it from anywhere:

```bash
ws run myprogram.ws
ws build myprogram.ws
```

## Exit Codes

- `0` - Success
- `1` - Compilation error or file not found
- Other codes - Program's exit code

## Features

✅ Colored output for better readability
✅ Clear compilation status messages
✅ Automatic cleanup when using `ws run`
✅ Custom output names with `-o` flag
✅ Pass arguments to programs
✅ Shows program exit codes

## Troubleshooting

### "wadescript compiler not found"

Make sure you've built the compiler:

```bash
cargo build
```

### "Source file not found"

Check that the path to your `.ws` file is correct. Use relative or absolute paths.

### Colors not showing

If you're piping output or using a terminal without color support, the ANSI color codes will show as escape sequences. The script should work fine; it just won't look as pretty.
