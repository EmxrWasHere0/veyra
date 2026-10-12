# Veyra

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
- [VSCode Extension](#vscode-extension)
- [Project Layout](#project-layout)
- [Limitations](#limitations)
- [License](#license)

---

## Why Veyra?

Veyra is a toy language built as a learning project, but it has one real idea worth exploring:

> **A name can hold both a frozen default and a mutable runtime override at the same time.**

This is the "channel" mechanism — a built-in override pattern that works like config files or environment variables, but inside the language itself. Combined with closed-scope functions, you get predictable code where overrides are explicit, not hidden.

---

## Features

- **Dynamic typing** with 6 value types (number, text, bool, list, function, null)
- **First-class functions**, lambdas, recursion, and nested definitions
- **Default parameters** (which can be expressions referencing earlier parameters)
- **Variadic parameters** (`...args`) in any position
- **Closed-scope functions** — no accidental access to outer variables
- **Channels** — the `? ` operator for runtime overrides
- **Line-based syntax** — no significant whitespace, blocks closed with `end`
- **Zero dependencies** — the interpreter is pure Rust, under 1500 lines

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

| Category     | Operators          |
| ------------ | ------------------ |
| Arithmetic   | `+ - * / %`        |
| Comparison   | `== != < > <= >=`  |
| Logical      | `and or not !`     |
| Assignment   | `= += -= *= /= %=` |
| Channel      | `?`                |
| Lambda arrow | `=>`               |
| Spread       | `...`              |

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

### Built-ins

| Function  | Description                                                                 |
| --------- | --------------------------------------------------------------------------- |
| `len(x)`  | Length of a list or text                                                    |
| `num(x)`  | Convert to number                                                           |
| `text(x)` | Convert to text                                                             |
| `type(x)` | Returns `"number"`, `"text"`, `"bool"`, `"list"`, `"function"`, or `"null"` |

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
│   └── main.rs              # the entire interpreter (~1500 lines)
├── examples/
│   ├── hello.vey
│   ├── fib.vey
│   └── full_test.vey
├── editors/
│   └── vscode/              # VSCode syntax extension
└── README.md
```

---

## Limitations

Veyra is intentionally small. The following are **not** supported (yet):

- Dictionary / map type
- List mutation helpers (`push`, `pop`, `slice`, `map`, `filter`)
- Exception handling (`try` / `catch`)
- Module / import system
- File I/O
- Standard library beyond the handful of built-ins
- String interpolation (`f"hello {name}"`)
- Pattern matching
- Real closures (functions capture no outer state — this is a deliberate design choice)

Contributions and ideas are welcome.

---

## License

GNU GPLv3 © 2026 — See [LICENSE](LICENSE) for details.
