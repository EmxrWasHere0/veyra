# Language Overview

Veyra is a dynamically typed, interpreted programming language with readable, line-based syntax.

Blocks are terminated using `end`, and indentation is optional.

This page provides a brief overview of the language. For detailed explanations, see the [Learning Guide](learn.md).

## Values

Veyra supports seven value types:

| Type       | Example                     |
| ---------- | --------------------------- |
| `number`   | `42`, `3.14`, `-7`          |
| `text`     | `"hello"`, `'world'`        |
| `bool`     | `true`, `false`             |
| `list`     | `[1, 2, 3]`                 |
| `jsondata` | `{"name": "Veyra"}`         |
| `function` | Named functions and lambdas |
| `null`     | `null`                      |

Values are dynamically typed and can be inspected with `type()`.

```veyra
say type(42)
say type([1, 2, 3])
say type({"name": "Veyra"})
```

## Variables and Constants

Use `add` to declare mutable variables:

```veyra
add x = 10
x = 20

say x
```

Use `set` to declare constants:

```veyra
set PI = 3.14159

say PI
```

Assigning to a `set` constant updates its channel rather than changing the original value.

## Channels

Channels are one of Veyra's distinctive features.

```veyra
set port = 8080
port = 3000

say port
say 0 ? port
```

Output:

```text
8080
3000
```

The `?` operator returns the channel value when available, otherwise the fallback value.

Channels can also be passed through function arguments.

For details, see [Channels](learning/channels.md).

## Input and Output

Print values using `say`:

```veyra
say "Hello, World!"
say 10 + 20
```

Read user input using `get`:

```veyra
get name "Enter your name: "

say f"Hello, {name}!"
```

Convert values using built-in functions:

```veyra
get age "Enter your age: "

add next_age = num(age) + 1
say f"Next year: {next_age}"
```

## Operators

Veyra supports arithmetic, comparison, logical, and assignment operators.

| Category               | Operators                        |
| ---------------------- | -------------------------------- |
| Arithmetic             | `+`, `-`, `*`, `/`, `%`          |
| Comparison             | `==`, `!=`, `<`, `>`, `<=`, `>=` |
| Logical                | `and`, `or`, `not`, `!`          |
| Assignment             | `=`                              |
| Membership             | `in`, `not in`                   |
| Channels               | `?`                              |
| Conditional assignment | `?=`                             |
| Null coalescing        | `??`                             |
| Lambda                 | `=>`                             |

Example:

```veyra
add numbers = [10, 20, 30]

say 20 in numbers
say 50 not in numbers

add value = null
say value ?? "default"
```

For details, see [Operators](learning/operators.md).

## Control Flow

### Conditionals

```veyra
add age = 18

if age >= 18
    say "Adult"
else
    say "Minor"
end
```

### Loops

Counted loops:

```veyra
repeat 3
    say "Hello"
end
```

Conditional loops:

```veyra
add i = 0

while i < 3
    say i
    i = i + 1
end
```

Collection iteration:

```veyra
add colors = ["red", "green", "blue"]

repeat color in colors
    say color
end
```

Labeled loops:

```veyra
@outer repeat 5
    repeat 3
        stop @outer
    end
end
```

Use `stop` to exit a loop and `fwd` to continue to its next iteration.

## Lists

Lists support indexing, slicing, and common collection operations.

```veyra
add numbers = [10, 20, 30, 40]

say numbers[0]
say numbers[-1]
say numbers[1:3]

numbers[1] = 99
numbers.append(50)

say numbers
say numbers.len()
```

Available list methods include `append`, `pop`, `insert`, `remove`, `clear`, `sort`, and `reverse`.

### Functional List Methods

Veyra 0.2.7 introduces functional list operations using callbacks.

```veyra
add numbers = [1, 2, 3, 4, 5]

add doubled = numbers.map(func(x) => x * 2)
add evens = numbers.filter(func(x) => x % 2 == 0)
add total = numbers.reduce(func(acc, x) => acc + x, 0)

say doubled
say evens
say total
```

Output:

```text
[2, 4, 6, 8, 10]
[2, 4]
15
```

Use `any()` and `all()` to test conditions across a list:

```veyra
say numbers.any(func(x) => x > 3)
say numbers.all(func(x) => x > 0)
```

Output:

```text
true
true
```

