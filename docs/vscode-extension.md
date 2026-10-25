# VSCode Extension

Veyra provides a Visual Studio Code extension for editing and running Veyra programs.

The extension supports Veyra 0.2.7 and includes syntax highlighting, editor configuration, code snippets, and integrated REPL access.

## Features

- **Syntax highlighting** for Veyra keywords, functions, modules, collections, and operators.
- **String interpolation highlighting** for `f"..."`, `f'...'`, `f"""..."""`, and `f'''...'''`.
- **Expression highlighting** inside interpolated strings using `{expression}`.
- **Functional list method highlighting** for `map()`, `filter()`, `reduce()`, `any()`, and `all()`.
- **JSONData method highlighting** for `merge()` and `items()`.
- **File support** for `.vey`, `.veyra`, `.vy`, `.veymod`, and `.wei`.
- **Automatic bracket closing** for `()`, `[]`, and `{}`.
- **Automatic quote closing** for single and double quotes.
- **Automatic indentation** for functions, conditionals, loops, and error-handling blocks.
- **Code folding** for supported block structures.
- **Code snippets** for functions, loops, imports, collections, string interpolation, and other common constructs.
- **Integrated REPL** through the `Veyra: Start REPL` command.

Triple-quoted strings (`"""..."""` and `'''...'''`) are also supported by the syntax highlighter.

## String Interpolation Support

Veyra 0.2.7 introduces interpolated strings.

The VSCode extension highlights interpolated string boundaries and embedded expressions.

Example:

```veyra
add name = "Alex"
add age = 20

say f"Hello, {name}!"
say f"Next year: {age + 1}"
```

Multiline interpolation is also supported:

```veyra
add language = "Veyra"

say f"""
Language: {language}
Version: 0.2.7
"""
```

Literal braces can be written using `{{` and `}}`:

```veyra
add value = 42

say f"Result: {{ {value} }}"
```

Syntax highlighting is based on TextMate grammar rules rather than full semantic analysis.

Some complex nested interpolation expressions may not be highlighted perfectly.

## Installation

### Build from Source

Clone the Veyra repository:

```bash
git clone https://github.com/EmxrWasHere0/veyra.git
cd veyra/veyra-vscode
```

Install the extension dependencies:

```bash
npm install
```

Package the extension:

```bash
npx vsce package
```

This creates a VSIX package named:

```text
veyra-0.2.7.vsix
```

Install it:

```bash
code --install-extension veyra-0.2.7.vsix
```

The generated filename depends on the version in `package.json`.

## Using the REPL

Veyra includes an interactive REPL, introduced in version 0.2.6.

To start it from VSCode:

1. Open the Command Palette with `Ctrl+Shift+P` or `Cmd+Shift+P`.
2. Search for `Veyra: Start REPL`.
3. Select the command.

The extension opens a terminal and executes:

```bash
veyra
```

You can then enter Veyra statements interactively.

```text
>>> add x = 10
>>> say x * 2
20
>>> say f"Double: {x * 2}"
Double: 20
```

You can also experiment with functional list methods:

```text
>>> add numbers = [1, 2, 3, 4]
>>> say numbers.map(func(x) => x * 2)
[2, 4, 6, 8]
```

Type `exit` or `quit` to leave the REPL.

**Requirement:** The `veyra` executable must be installed and available in your system's `PATH`.

## Code Snippets

The extension includes shortcuts for common language constructs.

### General Snippets

| Prefix        | Description                |
| ------------- | -------------------------- |
| `add`         | Declare a variable         |
| `set`         | Declare a constant         |
| `func`        | Define a function          |
| `lambda`      | Create a lambda            |
| `if`          | Create a conditional block |
| `ifelse`      | Create an if-else block    |
| `while`       | Create a conditional loop  |
| `repeat`      | Create a counted loop      |
| `repeatin`    | Iterate over a collection  |
| `repeatlabel` | Create a labeled loop      |
| `try`         | Create a try-catch block   |
| `throw`       | Raise a runtime error      |
| `jsondata`    | Create a JSON-like object  |
| `multistr`    | Create a multiline string  |

### String Interpolation Snippets

| Prefix      | Description                            |
| ----------- | -------------------------------------- |
| `fstr`      | Create an interpolated string          |
| `fmultistr` | Create a multiline interpolated string |

Example:

Typing `fstr` and accepting the snippet inserts an interpolated string template.

```veyra
f"Hello, {name}!"
```

### Functional List Snippets

| Prefix   | Description                       |
| -------- | --------------------------------- |
| `map`    | Transform list elements           |
| `filter` | Filter list elements              |
| `reduce` | Reduce a list to a single value   |
| `any`    | Check whether any element matches |
| `all`    | Check whether all elements match  |

Example:

```veyra
add numbers = [1, 2, 3, 4]

say numbers.map(func(x) => x * 2)
say numbers.filter(func(x) => x % 2 == 0)
say numbers.reduce(func(acc, x) => acc + x, 0)
```

### JSONData Snippets

| Prefix  | Description                  |
| ------- | ---------------------------- |
| `merge` | Merge two JSONData objects   |
| `items` | Get JSONData key-value pairs |

Example:

```veyra
add user = {"name": "Alex"}
add updates = {"age": 20}

say user.merge(updates)
say user.items()
```

Module import snippets are also available, including `modmath`, `modjson`, `modtime`, `modfs`, and `modrandom`.

## Requirements

- Visual Studio Code 1.75.0 or newer.
- Veyra 0.2.7 or newer for all documented language features.
- The Veyra interpreter installed and available in `PATH` to use the integrated REPL.

Syntax highlighting and snippets work without the interpreter.

## Limitations

The extension currently does not include a Language Server Protocol (LSP) implementation.

Features such as semantic diagnostics, intelligent autocomplete, Go to Definition, and integrated debugging are not yet available.

Syntax highlighting is based on TextMate grammar rules and may not accurately identify every nested expression or runtime type.

## More Information

For complete installation instructions, snippet references, and examples, see the [Veyra VSCode Extension README](https://github.com/EmxrWasHere0/veyra/tree/main/veyra-vscode).
