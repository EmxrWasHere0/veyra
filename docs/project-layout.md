# Project Layout

Veyra 0.2.7 is implemented in Rust.

The interpreter's core is organized around `main.rs`, `lexer.rs`, and `runtime.rs`, while built-in modules are maintained separately.

This structure keeps the interpreter compact and allows built-in libraries, tests, documentation, and editor tooling to evolve independently.

## Directory Structure

```text
veyra/
├── Cargo.toml
├── Cargo.lock
├── src/
│   ├── main.rs
│   ├── lexer.rs
│   ├── runtime.rs
│   └── modules/
│       ├── mod.rs
│       ├── base.rs
│       ├── system.rs
│       ├── io.rs
│       ├── json.rs
│       ├── http.rs
│       ├── websocket.rs
│       ├── math.rs
│       ├── time.rs
│       ├── fs.rs
│       └── random.rs
├── tests/
│   ├── cli-tests.rs
│   └── fixtures/
│       └── *.vey
├── examples/
├── docs/
│   ├── conf.py
│   ├── requirements.txt
│   ├── index.md
│   ├── features.md
│   ├── language-overview.md
│   ├── learn.md
│   ├── project-layout.md
│   └── learning/
│       └── *.md
├── veyra-vscode/
│   ├── package.json
│   ├── extension.js
│   ├── language-configuration.json
│   ├── syntaxes/
│   │   └── veyra.tmLanguage.json
│   ├── snippets/
│   │   └── veyra.json
│   └── README.md
├── media/
├── README.md
└── LICENSE
```

## Interpreter

The interpreter is implemented primarily in three Rust source files.

### `src/main.rs`

Responsible for:

- Command-line argument handling.
- Executing Veyra source files.
- Starting the interactive REPL.
- Processing CLI options such as `--help`, `--version`, and `--test`.
- Displaying runtime errors.

The interactive REPL was introduced in Veyra 0.2.6.

### `src/lexer.rs`

Responsible for:

- Tokenizing Veyra source code.
- Recognizing keywords, operators, identifiers, and literals.
- Parsing source lines.
- Recognizing single-quoted, double-quoted, and triple-quoted strings.
- Recognizing interpolated strings.

Veyra 0.2.7 introduces the `InterpolatedString` token for syntax such as:

```veyra
say f"Hello, {name}!"
```

Supported interpolation formats include:

```veyra
f"Hello, {name}!"
f'Hello, {name}!'
f"""Hello, {name}!"""
f'''Hello, {name}!'''
```

### `src/runtime.rs`

Responsible for:

- Expression evaluation.
- Variable and constant management.
- Channel operations.
- Conditional statements and loops.
- Function definitions and calls.
- Lambda expressions and callbacks.
- List and JSONData operations.
- Error handling.
- User-defined module loading.
- Runtime execution limits.

Veyra 0.2.7 adds several runtime features.

**String interpolation**

Interpolated expressions are evaluated using Veyra's normal expression evaluator.

```veyra
add age = 20

say f"Next year: {age + 1}"
```

**Functional list methods**

The runtime implements:

- `map(callback)`
- `filter(callback)`
- `reduce(callback, initial)`
- `any(callback)`
- `all(callback)`

These methods use the existing function-call and callback mechanisms.

**JSONData enhancements**

The runtime supports:

- `merge(other)`
- `items()`
- JSONData merging with `+`

Merging uses shallow merge semantics.

Existing keys retain their positions, and values from the right-hand object take priority when keys overlap.

## Built-in Modules

The `src/modules/` directory contains built-in modules implemented in Rust.

| File           | Module      |
| -------------- | ----------- |
| `base.rs`      | `base`      |
| `system.rs`    | `system`    |
| `io.rs`        | `io`        |
| `json.rs`      | `json`      |
| `http.rs`      | `http`      |
| `websocket.rs` | `websocket` |
| `math.rs`      | `math`      |
| `time.rs`      | `time`      |
| `fs.rs`        | `fs`        |
| `random.rs`    | `random`    |

