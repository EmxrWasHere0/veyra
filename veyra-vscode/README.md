<div align="center">
  <img src="https://raw.githubusercontent.com/EmxrWasHere0/veyra/refs/heads/main/veyra-vscode/icons/veyra-vscode.png" alt="Veyra logo">
</div>

# Veyra for Visual Studio Code

Language support for **Veyra**, a dynamically typed, interpreted programming language written in Rust.

This extension provides syntax highlighting, automatic bracket and quote closing, code folding, code snippets, and integrated REPL support.

## Features

### Syntax Highlighting

Syntax highlighting for Veyra source files:

- `.vey`
- `.veyra`
- `.vy`
- `.veymod`
- `.wei`

Supported language constructs include:

- Variables and constants: `add`, `set`
- Functions and lambdas: `add name(...)`, `func(...) =>`
- Conditionals: `if`, `else`
- Loops: `repeat`, `while`, `repeat item in collection`
- Labeled loops: `@outer repeat`, `stop @outer`, `fwd @outer`
- Error handling: `try`, `catch`, `throw`
- Tests and assertions: `test`, `assert`
- Module imports: `mod`
- Lists, JSON-like objects, and multiline strings
- Built-in modules and common methods

### Editor Support

- Line comments using `#`
- Automatic closing for `()`, `[]`, and `{}`
- Automatic closing for single and double quotes
- Indentation rules for blocks
- Code folding
- Built-in code snippets

Triple-quoted strings (`"""..."""` and `'''...'''`) are supported by the syntax highlighter.

### Code Snippets

Type a snippet prefix and press `Tab` or select it from the suggestion list.

| Prefix        | Description                |
| ------------- | -------------------------- |
| `add`         | Declare a mutable variable |
| `set`         | Declare a constant         |
| `say`         | Print a value              |
| `get`         | Read user input            |
| `if`          | Create an if block         |
| `ifelse`      | Create an if-else block    |
| `while`       | Create a while loop        |
| `repeat`      | Create a counted loop      |
| `repeatin`    | Iterate over a collection  |
| `repeatlabel` | Create a labeled loop      |
| `stop`        | Stop a loop                |
| `fwd`         | Continue a loop            |
| `func`        | Define a function          |
| `lambda`      | Create a lambda expression |
| `return`      | Return a value             |
| `try`         | Create a try-catch block   |
| `throw`       | Throw an error             |
| `test`        | Create a test block        |
| `assert`      | Assert a condition         |
| `list`        | Create a list              |
| `jsondata`    | Create a JSON-like object  |
| `multistr`    | Create a multiline string  |

Module import snippets are also available:

| Prefix      | Module                    |
| ----------- | ------------------------- |
| `mod`       | Custom module             |
| `mods`      | Import all module members |
| `modi`      | Import selected members   |
| `modsys`    | `system`                  |
| `modio`     | `io`                      |
| `modbase`   | `base`                    |
| `modjson`   | `json`                    |
| `modhttp`   | `http`                    |
| `modws`     | `websocket`               |
| `modmath`   | `math`                    |
| `modtime`   | `time`                    |
| `modfs`     | `fs`                      |
| `modrandom` | `random`                  |

Additional snippet variants are available for importing all or selected members from the `system`, `io`, and `base` modules.

### Integrated REPL

Veyra 0.2.6 introduces an interactive REPL.

To start it from Visual Studio Code:

1. Open the Command Palette (`Ctrl+Shift+P` or `Cmd+Shift+P`).
2. Search for `Veyra: Start REPL`.
3. Select the command.

A new terminal named **Veyra REPL** will open and execute:

```bash
veyra
```

The REPL supports persistent variables, functions, imports, and multiline blocks.

Example:

```veyra
>>> add x = 10
>>> say x * 2
20
>>> add greet(name)
...     say "Hello, " + name
... end
>>> greet("Veyra")
Hello, Veyra
```

Type `exit` or `quit` to leave the REPL.

**Important:** The Veyra interpreter must be installed and available in your system's `PATH` for this command to work.

## Veyra Language Examples

### Variables and Functions

```veyra
add name = "Veyra"
set version = "0.2.6"

add greet(name)
    say "Hello, " + name
end

greet(name)
```

### Collection Loops

```veyra
add numbers = [10, 20, 30]

repeat number in numbers
    say number
end
```

### JSON-like Objects

```veyra
add user = {
    "name": "Veyra",
    "version": "0.2.6",
    "active": true
}

say user["name"]
say user.keys()
```

### Error Handling

```veyra
try
    throw "Example error"
catch error
    say error
end
```

### Multiline Strings

```veyra
add message = """
Hello from Veyra!
This is a multiline string.
"""

say message
```

## Installation

### Install from VSIX

Download or build the extension package:

```text
veyra-0.2.6.vsix
```

Install it using the command line:

```bash
code --install-extension veyra-0.2.6.vsix
```

Alternatively, install it through Visual Studio Code:

1. Open the Extensions view.
2. Click the `...` menu.
3. Select **Install from VSIX...**.
4. Choose `veyra-0.2.6.vsix`.

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

This generates:

```text
veyra-0.2.6.vsix
```

Install it:

```bash
code --install-extension veyra-0.2.6.vsix
```

The generated filename depends on the version specified in `package.json`.

## Requirements

- Visual Studio Code 1.75.0 or newer.
- Veyra 0.2.6 or newer for the documented language features.
- The `veyra` executable available in `PATH` to use the integrated REPL.

Syntax highlighting and snippets work without installing the interpreter.

## Limitations

This extension currently provides language configuration, syntax highlighting, snippets, and REPL integration.

It does not currently include:

- Language Server Protocol (LSP) support
- Semantic diagnostics
- Intelligent code completion
- Go to Definition
- Symbol renaming
- Integrated debugging

Syntax highlighting is based on TextMate grammar rules rather than full semantic analysis.

## Contributing

Bug reports, feature requests, and contributions are welcome.

Repository:

https://github.com/EmxrWasHere0/veyra

## License

GPL-3.0-only
