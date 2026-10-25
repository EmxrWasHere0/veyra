# Installation

Veyra is an interpreted programming language written in Rust.

You can install the interpreter using Cargo, build it directly from source, or download an installer from the GitHub releases page.

This guide covers installation for Veyra 0.2.7.

## Requirements

To build Veyra, you need:

- [Rust and Cargo](https://rustup.rs/)
- [Git](https://git-scm.com/downloads) (for cloning or installing from the repository)
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

On Windows, the default location is typically `%USERPROFILE%\.cargo\bin`.

Make sure Cargo's binary directory is included in your system's `PATH`.

### Verify Installation

Check the installed version:

```bash
veyra --version
```

For Veyra 0.2.7, the interpreter should report version `0.2.7`.

The displayed version is determined by the package version in `Cargo.toml` at build time.

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

You can run the executable directly.

Linux/macOS:

```bash
./target/release/veyra examples/hello.vey
```

Windows PowerShell:

```powershell
.\target\release\veyra.exe examples\hello.vey
```

## Option 3: Use the Veyra Installer

Prebuilt executables or installers may be available on the GitHub releases page.

Visit the [latest Veyra release](https://github.com/EmxrWasHere0/veyra/releases/latest) to check the available downloads.

Choose the package appropriate for your operating system.

After installation, verify that the `veyra` command is available:

```bash
veyra --version
```

Availability of installers may vary by release and platform.

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
>>> say f"Double: {x * 2}"
Double: 20
```

The REPL preserves variables, functions, and imports throughout the session.

Leave the REPL using `exit` or `quit`.

### Running Tests

Execute Veyra test blocks:

```bash
veyra --test tests.vey
```

You can also use the short option:

```bash
veyra -t tests.vey
```

If you are developing the interpreter, run its automated regression tests:

```bash
cargo test
```

These tests verify interpreter behavior using Veyra programs.

## Updating Veyra

To update an installation made directly from GitHub, run:

```bash
cargo install --git https://github.com/EmxrWasHere0/veyra.git --force
```

For a local source checkout:

```bash
git pull
cargo install --path . --force
```

After updating, check the installed version:

```bash
veyra --version
```

For version 0.2.7, the expected package version is `0.2.7`.

## Uninstalling

If Veyra was installed using Cargo:

```bash
cargo uninstall veyra
```

If Veyra was installed using a platform-specific installer, follow the uninstall instructions provided with that package.

## Next Steps

- [Quick Start](quick-start.md)
- [Running Veyra Programs](learning/run.md)
- [VSCode Extension](vscode-extension.md)
