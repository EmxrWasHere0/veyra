## Installation

### Requirements

- [Rust](https://rustup.rs/) 1.70 or newer

### Option 1: Run from Source (`cargo run`)

Clone the repository and run programs directly through Cargo. Best for development and tinkering with the interpreter itself.

```bash
git clone https://github.com/EmxrWasHere0/veyra.git
cd veyra
cargo build --release
```

Run a `.vey` file:

```bash
cargo run --release -- examples/hello.vey
```

Or without `--release` (slower, but faster to compile during development):

```bash
cargo run -- examples/hello.vey
```

> The `--` separates Cargo's arguments from the arguments passed to Veyra.

### Option 2: Install Globally (`cargo install`)

Install Veyra as a system-wide command. Best if you just want to **use** the language.

From the repository root:

```bash
cargo install --path .
```

Or directly from a Git URL (no clone needed):

```bash
cargo install --git https://github.com/EmxrWasHere0/veyra.git
```

This places the `veyra` binary into `~/.cargo/bin/` (which should already be in your `PATH` if you installed Rust via `rustup`).

Verify installation:

```bash
veyra --version    # (if version flag is supported)
veyra              # prints usage
```

Run a program:

```bash
veyra hello.vey
```

To uninstall:

```bash
cargo uninstall veyra
```
