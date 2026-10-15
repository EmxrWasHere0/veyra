# Running Veyra Programs

Veyra is an interpreted programming language. Its interpreter reads and executes Veyra source files, allowing you to run programs without compiling each source file into a standalone executable.

This guide explains how to install the Veyra interpreter and run `.vey` programs using different methods.

## Requirements

To build or install Veyra from source, you need:

- [Rust 1.70 or newer](https://rustup.rs/)
- Cargo, which is included with a standard Rust installation
- A terminal or command-line interface

## Method 1: Run Veyra from Source

Running Veyra from source is useful when developing the language or testing changes to the interpreter.

### 1. Clone the repository

```bash
git clone https://github.com/EmxrWasHere0/veyra.git
```

### 2. Navigate to the project directory

```bash
cd veyra
```

### 3. Run a Veyra program

Pass the source file path to Cargo using `--`:

```bash
cargo run -- hello.vey
```

To run an example from the repository:

```bash
cargo run -- examples/hello.vey
```

Cargo builds the interpreter if necessary and then passes the specified file to it.

### 4. Run an optimized build

For optimized execution, use:

```bash
cargo run --release -- hello.vey
```

The release profile is optimized for running the interpreter, while the default development profile is generally more convenient when making changes to the source code.

## Method 2: Build the Interpreter

You can build the Veyra interpreter without immediately running a program.

From the repository root, execute:

```bash
cargo build --release
```

The resulting executable is located in the `target/release` directory. Its filename depends on the operating system:

- Linux: `target/release/veyra`
- macOS: `target/release/veyra`
- Windows: `target\release\veyra.exe`

You can use the executable directly by passing it a source file.

For example, on Linux or macOS:

```bash
./target/release/veyra hello.vey
```

On Windows PowerShell:

```powershell
.\target\release\veyra.exe hello.vey
```

Run these commands from the repository root, or adjust the paths to match your current directory.

## Method 3: Install Veyra Globally

If you want to use Veyra without navigating to its source repository every time, install it as a Cargo package.

### Install from a local clone

From the repository root, run:

```bash
cargo install --path .
```

### Install directly from GitHub

You can also install Veyra directly from its Git repository:

```bash
cargo install --git https://github.com/EmxrWasHere0/veyra.git
```

After installation, run a source file with:

```bash
veyra hello.vey
```

Cargo normally installs the executable in `~/.cargo/bin/` on Linux and macOS, or the corresponding Cargo binary directory on Windows. Make sure this directory is included in your `PATH`.

To uninstall Veyra if it was installed through Cargo:

```bash
cargo uninstall veyra
```

## Writing Your First Program

Create a file named `hello.vey` with the following contents:

```veyra
say "Hello, World!"

add greet(name)
    say "Hello, " + name + "!"
end

greet("Veyra")
```

Run the program:

```bash
veyra hello.vey
```

If you are running from the source repository instead, use:

```bash
cargo run -- hello.vey
```

Expected output:

```text
Hello, World!
Hello, Veyra!
```

## Running Files from Different Directories

You can pass a relative or absolute path to the interpreter.

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

Replace these example paths with the actual location of your source file.

## Passing Command-Line Arguments

Veyra provides the `system.args()` function for accessing command-line arguments passed to a running program.

For example:

```veyra
mod system

args = system.args()
say args
```

Run the program with additional arguments:

```bash
veyra hello.vey first second
```

The arguments can be accessed through `system.args()`. The interpreter's implementation excludes the interpreter and script paths from the returned list.

## Choosing a Running Method

| Method                            | Best for                                                 |
| --------------------------------- | -------------------------------------------------------- |
| `cargo run -- file.vey`           | Development and quick testing                            |
| `cargo run --release -- file.vey` | Running from source with release optimizations           |
| Built executable                  | Running the interpreter without Cargo on each invocation |
| `cargo install`                   | Using Veyra as a command from different directories      |

## Troubleshooting

### `cargo` is not recognized

Install Rust using [rustup](https://rustup.rs/) and ensure Cargo is available in your terminal's `PATH`.

### `veyra` is not recognized

If you installed Veyra using Cargo, check that Cargo's binary directory is in your `PATH`. Alternatively, run the interpreter from the source repository using `cargo run`.

### The source file cannot be found

Check the filename, file extension, and current working directory. Provide the correct relative or absolute path if necessary.

### The program reports a runtime error

Check the source code and the error message printed by the interpreter. Confirm that the program uses valid Veyra syntax and that any required files or modules are available.

## Further Reading

- [Veyra GitHub Repository](https://github.com/EmxrWasHere0/veyra)
- [Rust Installation Guide](https://rustup.rs/)
