# Quick Start

Write your first Veyra program and run it using the interpreter.

## Your First Program

Create a file named `hello.vey`:

```veyra
say "Hello, World!"

add greet(name, greeting = "Hi")
    return f"{greeting}, {name}!"
end

say greet("Alex")
say greet("Alice", "Hello")

add counter = 1

while counter <= 3
    say f"Count: {counter}"
    counter = counter + 1
end
```

Run it:

```bash
veyra hello.vey
```

Or, if you are running Veyra from source:

```bash
cargo run --release -- hello.vey
```

Expected output:

```text
Hello, World!
Hi, Alex!
Hello, Alice!
Count: 1
Count: 2
Count: 3
```

## String Interpolation

Veyra 0.2.7 supports interpolated strings using the `f` prefix.

```veyra
add name = "Alex"
add age = 20

say f"Hello, {name}!"
say f"Next year: {age + 1}"
```

Output:

```text
Hello, Alex!
Next year: 21
```

Expressions inside `{}` are evaluated automatically.

You can also use multiline interpolated strings:

```veyra
add report = f"""
Name: {name}
Age: {age}
"""

say report
```

## Working with Lists

Veyra supports lists and functional collection methods.

```veyra
add numbers = [1, 2, 3, 4, 5]

add doubled = numbers.map(func(x) => x * 2)
add evens = numbers.filter(func(x) => x % 2 == 0)

say doubled
say evens
```

Output:

```text
[2, 4, 6, 8, 10]
[2, 4]
```

Other functional methods include `reduce()`, `any()`, and `all()`.

See [Lists](learning/lists.md) for more examples.

## Working with JSONData

JSONData objects store key-value pairs.

```veyra
add user = {"name": "Alex", "age": 20}
add updates = {"age": 21, "active": true}

add merged = user.merge(updates)

say merged
say user.items()
```

Output:

```text
{"name": "Alex", "age": 21, "active": true}
[["name", "Alex"], ["age", 20]]
```

Veyra 0.2.7 also supports merging JSONData objects with the `+` operator.

## Interactive REPL

You can also run Veyra interactively without creating a file.

Start the REPL:

```bash
veyra
```

Enter expressions and statements:

```text
>>> add x = 10
>>> say f"Double: {x * 2}"
Double: 20
>>> add square(n)
...     return n * n
... end
>>> say square(5)
25
```

The REPL preserves variables, functions, and imports between commands.

Type `exit` or `quit` to leave.

## What's Next?

Continue with:

- [Learning Guide](learn.md)
- [Language Overview](language-overview.md)
- [Variables and Types](learning/variables.md)
- [Functions](learning/functions.md)
- [Running Veyra Programs](learning/run.md)
