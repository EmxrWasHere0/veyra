# Learn Veyra

Welcome to the Veyra learning guide!

This section teaches Veyra step by step, from writing your first program to working with functions, collections, string interpolation, channels, modules, and error handling.

Veyra is a dynamically typed, interpreted programming language written in Rust.

This guide covers **Veyra 0.2.7**, including functional list methods and enhanced JSONData operations.

You can follow the lessons in order or jump directly to a topic using the navigation menu.

## Learning Path

The recommended learning order is:

1. **Getting Started** — Learn what Veyra is, install the interpreter, and write your first program.
2. **Language Basics** — Understand values, variables, operators, dynamic typing, and string interpolation.
3. **Control Flow** — Use conditions, loops, collection iteration, and labeled loop control.
4. **Collections** — Work with lists, slicing, functional list methods, and JSONData objects.
5. **Functions** — Learn parameters, recursion, lambdas, callbacks, and closed scopes.
6. **Input and Output** — Read user input, format output, and work with files.
7. **Channels** — Explore Veyra's runtime override mechanism.
8. **Modules** — Use built-in libraries and create `.veymod` modules.
9. **Error Handling** — Handle errors using `try`, `catch`, and `throw`.
10. **Developer Tools** — Use the REPL, test runner, and VSCode extension.

## What's New in Veyra 0.2.7?

Version 0.2.7 introduces three major language improvements.

### String Interpolation

Embed expressions directly inside strings.

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

### Functional List Methods

Process collections using callbacks.

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

Additional methods include `any()` and `all()`.

### JSONData Enhancements

Merge objects and retrieve key-value pairs.

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

JSONData objects can also be merged using the `+` operator.

## Lessons

```{toctree}
:caption: Learn Veyra
:maxdepth: 2

learning/what-is-veyra
learning/installation
learning/quickstart
learning/values
learning/variables
learning/operators
learning/control-flow
learning/lists
learning/functions
learning/io
learning/channels
learning/modules
learning/error-handling
learning/built-ins
learning/run
learning/extension
```

## Additional Resources

- [Documentation Home](index.md)
- [Language Overview](language-overview.md)
- [Features](features.md)
- [VSCode Extension](vscode-extension.md)
- [Project Repository](https://github.com/EmxrWasHere0/veyra)
- [Report an Issue](https://github.com/EmxrWasHere0/veyra/issues)
