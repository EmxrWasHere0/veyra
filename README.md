[![Typing SVG](https://readme-typing-svg.demolab.com?font=Fira+Code&pause=1000&color=6F18F7&width=435&lines=%24+veyra+--test+readme.vey)](https://git.io/typing-svg)

<div align="center">
<img src="media/veyra-banner.png">
</div>

A small, dynamically-typed, interpreted programming language with easy-to-understand keywords and a unique **channel** mechanism for runtime value overrides.

```veyra
set port = 8080
port = 3000             # the constant doesn't change — the channel does

add connect(target)
    say target          # 8080  (constant value)
    say 0 ? target      # 3000  (channel value, carried with the argument)
end

connect(port)
```

---

## Table of Contents

- [Why Veyra?](#why-veyra)
- [Features](#features)
- [Installation](#installation)
  - [Option 1: Run from Source (`cargo run`)](#option-1-run-from-source-cargo-run)
  - [Option 2: Install Globally (`cargo install`)](#option-2-install-globally-cargo-install)
- [Quick Start](#quick-start)
- [Language Overview](#language-overview)
  - [Values](#values)
  - [Variables and Constants](#variables-and-constants)
  - [Channels](#channels)
  - [I/O](#io)
  - [Operators](#operators)
  - [Control Flow](#control-flow)
  - [Lists](#lists)
  - [Functions](#functions)
  - [Built-ins](#built-ins)
  - [Error Handling](#error-handling)
  - [Modules](#modules)
- [VSCode Extension](#vscode-extension)
- [Project Layout](#project-layout)
- [Limitations](#limitations)
- [Future Plans](#future-plans)
- [License](#license)

---

## Why Veyra?

Veyra started as a learning project built around a simple question:

**What if a programming language could make runtime overrides explicit without losing the original value?**

That idea became Veyra's channel mechanism — a way for a name to hold a stable default and a separate runtime override at the same time.

But Veyra is more than just channels.

- **Explicit runtime overrides** — Channels let programs distinguish between a value's original definition and its runtime override.
- **Predictable function scope** — Functions use a closed-scope model, preventing accidental access to surrounding local variables.
- **Readable syntax** — A small set of distinctive keywords keeps the language approachable while still supporting functions, recursion, lists, and structured control flow.
- **Room to experiment** — Veyra is a playground for exploring programming language design, runtime behavior, and new ideas without the complexity of a large language ecosystem.

Veyra isn't trying to replace established programming languages. It's an experiment in building something small, understandable, and interesting — one feature at a time.

**Small language. Different ideas. Built for curiosity.**

---

## Features

- **Dynamic typing** with 6 value types (number, text, bool, list, function, null)
- **First-class functions**, lambdas, recursion, and nested definitions
- **Default parameters** (which can be expressions referencing earlier parameters)
- **Variadic parameters** (`...args`) in any position
- **Closed-scope functions** — no accidental access to outer variables
- **Channels** — the `? ` operator for runtime overrides
- **Line-based syntax** — no significant whitespace, blocks closed with `end`
- **Zero dependencies** — Implemented in Rust with zero external dependencies
- **Multiple return values** and list destructuring, including rest elements
- **Conditional assignment** with `?=`
- **Null coalescing** with `??`
- **Labeled loop control** for targeting outer loops
- **Inline testing** with `test` blocks and `assert`
- **Built-in system and I/O modules** for environment, file, directory, and process operations

---

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

---

## Quick Start

Create a file called `hello.vey`:

```veyra
# hello.vey — your first Veyra program

say "Hello, World!"

add greet(name, greeting = "Hi")
    greeting + ", " + name + "!"
end

say greet("Alex")
say greet("Alice", "Hello")

# A tiny counter
i = 1
repeat while i <= 3
    say i
    i += 1
end
```

Run it:

```bash
# With cargo:
cargo run --release -- hello.vey

# After cargo install:
veyra hello.vey
```

Expected output:

```
Hello, World!
Hi, Alex!
Hello, Alice!
1
2
3
```

---

## Language Overview

### Values

| Type       | Example                               |
| ---------- | ------------------------------------- |
| `number`   | `42`, `3.14`, `-7`                    |
| `text`     | `"hello"`, `'single quotes work too'` |
| `bool`     | `true`, `false`                       |
| `list`     | `[1, 2, 3]`, `[[1,2],[3,4]]`          |
| `function` | user-defined or lambda                |
| `null`     | absence of value                      |

Falsy values: `0`, `""`, `[]`, `null`, `false`. Everything else is truthy.

### Variables and Constants

Regular variables are created by assignment and are **dynamically typed**:

```veyra
x = 5
x = "now a string"
x = [1, 2, 3]
```

Constants are declared with `set` and **never change**:

```veyra
set PI = 3.14159
set MAX = 100
```

### Channels

Writing to a constant does **not** modify it — instead it updates a parallel **channel** of the same name:

```veyra
set port = 80       # constant = 80
port = 8080         # channel["port"] = 8080

say port            # 80    (constant)
say 0 ? port        # 8080  (channel)
```

The `?` operator reads from a channel with a required fallback:

```
default ? channel_name
```

- Returns the **channel value** if it exists
- Returns the **default** otherwise

Channels **follow arguments** when passed to a function:

```veyra
set port = 80
port = 8080

add show(c)
    say c           # 80    (the constant value)
    say 0 ? c       # 8080  (channel tag "port" carried via argument)
end

show(port)
```

### I/O

```veyra
say "Hello"              # print with newline
say 1 + 2                # expressions work
say                      # empty line

get "Your name: "        # prompt + read line
say "Hi, " + _input      # input is always stored in _input (as text)
```

To convert input to a number:

```veyra
get "Age: "
age = num(_input)
say age + 1
```

### Operators

| Category               | Operators          |
| ---------------------- | ------------------ |
| Arithmetic             | `+ - * / %`        |
| Comparison             | `== != < > <= >=`  |
| Logical                | `and or not !`     |
| Assignment             | `= += -= *= /= %=` |
| Channel                | `?`                |
| Lambda arrow           | `=>`               |
| Spread                 | `...`              |
| Conditional assignment | `?=`               |
| Null coalescing        | `??`               |

`+` also concatenates strings: `"n=" + 42` → `"n=42"`.

### Control Flow

**Conditionals:**

```veyra
if age >= 18
    say "adult"
else if age >= 13
    say "teen"
else
    say "child"
end
```

**Counting loop:**

```veyra
repeat 5
    say "hi"
end
```

**Conditional loop:**

```veyra
i = 0
repeat while i < 10
    if i == 5
        stop         # break
    end
    if i % 2 == 0
        i += 1
        fwd          # continue
    end
    say i
    i += 1
end
```

- `stop` → exit the loop
- `fwd` → next iteration

`stop` and `fwd` are **only valid inside loops**. In functions, use `return`.

Safety limit: **1,000,000 iterations** per loop.

**Labeled loops:**

Labels let you control an outer loop from inside a nested loop.

```veyra
@outer repeat 3
    repeat 3
        stop @outer
    end
end
```

Use `stop @label` to exit the labeled loop and `fwd @label` to continue with the next iteration of that loop.

See [`examples/stage2_test.vy`](examples/stage2_test.vy) for more examples.

### Lists

```veyra
nums = [10, 20, 30]
say nums[0]         # 10
nums[1] = 99
say nums            # [10, 99, 30]
say len(nums)       # 3

matrix = [[1,2],[3,4]]
say matrix[1][0]    # 3
```

### Functions

Defined with `add`:

```veyra
add add_two(a, b)
    a + b           # last expression returns automatically
end

say add_two(3, 4)   # 7
```

**Early return:**

```veyra
add sign(n)
    if n > 0
        return "positive"
    end
    if n < 0
        return "negative"
    end
    "zero"
end
```

**Default parameters** (can reference earlier parameters):

```veyra
add range(start, stop = start + 10)
    stop - start
end
say range(5)        # 10
```

**Variadic** (`...` in any position; becomes a list):

```veyra
add sum_all(...nums)
    t = 0
    i = 0
    repeat while i < len(nums)
        t += nums[i]
        i += 1
    end
    t
end
say sum_all(1, 2, 3, 4, 5)   # 15
```

**Lambdas** (single-expression anonymous functions):

```veyra
square = func(x) => x * x
say square(9)       # 81

add apply(f, x, y)
    f(x, y)
end
say apply(func(a, b) => a - b, 10, 3)   # 7
```

**Closed scope** — functions cannot see outer variables:

```veyra
x = 100
add try()
    x + 1           # ERROR: Undefined variable: x
end
```

Only parameters, locals, other `add` functions (globally registered), and built-ins are visible inside a function. This makes functions predictable and side-effect-free with respect to outer state.

Call stack limit: **1,000** frames.

### Error Handling

Veyra supports `try` / `catch` blocks for handling runtime errors.

```veyra
try
    x = 10 / 0
catch err
    say "caught: " + err
end
```

The catch variable is optional:

```veyra
try
    x = undefined_variable
catch
    say "Something went wrong"
end
```

You can also use `??` to provide a fallback when a value is `null`.

```veyra
value = null
say value ?? "fallback"
```

See [`examples/stage2_test.vy`](examples/stage2_test.vy) for nested error handling and additional examples.

### Built-ins

| Function  | Description                                                                 |
| --------- | --------------------------------------------------------------------------- |
| `len(x)`  | Length of a list or text                                                    |
| `num(x)`  | Convert to number                                                           |
| `text(x)` | Convert to text                                                             |
| `type(x)` | Returns `"number"`, `"text"`, `"bool"`, `"list"`, `"function"`, or `"null"` |

### Modules

Veyra includes built-in modules for interacting with the operating system, environment, files, directories, and external processes.

**Available modules:**

- **`system`** — Operating system information, environment variables, command-line arguments, and process exit control.
- **`io`** — File and directory operations, working directory management, and external command execution.
- **`base`** — Basic system information, including hostname, username, home directory, process ID, and CPU core count.

Import a module with `mod`:

```veyra
mod system
mod io
```

Import all exported names from `base`:

```veyra
mod base *
say hostname()
say pid()
```

Import selected names:

```veyra
mod system [get_env, args]
say get_env("HOME")
```

For more examples, see [`examples/mod_test.vy`](examples/mod_test.vy).

**Note:** Functions that access files, environment variables, or external processes interact with the host system.

---

## VSCode Extension

Syntax highlighting, snippets, and basic language configuration are available as a separate extension. See [`veyra-vscode`](veyra-vscode) in this repository or install from source:

```bash
cd veyra-vscode
npm install -g @vscode/vsce
vsce package
code --install-extension veyra-0.1.0.vsix
```

---

## Project Layout

```
veyra/
├── Cargo.toml
├── src/
│   ├── main.rs
│   ├── lexer.rs
│   ├── runtime.rs
│   └── modules/
│       ├── base.rs
│       ├── io.rs
│       ├── mod.rs
│       └── system.rs
├── examples/
├── media/
├── veyra-vscode/
└── README.md
```

---

## Limitations

Veyra is intentionally small. The following are **not** supported (yet):

- Dictionary / map type
- List mutation helpers (`push`, `pop`, `slice`, `map`, `filter`)
- Extended standard library (`math`, `str`, `time`, `json`, `random`, etc.)
- String interpolation (`f"hello {name}"`)
- Pattern matching
- Real closures (functions capture no outer state — this is a deliberate design choice)
- User-defined modules in `.veymod` files _(planned)_

Contributions and ideas are welcome.

---

## Future Plans

Veyra is an experimental language, and there is still plenty of room to grow. Here are some of the features and improvements planned for future releases.

- [ ] **Improved error messages** — Clearer runtime errors with line numbers and source context.
- [ ] **Standard library expansion** — Additional modules for math, strings, time, JSON, and random number generation.
- [ ] **Better module system** — Support for importing user-defined modules from separate files.
- [ ] **List utilities** — Built-in helpers for common operations such as `push`, `pop`, `slice`, `map`, and `filter`.
- [ ] **Improved testing tools** — Better test reporting and a dedicated way to run test blocks.
- [ ] **Language documentation** — Comprehensive documentation covering syntax, built-ins, modules, and examples.
- [ ] **VS Code improvements** — Better diagnostics, autocomplete, and editor integration.
- [ ] **Performance improvements** — Optimize the interpreter and runtime as the language grows.
- [ ] **More examples and tutorials** — Practical projects to help users learn Veyra.
- [ ] **Hybrid Language** — Support both interpreted execution and compilation, allowing Veyra programs to run through the interpreter or be compiled into standalone executables.

These plans may change as Veyra evolves. Contributions, suggestions, and feature requests are welcome!

---

## License

GNU GPLv3 © 2026 — See [LICENSE](LICENSE) for details.

---

This project contains support by variable AI models.
