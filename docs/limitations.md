# Limitations

Veyra is an experimental programming language. While it supports functions, collections, modules, and error handling, some features are intentionally limited or not yet implemented.

## Unsupported Features

- **String interpolation** — Expressions such as `f"Hello, {name}"` are not supported.
- **Pattern matching** — There is no `match` or `switch` construct.
- **Advanced functional collection methods** — Methods such as `map`, `filter`, and `reduce` are not currently built in.
- **Static type checking** — Veyra is dynamically typed and does not provide compile-time type analysis.
- **Native compilation** — Veyra programs are executed by the interpreter rather than compiled into standalone executables.

## Design Choices

### Closed-Scope Functions

Veyra functions do not automatically capture variables from surrounding local scopes.

```veyra
add outer()
    add value = 10

    add inner()
        return value
    end

    return inner()
end
```

The example above cannot access `value` from `inner()` unless the value is explicitly passed as an argument.

This behavior is intentional and keeps function dependencies explicit.

### Value-Based Collections

Lists and `jsondata` objects use value-based semantics rather than shared mutable references.

Assigning a collection to another variable does not create a shared mutable object.

### Runtime Execution

Veyra is interpreted and dynamically typed. Certain errors are detected only when the relevant code executes.

The interpreter provides source locations and function tracebacks to help diagnose runtime errors.

## Editor Support

The Visual Studio Code extension currently supports:

- Syntax highlighting
- Automatic indentation and bracket closing
- Code folding
- Code snippets
- Integrated REPL

Advanced editor features such as semantic diagnostics, intelligent autocomplete, and integrated debugging are not currently available.

## Safety Limits

To prevent uncontrolled execution, Veyra applies runtime limits:

- **Maximum loop iterations:** 1,000,000 per loop.
- **Maximum function call depth:** 1,000.

## Contributing

Veyra continues to evolve, and suggestions or contributions are welcome.

Visit the [GitHub repository](https://github.com/EmxrWasHere0/veyra) to report issues or propose improvements.
