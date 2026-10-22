# Project Layout

Veyra is implemented in Rust. The interpreter's core is organized around `main.rs`, `lexer.rs`, and `runtime.rs`, while built-in modules are maintained separately.

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
│   ├── index.md
│   ├── features.md
│   ├── language-overview.md
│   ├── learn.md
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

- **`src/main.rs`** — Command-line interface, script execution, and interactive REPL.
- **`src/lexer.rs`** — Tokenization and source-line parsing.
- **`src/runtime.rs`** — Expression evaluation, control flow, functions, channels, collections, and module loading.
- **`src/modules/`** — Built-in modules implemented in Rust.

## Testing

- **`tests/cli-tests.rs`** — Automated integration tests that execute Veyra programs.
- **`tests/fixtures/`** — Veyra source files used by regression tests.

Run the tests with:

```bash
cargo test
```

## Documentation

- **`docs/`** — Sphinx documentation and language guides.
- **`docs/learning/`** — Step-by-step lessons covering Veyra syntax and features.
- **`examples/`** — Example Veyra programs.

## VSCode Extension

- **`veyra-vscode/package.json`** — Extension manifest and command registration.
- **`veyra-vscode/extension.js`** — Integrated REPL command.
- **`veyra-vscode/syntaxes/`** — TextMate syntax highlighting grammar.
- **`veyra-vscode/snippets/`** — Code snippets.
- **`veyra-vscode/language-configuration.json`** — Indentation, brackets, and editor behavior.