These operations return new values without modifying the original list.

For details, see [Lists](learning/lists.md).

## JSONData Objects

Veyra provides the `jsondata` type for key-value collections.

```veyra
add user = {
    "name": "Veyra",
    "version": "0.2.7",
    "active": true
}

say user["name"]

user["version"] = "0.2.8"

say user.keys()
say user.has("active")
```

Objects support nested indexing, assignment, and iteration over keys.

### JSONData Merging

Veyra 0.2.7 adds `merge()`, `items()`, and object merging with `+`.

```veyra
add user = {"name": "Alex", "age": 20}
add updates = {"age": 21, "active": true}

add merged = user.merge(updates)

say merged
say user + updates
say user.items()
```

Both `merge()` and `+` create new JSONData objects. If a key exists in both objects, the right-hand value takes precedence.

Merging is shallow: nested objects are replaced rather than recursively merged.

The `items()` method returns a list of key-value pairs:

```veyra
add user = {"name": "Alex", "age": 20}

say user.items()
```

Output:

```text
[["name", "Alex"], ["age", 20]]
```

## Strings

Strings support Unicode-aware indexing, slicing, and common text operations.

```veyra
add message = "Hello, Veyra!"

say message.upper()
say message.lower()
say message.contains("Veyra")
say message.replace("Hello", "Hi")
```

Multiline strings use triple quotes:

```veyra
add message = """
Hello!
This is a multiline string.
"""

say message
```

### String Interpolation

Veyra 0.2.7 supports interpolated strings using the `f` prefix.

```veyra
add name = "Alex"
add age = 20

say f"Hello, {name}!"
say f"Next year: {age + 1}"
say f"Adult: {age >= 18}"
```

Output:

```text
Hello, Alex!
Next year: 21
Adult: true
```

Interpolation supports Veyra expressions, including arithmetic, comparisons, function calls, and indexing.

Use `{{` and `}}` to insert literal braces:

```veyra
say f"{{name}} = {name}"
```

Output:

```text
{name} = Alex
```

Single-quoted and multiline interpolated strings are also supported:

```veyra
add report = f"""
Name: {name}
Age: {age}
"""

say report
```

## Functions

Functions are declared using `add`:

```veyra
add greet(name, greeting = "Hello")
    return f"{greeting}, {name}!"
end

say greet("Veyra")
```

Veyra supports default parameters, variadic parameters, recursion, and multiple return values.

Lambdas are also supported:

```veyra
add square = func(x) => x * x

say square(5)
```

Functions use closed scopes and do not implicitly capture surrounding local variables.

For details, see [Functions](learning/functions.md).

## Error Handling

Use `try`, `catch`, and `throw` to handle runtime errors.

```veyra
try
    throw "Something went wrong"
catch error
    say error
end
```

Uncaught errors include source file and line information, while function errors can include a traceback.

## Modules

Veyra includes several built-in Rust modules:

| Module      | Purpose                            |
| ----------- | ---------------------------------- |
| `system`    | Operating system and environment   |
| `io`        | Input/output and process execution |
| `base`      | System information                 |
| `json`      | JSON parsing and serialization     |
| `http`      | HTTP requests                      |
| `websocket` | WebSocket connections              |
| `math`      | Mathematical operations            |
| `time`      | Date and time utilities            |
| `fs`        | Filesystem operations              |
| `random`    | Random number generation           |

Import a module using `mod`:

```veyra
mod math

say math.sqrt(144)
```

Veyra also supports user-defined `.veymod` modules.

Import individual members:

```veyra
mod math [sqrt]

say sqrt(144)
```

For details, see [Modules](learning/modules.md).

## Interactive REPL

Start Veyra without specifying a file:

```bash
veyra
```

The REPL supports persistent variables, functions, imports, and multiline blocks.

```text
>>> add x = 10
>>> say f"Double: {x * 2}"
Double: 20
```

Type `exit` or `quit` to leave the REPL.

## Learn More

Continue with the detailed guides:

- [Getting Started](learning/quickstart.md)
- [Variables](learning/variables.md)
- [Channels](learning/channels.md)
- [Functions](learning/functions.md)
- [Control Flow](learning/control-flow.md)
- [Lists](learning/lists.md)
- [Modules](learning/modules.md)
- [Error Handling](learning/error-handling.md)
