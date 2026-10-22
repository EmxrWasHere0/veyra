# Installing Veyra

Welcome to Veyra!

This guide will help you install the Veyra interpreter, configure your development environment, and run your first program.

Veyra is an interpreted programming language written in Rust. You do not need to compile individual `.vey` files before running them.

## What You Need

To build or install Veyra from source, you need:

- **Rust** — The programming language used to implement Veyra.
- **Cargo** — Rust's build and package manager.
- **Git** — Used to download the Veyra repository.
- **A terminal** — PowerShell, Windows Terminal, macOS Terminal, or a Linux shell.

Cargo is included with standard Rust installations.

## 1. Install Rust and Cargo

Visit the official Rust installation website:

https://rustup.rs/

Follow the instructions for your operating system.

After installation, open a new terminal and verify both tools:

```bash
rustc --version
cargo --version
```

Both commands should display version information.

The minimum supported Rust version depends on the dependencies used by the current Veyra release.

## 2. Install Git

Download Git from:

https://git-scm.com/downloads

Verify the installation:

```bash
git --version
```

If Git is already installed, continue to the next step.

## P.S.

After these 2 steps, you can go to our [latest release](https://github.com/EmxrWasHere0/veyra/releases/latest) and use Veyra Installer.

## 3. Download Veyra

Clone the repository:

```bash
git clone https://github.com/EmxrWasHere0/veyra.git
```

Enter the project directory:

```bash
cd veyra
```

This directory contains `Cargo.toml` and the interpreter's Rust source code.

## 4. Run Veyra from Source

You can execute the interpreter through Cargo without installing it globally.

Display CLI help:

```bash
cargo run -- --help
```

Display the interpreter version:

```bash
cargo run -- --version
```

The first build may take some time because Cargo needs to compile the interpreter and its dependencies.

### Run a Source File

Create a file named `hello.vey`:

```veyra
say "Hello, World!"
```

Run it:

```bash
cargo run -- hello.vey
```

Output:

```text
Hello, World!
```

## 5. Build an Optimized Interpreter

For an optimized build:

```bash
cargo build --release
```

The executable will be located at:

| Platform | Executable                 |
| -------- | -------------------------- |
| Linux    | `target/release/veyra`     |
| macOS    | `target/release/veyra`     |
| Windows  | `target/release/veyra.exe` |

Run it directly on Linux or macOS:

```bash
./target/release/veyra hello.vey
```

On Windows PowerShell:

```powershell
.\target\release\veyra.exe hello.vey
```

This builds the Veyra interpreter, not a standalone executable from the Veyra source file.

## 6. Install Veyra Globally

To use Veyra from any directory, install it through Cargo.

From the repository root:

```bash
cargo install --path .
```

Alternatively, install directly from GitHub:

```bash
cargo install --git https://github.com/EmxrWasHere0/veyra.git
```

After installation, verify it:

```bash
veyra --version
```

Run a program:

```bash
veyra hello.vey
```

Cargo usually installs executables into its binary directory, such as `~/.cargo/bin/` on Linux and macOS.

Make sure this directory is available in your system's `PATH`.

### Updating Veyra

If you installed Veyra from a local repository, update the source code and reinstall:

```bash
git pull
cargo install --path . --force
```

For a Git-based installation:

```bash
cargo install --git https://github.com/EmxrWasHere0/veyra.git --force
```

## 7. Start the Interactive REPL

Veyra 0.2.6 includes an interactive REPL.

Start it by running:

```bash
veyra
```

You can enter Veyra statements directly:

```text
>>> add x = 10
>>> say x * 2
20
```

Variables and functions remain available during the session.

The REPL also supports multiline blocks:

```text
>>> add square(n)
...     return n * n
... end
>>> say square(5)
25
```

Type `exit` or `quit` to leave the REPL.

## 8. Running Tests

Veyra supports test blocks.

Run a Veyra test file:

```bash
veyra --test tests.vey
```

If you are developing the interpreter, run its Rust regression tests:

```bash
cargo test
```

The regression tests help verify that existing language features continue working after changes.

## 9. Optional: Install the VSCode Extension

Veyra provides a Visual Studio Code extension with syntax highlighting, snippets, and REPL integration.

To build it:

```bash
cd veyra-vscode
npm install
npx vsce package
```

Install the generated VSIX package:

```bash
code --install-extension veyra-0.2.6.vsix
```

The package filename depends on the extension version.

For details, see [VSCode Extension](extension.md).

## 10. Troubleshooting

### Cargo Is Not Recognized

Install Rust using [rustup](https://rustup.rs/).

Reopen your terminal and check:

```bash
cargo --version
```

### Git Is Not Recognized

Install Git and reopen your terminal.

Verify it using:

```bash
git --version
```

### Veyra Is Not Recognized

Check that the interpreter was installed successfully.

Make sure Cargo's binary directory is included in `PATH`.

You can also run the interpreter directly from the project directory:

```bash
cargo run -- hello.vey
```

### Source File Not Found

Check the current working directory and the file path.

Use an absolute path if necessary.

### Build Fails

Make sure Rust and Cargo are up to date:

```bash
rustup update stable
```

Then retry the build:

```bash
cargo build
```

### Runtime Error

Veyra reports source locations and function tracebacks for runtime errors.

Read the error message and check the indicated source line.

For more information, see [Error Handling](error-handling.md).

## What's Next?

Continue learning Veyra:

- [Quick Start](quickstart.md)
- [Running Veyra Programs](run.md)
- [Variables and Types](variables.md)
- [VSCode Extension](extension.md)

## Useful Links

- [Veyra GitHub Repository](https://github.com/EmxrWasHere0/veyra)
- [Rust Installation](https://rustup.rs/)
- [Git Downloads](https://git-scm.com/downloads)
