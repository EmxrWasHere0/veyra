# Why Veyra?

Veyra started as a learning project built around a simple question:

**What if a programming language could make runtime overrides explicit without losing the original value?**

That idea became Veyra's **channel mechanism** — a way to preserve a constant's original value while maintaining a separate runtime override.

Today, Veyra is a dynamically typed, interpreted programming language written in Rust, with functions, collections, modules, error handling, and developer tools.

## Design Philosophy

Veyra is built around a few core ideas:

- **Explicit runtime overrides** — Channels distinguish original values from their runtime overrides.
- **Predictable function scopes** — Closed-scope functions avoid implicitly capturing surrounding local variables.
- **Readable syntax** — Distinctive keywords and `end`-based blocks keep code approachable.
- **Practical experimentation** — Functions, collections, modules, and runtime tools provide room to explore language design.
- **Incremental development** — Features are introduced and refined as the language evolves.

## What Makes Veyra Different?

Consider a constant with a runtime override:

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

The original constant remains unchanged, while its channel stores a separate value.

This behavior is intentionally different from ordinary mutable variables.

Channel information can also travel through function arguments, allowing functions to access both the original value and its associated override.

For a detailed explanation, see [Channels](learning/channels.md).

## Growing Language Features

Veyra continues to expand while keeping its experimental design.

Version 0.2.7 introduces string interpolation, functional list methods, and enhanced JSONData operations.

### String Interpolation

Expressions can be embedded directly inside strings.

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

### Functional Collections

Lists support callback-based operations.

```veyra
add numbers = [1, 2, 3, 4, 5]

say numbers.map(func(x) => x * 2)
say numbers.filter(func(x) => x % 2 == 0)
```

Output:

```text
[2, 4, 6, 8, 10]
[2, 4]
```

Additional methods include `reduce()`, `any()`, and `all()`.

### JSONData Objects

Veyra supports JSON-like objects with indexing, methods, and merging.

```veyra
add user = {"name": "Alex", "age": 20}
add updates = {"age": 21}

say user.merge(updates)
```

Output:

```text
{"name": "Alex", "age": 21}
```

These features complement Veyra's existing functions, modules, channels, and control-flow constructs.

For a complete overview, see [Features](features.md).

## An Experimental Language

Veyra is not intended to replace established programming languages.

It is a project for exploring interpreter design, runtime behavior, and alternative programming language concepts.

The language continues to evolve through experimentation, implementation, and testing.

**Small language. Different ideas. Built for curiosity.**
