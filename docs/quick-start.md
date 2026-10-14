## Quick Start

Create a file called `hello.vey`:

```veyra
# hello.vey — your first Veyra program

say "Hello, World!"

add greet(name, greeting = "Hi")
    greeting + ", " + name + "!"
end

say greet("Alex")
say greet("Alice", "Hello")

# A tiny counter
i = 1
repeat while i <= 3
    say i
    i += 1
end
```

Run it:

```bash
# With cargo:
cargo run --release -- hello.vey

# After cargo install:
veyra hello.vey
```

Expected output:

```
Hello, World!
Hi, Alex!
Hello, Alice!
1
2
3
```
