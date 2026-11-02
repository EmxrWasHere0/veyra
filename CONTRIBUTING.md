# Contributing to Veyra

Thank you for your interest in contributing to Veyra and its ecosystem.

Contributions can improve the Veyra interpreter, built-in modules, documentation, VSCode extension, examples, tests, and Veyra Package Manager (VPM).

## Before You Start

For substantial changes, open an issue first to discuss the proposed design and scope.

Small fixes, documentation improvements, and focused tests can usually be submitted directly as a pull request.

Please do not report suspected security vulnerabilities in public issues. Follow the private reporting instructions in [SECURITY.md](SECURITY.md).

## Development Requirements

Install the tools relevant to the area you want to work on:

- **Veyra interpreter:** Rust and Cargo, installed through [rustup](https://rustup.rs/).
- **Python VPM:** Python 3.10 or newer.
- **VSCode extension:** Node.js and npm.
- **Documentation:** Sphinx and the documentation dependencies described in `docs/`.

The Python VPM implementation uses the Python standard library.

## Running Tests and Checks

Run the checks relevant to your changes before submitting a pull request.

### Veyra Interpreter

From the repository root:

```bash
cargo fmt --check
cargo test
cargo build --release
```

For changes to a specific language feature, run the relevant Veyra test fixture as well. For example:

```bash
cargo run -- --test tests/fixtures/short_circuit.vey
```

### Python VPM

From the `vpm/` directory:

```bash
python -m unittest discover -s tests -v
```

### Documentation

Build the Sphinx documentation from the `docs/` directory:

```bash
make html
```

On Windows, use:

```bat
make.bat html
```

### VSCode Extension

From the `veyra-vscode/` directory:

```bash
npm install
npx vsce package
```

If a check cannot be run in your environment, mention that in the pull request.

## Implementation Guidelines

- Keep changes focused and avoid unrelated refactoring.
- Preserve Veyra's existing language semantics, especially channel behavior, closed function scopes, recursion, callbacks, and module isolation.
- Built-in modules belong in Rust. User-defined `.veymod` modules are interpreted at runtime.
- Keep the interpreter's core organization in `src/lexer.rs`, `src/runtime.rs`, `src/main.rs`, and `src/modules/`. Avoid splitting the core into many files without first discussing the change.
- Add regression tests for bug fixes and new behavior where practical.
- Update relevant documentation when behavior or user-facing features change.
- Keep code comments, error messages, CLI messages, and module output in English.
- Consider cross-platform behavior when changing filesystem, process, networking, or path-related code.
- Do not include secrets, credentials, generated build output, or unrelated changes in a pull request.

## VPM Contributions

The Python implementation is the current VPM development focus. The Veyra implementation is experimental and its development is paused.

VPM changes should preserve the shared package format:

- Project and package manifests use `veyra.json`.
- Installed packages are placed in `veyra_packages/`.
- Packages declare their import name in the `module` field.
- Package entry files use the `.veymod` extension.

VPM 0.1.0 supports local path packages. Do not document or imply that remote package installation, lockfiles, or automatic dependency resolution are available until they are implemented.

When changing package installation or removal behavior, include tests for relevant failure and rollback cases.

## Submitting a Pull Request

1. Create a branch for your change.
2. Make a focused change and add or update tests where appropriate.
3. Run the relevant checks listed above.
4. Open a pull request with:
   - A concise description of the change.
   - The reason for the change.
   - The tests and checks you ran.
   - Any known limitations or follow-up work.

Please keep pull requests focused. If a proposal needs broader design discussion, open an issue before implementing it.

## Licensing

Veyra is licensed under the [GNU General Public License v3.0](LICENSE). Contributions to this repository are made available under the project's license.
