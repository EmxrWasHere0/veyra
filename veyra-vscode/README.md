<div align="center">
  <img src="https://raw.githubusercontent.com/EmxrWasHere0/veyra/refs/heads/main/veyra-vscode/icons/veyra-vscode.png">
</div>

Language support for **Veyra**, featuring syntax highlighting, automatic bracket and quote closing, code folding, and built-in snippets.

## Features

- **Syntax highlighting** for `.vey` files
- **Comment support** using `#`
- **Automatic bracket and quote closing**
- **Code folding** for `if`, `repeat`, and `add` blocks
- **Built-in snippets** for common language constructs:
  - `if` — Conditional statement
  - `ife` — Conditional with an `else` branch
  - `ifee` — Conditional with `else if` and `else` branches
  - `repeat` — Repetition loop
  - `rwhile` — Conditional loop
  - `add` — Function definition
  - `func` — Lambda expression
  - `set` — Constant declaration
  - `say` — Output statement
  - `get` — User input

## Installation

### Install from VSIX

The packaged extension is available as [`veyra-0.2.3.vsix`](veyra-0.2.3.vsix), if the file is included in the repository.

**Using the command line:**

```bash
code --install-extension veyra-0.2.3.vsix
```

**Using the VS Code interface:**

1. Open Visual Studio Code.
2. Open the **Extensions** view.
3. Click the three-dot menu (`...`) next to the search bar.
4. Select **Install from VSIX...**.
5. Choose `veyra-0.2.3.vsix`.
6. Follow any prompts to complete installation.

### Build from source

If you want to modify the extension or package a new version:

```bash
cd veyra-vscode
npm install
npm install -g @vscode/vsce
vsce package
```

This creates a `.vsix` package in the extension directory. Install it with:

```bash
code --install-extension veyra-0.2.3.vsix
```

If you change the extension version in `package.json`, the generated VSIX filename may also change.

## Requirements

- Visual Studio Code
- The Veyra language interpreter to run `.vey` programs

## Contributing

Found a bug or have an idea for improving Veyra's language support? Contributions and suggestions are welcome!
