# Why Veyra?

Veyra started as a learning project built around a simple question:

**What if a programming language could make runtime overrides explicit without losing the original value?**

That idea became Veyra's **channel mechanism** — a way to preserve a constant's original value while maintaining a separate runtime override.

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

For a detailed explanation, see [Channels](learning/channels.md).

## An Experimental Language

Veyra is not intended to replace established programming languages.

It is a project for exploring interpreter design, runtime behavior, and alternative programming language concepts.

**Small language. Different ideas. Built for curiosity.**
