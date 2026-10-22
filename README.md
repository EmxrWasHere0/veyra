[![Typing SVG](https://readme-typing-svg.demolab.com?font=Fira+Code&pause=1000&color=6F18F7&width=435&lines=%24+veyra+--test+readme.vey)](https://git.io/typing-svg)

<div align="center">
  <img src="media/veyra-banner.png" alt="Veyra Banner">
</div>

<div align="center">

# Veyra

**A small language. Different ideas. Built for curiosity.**

A dynamically typed, interpreted programming language written in Rust.

[Documentation](docs/) · [VSCode Extension](veyra-vscode/) · [Examples](examples/) · [Issues](https://github.com/EmxrWasHere0/veyra/issues)

</div>

---

## What is Veyra?

Veyra is an experimental programming language designed around readable syntax, predictable function scopes, and a unique **channel mechanism**.

Channels allow a constant to retain its original value while holding a separate runtime override.

```veyra
set port = 8080
port = 3000

say port        # 8080
say 0 ? port    # 3000
```

Channel values can also travel through function arguments:

```veyra
set port = 8080
port = 3000

add connect(target)
    say target        # 8080
    say 0 ? target    # 3000
end

connect(port)
```

Veyra started as a learning project and continues to evolve as an experiment in programming language design.

## Features

- **Dynamic typing** — Numbers, strings, booleans, lists, objects, functions, and null.
- **Channels** — Explicit runtime overrides without changing constant values.
- **Functions** — Recursion, lambdas, default arguments, variadic parameters, and multiple return values.
- **Closed scopes** — Functions do not implicitly capture surrounding local variables.
- **Control flow** — Conditionals, loops, collection iteration, and labeled loops.
- **Collections** — Lists, slicing, indexing, and JSON-like objects.
- **Error handling** — `try`, `catch`, and `throw`.
- **Modules** — Built-in Rust modules and user-defined `.veymod` modules.
- **Testing** — Test blocks, assertions, and automated regression tests.
- **REPL** — Interactive execution with persistent variables and functions.
- **Developer tools** — Detailed runtime errors, function tracebacks, and VSCode support.

## Installation

### Requirements

- [Rust and Cargo](https://rustup.rs/)

### Install from Git

```bash
cargo install --git https://github.com/EmxrWasHere0/veyra.git
```

Or clone and build from source:

```bash
git clone https://github.com/EmxrWasHere0/veyra.git
cd veyra

cargo build --release
```

To install the local version globally:

```bash
cargo install --path .
```

Verify the installation:

```bash
veyra --version
```

> [!INFO]
> You can also install Veyra using Veyra Installer. Check out our [latest release](https://github.com/EmxrWasHere0/veyra/releases/latest) to find the installer for your operating system.

## Quick Start

Create a file named `hello.vey`:

```veyra
say "Hello, World!"

add greet(name)
    return "Hello, " + name + "!"
end

say greet("Veyra")
```

Run it:

```bash
veyra hello.vey
```

Expected output:

```text
Hello, World!
Hello, Veyra!
```

### Interactive REPL

Start Veyra without arguments:

```bash
veyra
```

Example:

```text
>>> add x = 10
>>> say x * 2
20
>>> add square(n)
...     return n * n
... end
>>> say square(5)
25
```

Type `exit` or `quit` to leave the REPL.

## Built-in Modules

Veyra includes built-in modules implemented in Rust.

| Module      | Purpose                            |
| ----------- | ---------------------------------- |
| `system`    | Operating system and environment   |
| `io`        | Input/output and process execution |
| `base`      | System information                 |
| `json`      | JSON parsing and serialization     |
| `http`      | HTTP requests                      |
| `websocket` | WebSocket connections              |
| `math`      | Mathematical operations            |
| `time`      | Date and time utilities            |
| `fs`        | Filesystem operations              |
| `random`    | Random number generation           |

Modules can be imported using `mod`:

```veyra
mod math

say math.sqrt(144)
```

Veyra also supports user-defined `.veymod` modules.

## Documentation

Detailed language documentation is maintained separately from this README.

- [Language Documentation](https://veyra.readthedocs.io)
- [Code Examples](examples/)
- [VSCode Extension](veyra-vscode/)

The documentation covers syntax, functions, channels, collections, modules, error handling, and other language features.

## VSCode Extension

Veyra provides a Visual Studio Code extension with:

- Syntax highlighting
- Automatic indentation and bracket closing
- Code folding
- Code snippets
- Integrated REPL command

Build and install the extension:

```bash
cd veyra-vscode
npm install
npx vsce package
code --install-extension veyra-0.2.6.vsix
```

You can then open the Command Palette and run:

**Veyra: Start REPL**

See the [extension README](veyra-vscode/README.md) for details.

> [!INFO]
> You can also install the VSCode extension from our [latest release](https://github.com/EmxrWasHere0/veyra/releases/latest).

## Development

Run the automated tests:

```bash
cargo test
```

Build an optimized interpreter:

```bash
cargo build --release
```

Run a Veyra test file:

```bash
veyra --test tests.vey
```

## Contributing

Veyra is an experimental project, and contributions are welcome.

You can report bugs, suggest features, or contribute improvements through [GitHub Issues](https://github.com/EmxrWasHere0/veyra/issues).

## License

Veyra is licensed under the [GNU General Public License v3.0](LICENSE).

Copyright © 2026.

---

This project was developed with assistance from AI models.
