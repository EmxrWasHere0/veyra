# Limitations

Veyra is an experimental programming language. While it supports functions, collections, modules, error handling, string interpolation, and functional list methods, some features are intentionally limited or not yet implemented.

## Unsupported Features

- **Compound assignment operators** — Operators such as `+=`, `-=`, `*=`, `/=`, and `%=` are not currently supported. Use regular assignments such as `x = x + 1` instead.
- **Pattern matching** — There is no `match` or `switch` construct.
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

```veyra
add original = [1, 2, 3]
add copied = original

copied.append(4)

say original
say copied
```

Output:

```text
[1, 2, 3]
[1, 2, 3, 4]
```

Functional list methods such as `map()`, `filter()`, `reduce()`, `any()`, and `all()` do not modify the original list.

### JSONData Merging

Veyra 0.2.7 supports merging JSONData objects with `merge()` and the `+` operator.

Both operations perform a **shallow merge**.

```veyra
add left = {
    "user": {"name": "Alex", "age": 20}
}

add right = {
    "user": {"age": 21}
}

say left.merge(right)
```

Output:

```text
{"user": {"age": 21}}
```

Nested objects are replaced rather than recursively merged.

When keys overlap, values from the right-hand object take priority.

The original objects remain unchanged.

### Runtime Execution

Veyra is interpreted and dynamically typed. Certain errors are detected only when the relevant code executes.

The interpreter provides source locations and function tracebacks to help diagnose runtime errors.

Error messages currently identify the relevant source line rather than the exact character column.

## Editor Support

The Visual Studio Code extension currently supports:

- Syntax highlighting, including interpolated strings
- Automatic indentation and bracket closing
- Code folding
- Code snippets, including functional list methods
- Integrated REPL

Advanced editor features such as semantic diagnostics, intelligent autocomplete, and integrated debugging are not currently available.

Syntax highlighting is based on TextMate grammar rules rather than full semantic analysis. Complex nested expressions, particularly inside interpolated strings, may not always be highlighted perfectly.

## Safety Limits

To prevent uncontrolled execution, Veyra applies runtime limits:

- **Maximum loop iterations:** 1,000,000 per loop.
- **Maximum function call depth:** 1,000.

## Contributing

Veyra continues to evolve, and suggestions or contributions are welcome.

Visit the [GitHub repository](https://github.com/EmxrWasHere0/veyra) to report issues or propose improvements.
