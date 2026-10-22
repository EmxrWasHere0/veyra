# VS Code Extension

The Veyra VS Code extension provides language support for writing Veyra programs in Visual Studio Code.

It includes syntax highlighting, automatic indentation, code snippets, code folding, and integrated REPL support.

For a quick overview, see [VSCode Extension](../vscode-extension.md).

## What You Will Learn

By the end of this guide, you will know how to:

- Install the Veyra extension.
- Open and edit Veyra source files.
- Use code snippets.
- Start the Veyra REPL from VS Code.
- Configure basic editor behavior.
- Troubleshoot common extension problems.

## 1. Supported Files

The extension recognizes the following extensions:

| Extension | Description                                   |
| --------- | --------------------------------------------- |
| `.vey`    | Standard Veyra source files                   |
| `.veyra`  | Alternative Veyra source extension            |
| `.vy`     | Alternative Veyra source extension            |
| `.veymod` | User-defined Veyra modules                    |
| `.wei`    | Additional extension recognized by the editor |

When opening a supported file, VS Code automatically selects the Veyra language mode.

If it does not, select **Veyra** manually from the language mode selector in the bottom-right corner.

## 2. Installation

### Build from Source

Clone the Veyra repository:

```bash
git clone https://github.com/EmxrWasHere0/veyra.git
cd veyra/veyra-vscode
```

Install the required dependencies:

```bash
npm install
```

Build the VSIX package:

```bash
npx vsce package
```

This generates a file named:

```text
veyra-0.2.6.vsix
```

Install it:

```bash
code --install-extension veyra-0.2.6.vsix
```

The generated filename depends on the version in `package.json`.

### Install Using VS Code

You can also install the generated VSIX file through the editor:

1. Open Visual Studio Code.
2. Open the Extensions view.
3. Click the `...` menu.
4. Select **Install from VSIX...**.
5. Choose the generated `.vsix` file.

Reload VS Code if the extension does not activate immediately.

## 3. Syntax Highlighting

The extension highlights Veyra syntax, including:

- Keywords and control-flow statements.
- Variables, constants, and functions.
- Operators and module imports.
- Lists and JSON-like objects.
- Built-in functions and modules.
- Comments and strings.
- Triple-quoted multiline strings.

Example:

```veyra
mod math

add square(x)
    return x * x
end

add numbers = [2, 4, 6]

repeat number in numbers
    say square(number)
end
```

Comments begin with `#`.

The extension uses TextMate grammar rules. It does not perform complete semantic analysis.

## 4. Automatic Editing Features

The extension provides language configuration for common editing operations.

### Automatic Bracket Closing

Supported pairs include:

```text
()
[]
{}
```

### Automatic Quote Closing

Single and double quotes are supported:

```veyra
add name = "Veyra"
add message = 'Hello'
```

Triple-quoted strings are recognized by the syntax highlighter:

```veyra
add message = """
Hello from Veyra!
This is a multiline string.
"""
```

Automatic completion of triple quotes may vary depending on VS Code behavior.

### Indentation

The extension provides indentation rules for:

- `if`, `else if`, and `else`.
- `repeat` and `while`.
- Named function declarations.
- `try` and `catch`.
- Labeled `repeat` loops.

Example:

```veyra
add greet(name)
    if name == "Veyra"
        say "Hello, Veyra!"
    else
        say "Hello, " + name
    end
end
```

### Code Folding

VS Code can collapse and expand supported code blocks.

This is useful when working with long functions, nested loops, and conditional statements.

## 5. Code Snippets

Snippets provide reusable templates for common Veyra constructs.

Type a prefix and select the corresponding suggestion.

### Language Snippets

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
| `func`        | Define a named function    |
| `lambda`      | Create a lambda expression |
| `return`      | Return a value             |
| `try`         | Create a try-catch block   |
| `throw`       | Throw an error             |
| `test`        | Create a test block        |
| `assert`      | Assert a condition         |
| `list`        | Create a list              |
| `jsondata`    | Create a JSON-like object  |
| `multistr`    | Create a multiline string  |

