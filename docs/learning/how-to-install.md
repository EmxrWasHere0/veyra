# How to Install?

To install Veyra, you _only_ need one program:

- [Rust](https://rustup.rs) version 1.70 or newer

You have 2 options to run Veyra:

## 1: From Source

To run Veyra from source, you need to clone the repository first:

```bash
git clone https://github.com/EmxrWasHere0/veyra.git
cd veyra
```

After cloning the repository, you need to run:

```bash
cargo run -- /path/to/file.vey
```

to run your code directly.

## 2: Install as Binary

To run Veyra as a binary (executable), you need to clone the repository first:

```bash
git clone https://github.com/EmxrWasHere0/veyra.git
cd veyra
```

After cloniing the repository, you can simply run:

```bash
cargo install --path .
```

to install Veyra as a binary (executable).

**Note**: Make sure to add the path Cargo gives you to your PATH environment.
