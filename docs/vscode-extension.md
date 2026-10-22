# VSCode Extension

Veyra provides a Visual Studio Code extension for editing and running Veyra programs.

The extension supports Veyra 0.2.6 and includes syntax highlighting, editor configuration, code snippets, and integrated REPL access.

## Features

- **Syntax highlighting** for Veyra keywords, functions, modules, collections, and operators.
- **File support** for `.vey`, `.veyra`, `.vy`, `.veymod`, and `.wei`.
- **Automatic bracket closing** for `()`, `[]`, and `{}`.
- **Automatic quote closing** for single and double quotes.
- **Automatic indentation** for functions, conditionals, loops, and error-handling blocks.
- **Code folding** for supported block structures.
- **Code snippets** for functions, loops, imports, collections, and other common constructs.
- **Integrated REPL** through the `Veyra: Start REPL` command.

Triple-quoted strings (`"""..."""` and `'''...'''`) are also supported by the syntax highlighter.

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

This creates a VSIX package, typically named:

```text
veyra-0.2.6.vsix
```

Install it:

```bash
code --install-extension veyra-0.2.6.vsix
```

The generated filename depends on the version in `package.json`.

## Using the REPL

Veyra 0.2.6 includes an interactive REPL.

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
```

Type `exit` or `quit` to leave the REPL.

**Requirement:** The `veyra` executable must be installed and available in your system's `PATH`.

## Code Snippets

The extension includes shortcuts for common language constructs.

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

Module import snippets are also available, including `modmath`, `modjson`, `modtime`, `modfs`, and `modrandom`.

## Requirements

- Visual Studio Code 1.75.0 or newer.
- Veyra 0.2.6 or newer for the documented language features.
- The Veyra interpreter installed and available in `PATH` to use the integrated REPL.

Syntax highlighting and snippets work without the interpreter.

## Limitations

The extension currently does not include a Language Server Protocol (LSP) implementation.

Features such as semantic diagnostics, intelligent autocomplete, Go to Definition, and integrated debugging are not yet available.

## More Information

For complete installation instructions, snippet references, and examples, see the [Veyra VSCode Extension README](https://github.com/EmxrWasHere0/veyra/tree/main/veyra-vscode).
