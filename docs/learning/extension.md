# VS Code Extension

The **Veyra VS Code extension** provides language support for writing Veyra programs in Visual Studio Code. It improves the editing experience with syntax highlighting, automatic bracket and quote closing, code folding, and built-in code snippets.

The extension supports the following file extensions:

- `.vey`
- `.veyra`
- `.vy`

## Features

### Syntax Highlighting

The extension highlights Veyra language syntax to make code easier to read and understand.

It provides syntax highlighting for supported language constructs, including keywords, operators, comments, and other syntax patterns defined by the extension's grammar.

```vey
add square(x)
    return x * x
end

# Calculate a square
say square(5)
```

Comments use the `#` character.

### Automatic Bracket and Quote Closing

When you type an opening bracket or quote, VS Code can automatically insert its matching closing character.

For example, typing an opening parenthesis can produce a matching pair:

```vey
say("Hello, world!")
```

This feature helps reduce typing and prevents common delimiter mistakes.

### Code Folding

Code folding allows you to collapse and expand sections of code in the editor.

The extension defines folding support for common Veyra constructs, including:

- `if` blocks
- `repeat` loops
- `add` function definitions

Folding is useful when working with large functions or deeply nested control-flow structures.

### Code Snippets

Snippets are reusable code templates that can be inserted by typing a prefix and selecting a suggestion from the editor.

The extension includes the following snippets:

| Prefix   | Description                                    |
| -------- | ---------------------------------------------- |
| `if`     | Conditional statement                          |
| `ife`    | Conditional with an `else` branch              |
| `ifee`   | Conditional with `else if` and `else` branches |
| `repeat` | Repetition loop                                |
| `rwhile` | Conditional loop                               |
| `add`    | Function definition                            |
| `func`   | Lambda expression                              |
| `set`    | Constant declaration                           |
| `say`    | Output statement                               |
| `get`    | User input                                     |

To use a snippet:

1. Open a Veyra source file.
2. Place the cursor where you want to insert code.
3. Type a snippet prefix, such as `add`.
4. Select the desired snippet from the completion menu.
5. Fill in any editable placeholders.

Snippets help you write common language constructs more quickly and consistently.

## Installation

### Option 1: Install from a VSIX Package

The repository includes a packaged extension named `veyra-0.2.0.vsix`.

1. Open Visual Studio Code.
2. Open the **Extensions** view.
3. Open the three-dot menu (`...`).
4. Select **Install from VSIX...**.
5. Select `veyra-0.2.0.vsix` from the repository's `veyra-vscode` directory.
6. Wait for installation to finish.

You can also install the package from a terminal:

```bash
code --install-extension veyra-0.2.0.vsix
```

Run this command from the directory containing the VSIX file, or provide the correct path to the package.

You can find the extension files in the [Veyra repository](https://github.com/EmxrWasHere0/veyra/tree/main/veyra-vscode).

### Option 2: Build from Source

To build your own package or modify the extension, install Node.js and npm first.

Clone the repository and navigate to the extension directory:

```bash
git clone https://github.com/EmxrWasHere0/veyra.git
cd veyra/veyra-vscode
```

Install the dependencies:

```bash
npm install
```

Install the VS Code extension packaging tool if it is not already available:

```bash
npm install -g @vscode/vsce
```

Package the extension:

```bash
vsce package
```

This creates a `.vsix` package in the current directory. Its filename depends on the extension's name and version in `package.json`.

Install the resulting package with:

```bash
code --install-extension veyra-0.2.0.vsix
```

If you changed the extension version, replace the example filename with the actual generated filename.

## Requirements

- Visual Studio Code version `1.75.0` or later, according to the extension manifest.
- The Veyra interpreter, if you want to execute Veyra programs.

The extension provides editor language support. Running a program requires the Veyra interpreter to be installed and available in your environment.

## Current Scope

The current extension configuration provides syntax highlighting, snippets, automatic bracket and quote closing, and code folding.

Features such as semantic diagnostics, go-to-definition, a dedicated debugger, and a language server are not declared in the current extension manifest. Do not assume these features are available unless they are implemented in a later version.

## Troubleshooting

### Syntax Highlighting Is Not Working

1. Make sure the file uses `.vey`, `.veyra`, or `.vy`.
2. Open the language mode selector in the bottom-right corner of VS Code.
3. Select **Veyra** if the language mode was not detected automatically.
4. If necessary, reload the VS Code window.

### Snippets Are Not Appearing

1. Confirm that the file is recognized as Veyra.
2. Type one of the supported snippet prefixes.
3. Check the editor's suggestion settings and ensure that suggestions are enabled.

### The Program Will Not Run

The extension's syntax highlighting does not itself execute Veyra programs.

Make sure the Veyra interpreter is installed and that you are using the correct command for your interpreter version.

## Contributing

Contributions, bug reports, and suggestions are welcome. To improve the extension, visit the [Veyra GitHub repository](https://github.com/EmxrWasHere0/veyra) and review the `veyra-vscode` directory.

When adding new features, update the extension manifest, grammar, snippets, or language configuration as appropriate, and keep this documentation consistent with the implementation.
