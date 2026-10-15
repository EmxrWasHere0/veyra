# Installing Veyra

Welcome to Veyra! This guide will help you install the tools required to run Veyra programs and get your development environment ready.

By the end of this guide, you will be able to build the Veyra interpreter, run a source file, and install Veyra as a command-line program.

## What You Need

To build Veyra from source, you need:

- **Git** — to download the repository.
- **Rust** — the programming language used to implement Veyra.
- **Cargo** — Rust's build and package manager. It is included with a standard Rust installation.
- **A terminal** — such as PowerShell, Windows Terminal, Terminal on macOS, or a Linux shell.

You do not need a separate compiler for your `.vey` programs. Veyra's interpreter executes them.

## Step 1: Install Rust and Cargo

Visit the official Rust installation website:

[https://rustup.rs/](https://rustup.rs/)

Follow the instructions for your operating system.

After installation, open a **new terminal** and check that Rust and Cargo are available:

```bash
rustc --version
cargo --version
```

Both commands should print version information.

If either command is not recognized, check your Rust installation and ensure the appropriate binary directory is included in your `PATH`.

## Step 2: Install Git

If Git is not already installed, download it from:

[https://git-scm.com/downloads](https://git-scm.com/downloads)

Verify the installation:

```bash
git --version
```

If you already have Git installed, you can continue to the next step.

## Step 3: Download the Veyra Repository

Clone the official Veyra repository:

```bash
git clone https://github.com/EmxrWasHere0/veyra.git
```

Move into the project directory:

```bash
cd veyra
```

You should now be in the directory containing the project's `Cargo.toml` file.

## Step 4: Build and Run Veyra

You can run the interpreter directly through Cargo without installing it globally.

From the repository root, execute:

```bash
cargo run -- --help
```

This command builds and launches the interpreter, passing the `--help` option to it.

To display the interpreter version, run:

```bash
cargo run -- --version
```

The first build may take a little time because Cargo needs to compile the project and its dependencies.

## Step 5: Run a Veyra Source File

Create a file named `hello.vey` in the repository root.

Add the following code:

```veyra
say "Hello, World!"
```

Run it with:

```bash
cargo run -- hello.vey
```

Expected output:

```text
Hello, World!
```

Congratulations! You have successfully run your first Veyra program.

## Step 6: Build an Optimized Interpreter

For an optimized build, run:

```bash
cargo build --release
```

The executable will be generated in the `target/release` directory.

Typical executable paths are:

- Linux: `target/release/veyra`
- macOS: `target/release/veyra`
- Windows: `target/release/veyra.exe`

You can run the executable directly.

On Linux or macOS:

```bash
./target/release/veyra hello.vey
```

On Windows PowerShell:

```powershell
.\target\release\veyra.exe hello.vey
```

Run these commands from the repository root, or adjust the paths if your source file is elsewhere.

**Important:** This builds the Veyra interpreter, not a standalone executable from your `.vey` source file.

## Step 7: Install Veyra as a Command-Line Tool

If you want to run Veyra without typing `cargo run` each time, install it using Cargo.

From the repository root, execute:

```bash
cargo install --path .
```

After installation, you can run programs using:

```bash
veyra hello.vey
```

You can also check the installed interpreter:

```bash
veyra --version
```

If the `veyra` command is not recognized, ensure Cargo's binary directory is included in your `PATH`. On many systems, this directory is `~/.cargo/bin`.

To update an existing local installation after making changes to the repository, run `cargo install --path .` again.

## Troubleshooting

### `cargo` is not recognized

Install Rust through [rustup](https://rustup.rs/), reopen your terminal, and check your `PATH`.

### Git cannot be found

Install Git from the [official download page](https://git-scm.com/downloads), then reopen your terminal.

### The source file cannot be found

Make sure your terminal is in the directory containing `hello.vey`, or provide the correct relative or absolute path.

### The program reports an error

Read the error message and check your Veyra syntax. If the error occurs during the build, check that Rust and Cargo are installed correctly.

## What's Next?

Now that Veyra is ready, continue to the [Quick Start](quickstart.md) guide to learn the basics of writing and running Veyra programs.

## Useful Links

- [Veyra GitHub Repository](https://github.com/EmxrWasHere0/veyra)
- [Rust Installation](https://rustup.rs/)
- [Git Downloads](https://git-scm.com/downloads)
