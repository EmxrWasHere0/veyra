# Future Plans

Veyra is an experimental programming language that continues to evolve.

Version 0.2.8 introduced compound assignment operators, short-circuit evaluation, improved REPL stability, and column-based error diagnostics.

Development has since expanded into the Veyra ecosystem with **VPM 0.1.0**, a local package manager for reusable `.veymod` modules.

VPM supports project initialization, local package installation, updates, removal, and dependency management. The interpreter can discover installed packages through their `veyra.json` manifests.

The following ideas are being considered for future development.

## Package Manager (VPM)

The Veyra Package Manager is being developed to make reusable modules easier to distribute and manage.

VPM 0.1.0 provides a working foundation for local package management.

The Python implementation is the primary development focus. An experimental Veyra implementation is also available, but its development is currently paused.

See the [VPM documentation](../vpm/README.md) for current usage instructions.

### VPM 0.2.0 — Remote Packages

Planned improvements include:

- Installing packages from GitHub repositories.
- Selecting package versions through Git tags or commits.
- Generating `veyra.lock` files.
- Verifying downloaded package contents using SHA-256.
- Caching downloaded packages.
- Secure extraction of downloaded archives.

Remote package installation must validate paths, archive entries, package manifests, and downloaded content before installation.

### VPM 0.3.0 — Dependency Resolution

Planned improvements include:

- Installing transitive dependencies.
- Supporting semantic version constraints.
- Detecting dependency version conflicts.
- Resolving compatible dependency versions.
- Improving dependency installation consistency.

These features are planned directions and may change during development.

## Language Improvements

### Pattern Matching

Explore structured pattern matching for values and conditions.

Potential syntax:

```veyra
add status = 200

match status
    case 200
        say "OK"
    case 404
        say "Not Found"
    case _
        say "Unknown status"
end
```

This syntax is conceptual and is not currently supported.

Pattern matching is not part of the Veyra 0.2.8 feature set. No release version has been assigned to it.

### Additional Built-in Utilities

Continue expanding existing modules where useful.

Potential improvements include:

- Additional text processing utilities.
- More collection operations.
- Expanded standard library functions.
- Improvements to existing module APIs.

New utilities should remain consistent with Veyra's existing value types and language semantics.

## Developer Experience

### Improved Diagnostics

Veyra 0.2.8 introduces column-based runtime error locations and more precise token highlighting.

Future diagnostic improvements may include:

- Clearer syntax error messages.
- Suggestions for common programming mistakes.
- More consistent error locations across built-in modules.
- Better error reporting for complex nested expressions.
- Improved traceback presentation.

Veyra already provides source filenames, line and column information, source context, and function tracebacks.

### Editor Integration

Explore additional features for the Visual Studio Code extension:

- Intelligent autocomplete.
- Real-time diagnostics.
- Symbol navigation.
- Improved syntax highlighting.
- Integrated debugging tools.

The current extension provides syntax highlighting, snippets, indentation, folding, and REPL integration.

### Documentation Improvements

Continue maintaining and expanding:

- Language references.
- Beginner tutorials.
- Built-in module documentation.
- Practical examples.
- Migration notes between versions.

Documentation should reflect the behavior of the current interpreter rather than planned or experimental syntax.

### Testing Improvements

Veyra includes automated interpreter regression tests using Cargo.

Version 0.2.8 expanded coverage for compound assignment, short-circuit evaluation, interpolation edge cases, and error diagnostics.

VPM also includes a Python `unittest` suite with 17 regression tests covering package installation, updates, removal, path validation, symbolic link handling, and rollback behavior.

Future testing improvements may focus on:

- Complex nested expressions.
- String interpolation edge cases.
- Functional callbacks.
- Module interactions.
- Channel behavior.
- Error handling and tracebacks.
- REPL input completion.
- Cross-platform compatibility.
- Package manifest validation.
- Package installation rollback.
- Package import resolution.
- Cross-platform package management.

Additional automated REPL tests and parser consistency checks may also be explored.

## Performance

### Interpreter Optimizations

Investigate improvements to execution speed and memory efficiency.

Potential areas include:

- Expression evaluation.
- Function calls.
- Collection operations.
- Module loading.
- Memory allocation.

Any optimizations should preserve Veyra's existing language semantics, including channels, closed scopes, and callback behavior.

## Long-Term Possibilities

### Standalone Executables

Explore compiling Veyra programs into native executables.

This could allow applications written in Veyra to run without requiring a separately installed interpreter.

### Hybrid Execution

Investigate supporting both interpreted and compiled execution.

Possible approaches could include ahead-of-time compilation or other execution strategies.

These are potential directions rather than confirmed features or release commitments.

Veyra's development priorities may change based on experimentation, implementation complexity, and community feedback.

## Contributing

Suggestions, bug reports, and contributions are welcome.

Visit the [Veyra GitHub repository](https://github.com/EmxrWasHere0/veyra) to participate in development.
