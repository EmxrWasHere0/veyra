# Running Veyra Programs

Veyra is an interpreted programming language written in Rust.

The interpreter executes Veyra source files directly, without requiring each program to be compiled into a standalone executable.

This guide explains how to run scripts, use the interactive REPL, execute tests, and work with Veyra's command-line interface.

## Requirements

To run Veyra programs, you need the Veyra interpreter.

If you want to build the interpreter from source, you also need Rust and Cargo.

For installation instructions, see [Installation](installation.md).

## 1. Running a Veyra File

Create a file named `hello.vey`:

```veyra
say "Hello, World!"

add greet(name)
    say "Hello, " + name + "!"
end

greet("Veyra")
```

Run the file:

```bash
veyra hello.vey
```

Output:

```text
Hello, World!
Hello, Veyra!
```

Veyra executes the statements in the file in order.

## 2. Running from Source

If you are developing Veyra, you can run programs using Cargo.

Clone the repository:

```bash
git clone https://github.com/EmxrWasHere0/veyra.git
cd veyra
```

Run a program:

```bash
cargo run -- hello.vey
```

Or run a repository example:

```bash
cargo run -- examples/hello.vey
```

The `--` separates Cargo arguments from arguments passed to Veyra.

### Release Mode

For optimized execution:

```bash
cargo run --release -- hello.vey
```

Release builds generally run faster than development builds.

## 3. Building the Interpreter

Build an optimized Veyra executable:

```bash
cargo build --release
```

The executable is placed in:

| Platform | Executable                 |
| -------- | -------------------------- |
| Linux    | `target/release/veyra`     |
| macOS    | `target/release/veyra`     |
| Windows  | `target/release/veyra.exe` |

Run the executable directly on Linux or macOS:

```bash
./target/release/veyra hello.vey
```

On Windows PowerShell:

```powershell
.\target\release\veyra.exe hello.vey
```

## 4. Interactive REPL

Veyra 0.2.6 includes an interactive Read-Eval-Print Loop (REPL).

Start it by running Veyra without arguments:

```bash
veyra
```

The interpreter displays an interactive prompt:

```text
>>>
```

You can enter statements directly:

```text
>>> add x = 10
>>> say x * 2
20
```

### Persistent State

Variables, functions, and imports remain available throughout the REPL session.

```text
>>> add name = "Veyra"
>>> say name
Veyra
>>> name = "Veyra 0.2.6"
>>> say name
Veyra 0.2.6
```

### Multiline Blocks

The REPL supports multiline functions, loops, and other blocks.

```text
>>> add square(n)
...     return n * n
... end
>>> say square(5)
25
```

The `...` prompt indicates that Veyra is waiting for more input.

### Leaving the REPL

Type:

```text
exit
```

Or:

```text
quit
```

You can also send an end-of-file signal using your terminal's standard keyboard shortcut.

## 5. Command-Line Options

Veyra provides several command-line options.

| Command                 | Description                     |
| ----------------------- | ------------------------------- |
| `veyra file.vey`        | Execute a Veyra source file     |
| `veyra`                 | Start the interactive REPL      |
| `veyra --help`          | Display usage information       |
| `veyra -h`              | Short form of `--help`          |
| `veyra --version`       | Display the interpreter version |
| `veyra -v`              | Short form of `--version`       |
| `veyra --test file.vey` | Execute Veyra test blocks       |
| `veyra -t file.vey`     | Short form of `--test`          |

### Checking the Version

```bash
veyra --version
```

The version is determined by the interpreter's Cargo package version.

### Displaying Help

```bash
veyra --help
```

This displays the supported command-line syntax and options.

## 6. Running Veyra Tests

Veyra supports test blocks and assertions.

Example file `tests.vey`:

```veyra
test addition
    assert 2 + 2 == 4
end
```

Run the test file:

```bash
veyra --test tests.vey
```

You can also use:

```bash
veyra -t tests.vey
```

Test mode executes the file's Veyra tests and reports their results.

### Interpreter Regression Tests

If you are developing the Veyra interpreter itself, run the Rust regression tests:

```bash
cargo test
```

These tests execute real Veyra programs and check their behavior.

This is different from using `veyra --test` to execute test blocks inside a Veyra source file.

## 7. Running Files from Different Directories

Veyra accepts relative and absolute file paths.

Relative path:

```bash
veyra examples/hello.vey
```

Absolute path on Linux or macOS:

```bash
veyra /home/user/projects/hello.vey
```

Absolute path on Windows:

```powershell
veyra "C:\Projects\Veyra\hello.vey"
```

Relative paths depend on the current working directory.

## 8. Command-Line Arguments

Veyra provides access to command-line arguments through the `system` module.

Example file `arguments.vey`:

```veyra
mod system

add arguments = system.args()

say arguments
```

Run it with arguments:

```bash
veyra arguments.vey first second
```

The `system.args()` function returns the script arguments exposed by the runtime.

The interpreter and script paths are excluded from the returned argument list.

## 9. Running Programs in VSCode

The Veyra VSCode extension supports syntax highlighting, snippets, and REPL integration.

To start the REPL:

1. Open the VSCode Command Palette.
2. Search for `Veyra: Start REPL`.
3. Select the command.

To run a script, use the integrated terminal:

```bash
veyra hello.vey
```

For installation and usage instructions, see [VSCode Extension](extension.md).

## 10. Runtime Errors

Veyra 0.2.6 provides detailed runtime error messages.

For example:

```veyra
add x = 10
add y = 0

say x / y
```

An error may look like:

```text
Runtime Error: example.vey:4: Division by zero

  4 | say x / y
    | ^^^^^^^^^
```

The error includes the source file, line number, and relevant source line.

Errors inside functions can also include a traceback showing the function call chain.

For more information, see [Error Handling](error-handling.md).

## 11. Troubleshooting

### Cargo Is Not Recognized

Install Rust and Cargo using [rustup](https://rustup.rs/).

Make sure Cargo is available in your system's `PATH`.

### Veyra Is Not Recognized

Check whether Veyra is installed:

```bash
cargo install --path .
```

Make sure Cargo's binary directory is included in your `PATH`.

You can also run Veyra through Cargo:

```bash
cargo run -- hello.vey
```

### Source File Not Found

Check the filename, file extension, and current working directory.

Use an absolute path when necessary.

### Module Not Found

Check that the module exists in one of Veyra's module search directories.

You can also configure additional directories using `VEYRA_PATH`.

See [Modules](modules.md).

### Runtime Error

Read the source location and traceback.

Check the operation, its arguments, and any required external resources.

## What's Next?

Continue learning Veyra:

- [Installation](installation.md)
- [Quick Start](quickstart.md)
- [Modules](modules.md)
- [Error Handling](error-handling.md)
- [VSCode Extension](extension.md)
