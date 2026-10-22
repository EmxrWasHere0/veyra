# Installation

Veyra is an interpreted programming language written in Rust.

You can install the interpreter using Cargo or build it directly from source.

## Requirements

To build Veyra, you need:

- [Rust and Cargo](https://rustup.rs/)
- Git (for cloning or installing from the repository)
- A supported operating system

The minimum Rust version depends on the dependencies used by the current Veyra release.

## Option 1: Install with Cargo

Install Veyra directly from GitHub:

```bash
cargo install --git https://github.com/EmxrWasHere0/veyra.git
```

Alternatively, clone the repository:

```bash
git clone https://github.com/EmxrWasHere0/veyra.git
cd veyra
```

Install the local version:

```bash
cargo install --path .
```

Cargo installs the `veyra` executable in its binary directory, usually `~/.cargo/bin/` on Linux and macOS.

Make sure Cargo's binary directory is included in your system's `PATH`.

### Verify Installation

Check the installed version:

```bash
veyra --version
```

Display command-line help:

```bash
veyra --help
```

## Option 2: Run from Source

Clone the repository:

```bash
git clone https://github.com/EmxrWasHere0/veyra.git
cd veyra
```

Run a Veyra program:

```bash
cargo run -- examples/hello.vey
```

For an optimized build:

```bash
cargo run --release -- examples/hello.vey
```

The `--` separates Cargo arguments from arguments passed to Veyra.

### Build an Executable

Build the interpreter:

```bash
cargo build --release
```

The executable will be located at:

- Linux/macOS: `target/release/veyra`
- Windows: `target/release/veyra.exe`

## Option 3: Use Veyra Installer

You can install Veyra using the installers given in our [latest release](https://github.com/EmxrWasHere0/veyra/releases/latest)

## Running Veyra

Execute a source file:

```bash
veyra hello.vey
```

Start the interactive REPL:

```bash
veyra
```

Example:

```text
>>> add x = 10
>>> say x * 2
20
```

Leave the REPL using `exit` or `quit`.

### Running Tests

Execute Veyra test blocks:

```bash
veyra --test tests.vey
```

If you are developing the interpreter, run its automated regression tests:

```bash
cargo test
```

## Uninstalling

If Veyra was installed using Cargo:

```bash
cargo uninstall veyra
```

## Next Steps

- [Quick Start](quick-start.md)
- [Running Veyra Programs](learning/run.md)
- [VSCode Extension](vscode-extension.md)