### Module Snippets

| Prefix      | Description                 |
| ----------- | --------------------------- |
| `mod`       | Import a module             |
| `mods`      | Import all exported members |
| `modi`      | Import selected members     |
| `modsys`    | Import `system`             |
| `modio`     | Import `io`                 |
| `modbase`   | Import `base`               |
| `modjson`   | Import `json`               |
| `modhttp`   | Import `http`               |
| `modws`     | Import `websocket`          |
| `modmath`   | Import `math`               |
| `modtime`   | Import `time`               |
| `modfs`     | Import `fs`                 |
| `modrandom` | Import `random`             |

Additional import variants are available for the `system`, `io`, and `base` modules.

### Using a Snippet

1. Open a Veyra source file.
2. Type a snippet prefix, such as `repeatin`.
3. Select the snippet from the suggestion list.
4. Press `Tab` or `Enter`, depending on your editor settings.
5. Fill in the placeholders.

For example, the `repeatin` snippet generates:

```veyra
repeat item in collection

end
```

You can then replace the placeholders and write the loop body.

## 6. Integrated REPL

Veyra 0.2.6 includes an interactive Read-Eval-Print Loop (REPL).

The extension provides a command to start it directly from VS Code.

### Starting the REPL

1. Open the Command Palette:
   - Windows/Linux: `Ctrl+Shift+P`
   - macOS: `Cmd+Shift+P`
2. Search for `Veyra: Start REPL`.
3. Select the command.

VS Code opens a terminal named **Veyra REPL** and runs:

```bash
veyra
```

### Using the REPL

Example session:

```text
>>> add x = 10
>>> say x * 2
20
>>> add square(n)
...     return n * n
... end
>>> say square(5)
25
```

The REPL preserves variables, functions, and imports during the session.

Type `exit` or `quit` to leave.

### Interpreter Requirement

The REPL command requires the Veyra interpreter to be installed and available in your system's `PATH`.

Verify installation:

```bash
veyra --version
```

If the command is not recognized, install the interpreter or add its executable directory to `PATH`.

## 7. Running Veyra Programs

The extension currently provides a REPL command rather than a dedicated script execution command.

To run a `.vey` file, open a terminal:

```bash
veyra hello.vey
```

To execute Veyra test blocks:

```bash
veyra --test tests.vey
```

For more information, see [Running Veyra](run.md).

## 8. Troubleshooting

### Syntax Highlighting Does Not Work

Check that the file has a supported extension.

If necessary:

1. Open the language mode selector.
2. Select **Veyra**.
3. Reload the VS Code window.

### Snippets Do Not Appear

Make sure the file is recognized as Veyra.

You can also check the VS Code setting:

```json
{
  "editor.snippetSuggestions": "top"
}
```

### REPL Command Is Missing

Make sure version 0.2.6 of the extension is installed.

Try reloading the VS Code window.

If necessary, reinstall the VSIX package.

### REPL Does Not Start

Open a terminal and run:

```bash
veyra --version
```

If the executable cannot be found, verify that Veyra is installed and its directory is available in `PATH`.

### A Script Produces an Error

Run the script directly:

```bash
veyra example.vey
```

Veyra 0.2.6 reports source locations and function tracebacks for runtime errors.

## 9. Current Limitations

The extension does not currently provide:

- Language Server Protocol (LSP) support.
- Semantic diagnostics.
- Intelligent code completion.
- Go to Definition.
- Symbol renaming.
- Integrated debugging.

Syntax highlighting and snippets work without installing the interpreter, but running programs requires Veyra.

## What's Next?

Continue with:

- [Running Veyra](run.md)
- [Quick Start](quickstart.md)
- [Modules](modules.md)
- [Error Handling](error-handling.md)

For extension source code and additional information, visit the [Veyra VS Code Extension repository](https://github.com/EmxrWasHere0/veyra/tree/main/veyra-vscode).
