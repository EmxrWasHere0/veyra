# What Is Veyra?

<div align="center">
  <img src="https://raw.githubusercontent.com/EmxrWasHere0/veyra/refs/heads/main/media/veyra-banner.png" alt="Veyra Banner">
</div>

Veyra is a dynamically typed, interpreted programming language written in Rust.

It uses readable keywords, simple block structures, and a unique channel mechanism for runtime value overrides.

Veyra is an experimental language developed to explore programming language design.

## How Does Veyra Work?

Veyra programs are written in source files such as `hello.vey`.

The interpreter reads and executes these files directly.

For example:

```veyra
say "Hello, World!"
```

Run the program using:

```bash
veyra hello.vey
```

Output:

```text
Hello, World!
```

You can also start an interactive REPL by running:

```bash
veyra
```

## What Can Veyra Do?

Veyra supports several programming features:

- **Dynamic typing** — Variables can store different value types.
- **Functions** — Named functions, lambdas, recursion, and default parameters.
- **Collections** — Lists and JSON-like objects.
- **Control flow** — Conditions, loops, and collection iteration.
- **Error handling** — `try`, `catch`, and `throw`.
- **Modules** — Built-in Rust modules and user-defined `.veymod` modules.
- **Developer tools** — Interactive REPL, automated tests, and VSCode support.

## Channels

One of Veyra's distinctive features is its channel mechanism.

Channels allow constants to preserve their original values while holding separate runtime overrides.

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

The first expression reads the constant.

The second reads its channel value, using `0` as a fallback when no override exists.

Channels can also be carried through function arguments.

For details, see [Channels](channels.md).

## Why Learn Veyra?

Veyra provides an opportunity to explore:

- How interpreted programming languages work.
- Dynamic typing and runtime evaluation.
- Functions and scope rules.
- Module systems.
- Alternative approaches to variable and constant behavior.

It is designed as a learning and experimentation project rather than a replacement for established languages.

## What's Next?

Start learning Veyra:

- [Installation](installation.md)
- [Quick Start](quickstart.md)
- [Values](values.md)
- [Variables and Types](variables.md)
- [Channels](channels.md)