The `mod.rs` file organizes the built-in module implementations.

Built-in modules are compiled into the interpreter.

User-defined `.veymod` modules are interpreted at runtime.

## Testing

Veyra includes automated regression tests.

### `tests/cli-tests.rs`

Contains Rust integration tests that execute Veyra programs through the command-line interpreter.

These tests verify language features and detect regressions.

### `tests/fixtures/`

Contains `.vey` programs used by the regression tests.

Veyra 0.2.7 includes regression coverage for:

- String interpolation.
- Functional list methods.
- JSONData merging and `items()`.
- Existing language features.

Run the complete test suite:

```bash
cargo test
```

You can also execute individual regression tests:

```bash
cargo test string_interpolation
cargo test functional_lists
cargo test jsondata_enhancements
```

Veyra source files can also contain their own test blocks.

Run them using:

```bash
veyra --test tests.vey
```

## Documentation

The `docs/` directory contains Veyra's Sphinx documentation.

The documentation uses MyST Markdown and the Shibuya theme.

Important files include:

- **`docs/conf.py`** — Sphinx configuration.
- **`docs/requirements.txt`** — Python documentation dependencies.
- **`docs/index.md`** — Documentation homepage.
- **`docs/features.md`** — Language feature overview.
- **`docs/language-overview.md`** — Language reference.
- **`docs/learn.md`** — Learning guide navigation.
- **`docs/learning/`** — Step-by-step lessons.

### Building the Documentation

Install the documentation dependencies:

```bash
pip install -r docs/requirements.txt
```

On Linux or macOS:

```bash
cd docs
make html
```

On Windows PowerShell:

```powershell
cd docs
.\make.bat html
```

The generated HTML documentation is placed in the Sphinx build output directory.

The online documentation is available at [veyra.readthedocs.io](https://veyra.readthedocs.io).

## VSCode Extension

The `veyra-vscode/` directory contains the Visual Studio Code extension.

### `veyra-vscode/package.json`

Defines:

- Extension metadata.
- Supported file extensions.
- Language registration.
- Command registration.
- Extension version.

The extension version for this release is `0.2.7`.

### `veyra-vscode/extension.js`

Implements the integrated REPL command:

```text
Veyra: Start REPL
```

The command opens a terminal and starts the Veyra interpreter.

### `veyra-vscode/syntaxes/veyra.tmLanguage.json`

Contains TextMate syntax highlighting rules.

Veyra 0.2.7 adds highlighting support for:

- Interpolated strings.
- Embedded interpolation expressions.
- Functional list methods.
- JSONData `merge()` and `items()` methods.

### `veyra-vscode/snippets/veyra.json`

Contains code snippets for common Veyra constructs.

Version 0.2.7 introduces snippets for:

- `fstr`
- `fmultistr`
- `map`
- `filter`
- `reduce`
- `any`
- `all`
- `merge`
- `items`

### `veyra-vscode/language-configuration.json`

Defines editor behavior such as:

- Automatic bracket closing.
- Automatic quote closing.
- Indentation rules.
- Code folding.

### Packaging the Extension

From the repository root:

```bash
cd veyra-vscode
npm install
npx vsce package
```

The generated package is named:

```text
veyra-0.2.7.vsix
```

## Examples

The `examples/` directory contains example Veyra programs.

These programs demonstrate language syntax and built-in functionality.

To execute an example:

```bash
veyra examples/hello.vey
```

Or from source:

```bash
cargo run -- examples/hello.vey
```

## Project Resources

- [GitHub Repository](https://github.com/EmxrWasHere0/veyra)
- [Documentation](https://veyra.readthedocs.io)
- [VSCode Extension](vscode-extension.md)
- [Features](features.md)
- [Language Overview](language-overview.md)
