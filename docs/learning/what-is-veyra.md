# What Is Veyra?

<div align="center">
  <img src="https://raw.githubusercontent.com/EmxrWasHere0/veyra/refs/heads/main/media/veyra-banner.png" alt="Veyra Banner">
</div>

Veyra is a dynamically typed, interpreted programming language written in Rust.

It uses readable keywords, simple block structures, and a unique channel mechanism for runtime value overrides.

Veyra is an experimental language developed to explore programming language design.

The current version is **0.2.7**.

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

The REPL allows you to execute Veyra statements without creating a source file.

## What Can Veyra Do?

Veyra supports several programming features:

- **Dynamic typing** — Variables can store different value types.
- **Functions** — Named functions, lambdas, recursion, default parameters, and variadic arguments.
- **Collections** — Lists, slicing, functional list methods, and JSONData objects.
- **String interpolation** — Embed expressions directly inside strings.
- **Control flow** — Conditions, loops, collection iteration, and labeled loop control.
- **Error handling** — `try`, `catch`, `throw`, and function tracebacks.
- **Modules** — Built-in Rust modules and user-defined `.veymod` modules.
- **Developer tools** — Interactive REPL, automated tests, and VSCode support.

## String Interpolation

Veyra 0.2.7 introduces string interpolation.

Prefix a string with `f` to embed expressions using `{expression}`.

```veyra
add name = "Alex"
add age = 20

say f"Hello, {name}!"
say f"Next year: {age + 1}"
```

Output:

```text
Hello, Alex!
Next year: 21
```

Interpolation supports expressions, function calls, and multiline strings.

## Functional Lists

Veyra provides methods for processing lists using callback functions.

```veyra
add numbers = [1, 2, 3, 4, 5]

say numbers.map(func(x) => x * 2)
say numbers.filter(func(x) => x % 2 == 0)
say numbers.reduce(func(acc, x) => acc + x, 0)
```

Output:

```text
[2, 4, 6, 8, 10]
[2, 4]
15
```

Additional functional methods include `any()` and `all()`.

## JSONData Objects

Veyra provides a `jsondata` type for key-value collections.

```veyra
add user = {"name": "Alex", "age": 20}

say user["name"]
say user["age"]
```

Output:

```text
Alex
20
```

Veyra 0.2.7 introduces JSONData merging and key-value pair retrieval.

```veyra
add user = {"name": "Alex", "age": 20}
add updates = {"age": 21}

say user.merge(updates)
say user.items()
```

Output:

```text
{"name": "Alex", "age": 21}
[["name", "Alex"], ["age", 20]]
```

JSONData objects can also be merged using `+`.

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
- Functions, callbacks, and scope rules.
- Module systems.
- Collection operations and expression evaluation.
- Alternative approaches to variable and constant behavior.

It is designed as a learning and experimentation project rather than a replacement for established languages.

## What's Next?

Start learning Veyra:

- [Installation](installation.md)
- [Quick Start](quickstart.md)
- [Values](values.md)
- [Variables and Types](variables.md)
- [Channels](channels.md)
