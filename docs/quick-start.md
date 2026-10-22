# Quick Start

Write your first Veyra program and run it using the interpreter.

## Your First Program

Create a file named `hello.vey`:

```veyra
say "Hello, World!"

add greet(name, greeting = "Hi")
    return greeting + ", " + name + "!"
end

say greet("Alex")
say greet("Alice", "Hello")

add counter = 1

while counter <= 3
    say counter
    counter += 1
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
1
2
3
```

## Interactive REPL

You can also run Veyra interactively without creating a file.

Start the REPL:

```bash
veyra
```

Enter expressions and statements:

```text
>>> add x = 10
>>> say x * 2
20
>>> add square(n)
...     return n * n
... end
>>> say square(5)
25
```

The REPL preserves variables and functions between commands.

Type `exit` or `quit` to leave.

## What's Next?

Continue with:

- [Learning Guide](learn.md)
- [Language Overview](language-overview.md)
- [Variables and Types](learning/variables.md)
- [Functions](learning/functions.md)
- [Running Veyra Programs](learning/run.md)
