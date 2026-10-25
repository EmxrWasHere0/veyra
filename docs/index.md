# Veyra Documentation

Welcome to the official documentation for **Veyra**, a dynamically typed, interpreted programming language written in Rust.

Veyra combines readable syntax, closed-scope functions, flexible modules, and a unique **channel mechanism** for explicit runtime value overrides.

**Current version: 0.2.7**

<div align="center">
  <img
    src="https://raw.githubusercontent.com/EmxrWasHere0/veyra/refs/heads/main/media/veyra-banner.png"
    alt="Veyra Banner"
  >
</div>

## A Quick Example

Channels allow constants to preserve their original values while storing separate runtime overrides.

```veyra
set port = 8080
port = 3000

say port
say 0 ? port
```

Output:

```text
8080
3000
```

For a detailed explanation, see [Channels](learning/channels.md).

## What's New in Veyra 0.2.7?

Version 0.2.7 introduces string interpolation, functional list methods, and enhanced JSONData operations.

```veyra
add name = "Alex"
add numbers = [1, 2, 3, 4]

say f"Hello, {name}!"
say numbers.map(func(x) => x * 2)

add user = {"name": "Alex", "age": 20}
add updates = {"age": 21}

say user.merge(updates)
```

Output:

```text
Hello, Alex!
[2, 4, 6, 8]
{"name": "Alex", "age": 21}
```

Highlights:

- **String interpolation** — Embed expressions using `f"Hello, {name}!"`.
- **Functional list methods** — `map()`, `filter()`, `reduce()`, `any()`, and `all()`.
- **JSONData enhancements** — `merge()`, `items()`, and object merging with `+`.

For a complete overview, see [Features](features.md).

## Getting Started

Install Veyra using Cargo:

```bash
cargo install --git https://github.com/EmxrWasHere0/veyra.git
```

Run a program:

```bash
veyra hello.vey
```

Or start the interactive REPL:

```bash
veyra
```

Example:

```text
>>> add x = 10
>>> say f"Double: {x * 2}"
Double: 20
```

See [Installation](installation.md) and [Quick Start](quick-start.md) for more information.

## Explore Veyra

Veyra supports:

- **Functions** — Recursion, lambdas, default parameters, and variadic arguments.
- **Channels** — Separate runtime overrides for constants.
- **Collections** — Lists, slicing, functional methods, and JSONData objects.
- **String interpolation** — Embedded expressions and multiline formatted strings.
- **Control flow** — Conditionals, loops, collection iteration, and labeled loops.
- **Modules** — Built-in Rust modules and user-defined `.veymod` modules.
- **Error handling** — `try`, `catch`, `throw`, source locations, and function tracebacks.
- **Developer tools** — Interactive REPL, automated tests, and VSCode integration.

For a complete overview, see [Features](features.md).

## Documentation

```{toctree}
:caption: Documentation
:maxdepth: 2

why-veyra
features
installation
quick-start
language-overview
learn
vscode-extension
project-layout
limitations
future-plans
license
```

## Start Learning

If you are new to Veyra, begin with the [Learning Guide](learn.md).

For a quick reference, visit the [Language Overview](language-overview.md).

You can also explore:

- [Built-in Functions and Modules](learning/built-ins.md)
- [Lists and Functional Methods](learning/lists.md)
- [Channels](learning/channels.md)
- [VSCode Extension](vscode-extension.md)
- [Running Veyra Programs](learning/run.md)

## Project

Veyra is an experimental programming language developed as a learning project.

Source code, examples, and issue tracking are available on [GitHub](https://github.com/EmxrWasHere0/veyra).
