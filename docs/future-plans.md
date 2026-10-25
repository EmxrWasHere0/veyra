# Future Plans

Veyra is an experimental programming language that continues to evolve.

With version 0.2.7, the language includes string interpolation, functional list methods, enhanced JSONData operations, an interactive REPL, user-defined modules, expanded built-in libraries, automated tests, and improved developer tools.

The following ideas are being considered for future development.

## Planned for Veyra 0.2.8

### Compound Assignment Operators

Veyra 0.2.8 is planned to introduce compound assignment operators.

These operators will provide shorter syntax for updating variables.

```veyra
add x = 10

x += 5
x -= 3
x *= 2
x /= 4
x %= 3

say x
```

Planned operators:

- `+=` — Addition assignment
- `-=` — Subtraction assignment
- `*=` — Multiplication assignment
- `/=` — Division assignment
- `%=` — Modulo assignment

These operators are **not available in Veyra 0.2.7**.

Until they are implemented, use regular assignments:

```veyra
add counter = 0

counter = counter + 1
```

Compound assignments are intended to follow Veyra's existing assignment rules, including its channel semantics.

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

### Additional Built-in Utilities

Continue expanding existing modules where useful.

Potential improvements include additional text utilities, collection operations, and standard library functions.

## Developer Experience

### Improved Diagnostics

Improve error reporting with:

- More precise source locations, including character columns
- Clearer syntax error messages
- More helpful suggestions for common mistakes
- Better traceback presentation

Veyra already provides source-line information and function tracebacks.

### Editor Integration

Explore additional features for the Visual Studio Code extension:

- Intelligent autocomplete
- Real-time diagnostics
- Symbol navigation
- Improved syntax highlighting
- Integrated debugging tools

The current extension provides syntax highlighting, snippets, indentation, folding, and an integrated REPL.

### Documentation Improvements

Continue maintaining and expanding:

- Language references
- Beginner tutorials
- Built-in module documentation
- Practical examples
- Migration notes between versions

### Testing Improvements

Expand automated regression coverage, particularly for:

- Nested expressions
- String interpolation edge cases
- Functional callbacks
- Module interactions
- Channel behavior
- Error handling

Improve test reporting and cross-platform compatibility.

## Performance

### Interpreter Optimizations

Investigate improvements to execution speed and memory efficiency.

Potential areas include:

- Expression evaluation
- Function calls
- Collection operations
- Module loading
- Memory allocation

Any optimizations should preserve Veyra's existing language semantics.

## Long-Term Possibilities

### Standalone Executables

Explore compiling Veyra programs into native executables.

### Hybrid Execution

Investigate supporting both interpreted and compiled execution.

These are potential directions rather than confirmed features or release commitments.

Veyra's development priorities may change based on experimentation, implementation complexity, and community feedback.

## Contributing

Suggestions, bug reports, and contributions are welcome.

Visit the [Veyra GitHub repository](https://github.com/EmxrWasHere0/veyra) to participate in development.
