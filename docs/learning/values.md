# Values

Veyra is a dynamically typed programming language. Values have types, but variables do not require explicit type declarations.

Veyra supports **7 value types**:

| Type       | Example                  | Description         |
| ---------- | ------------------------ | ------------------- |
| `number`   | `42`, `3.14`, `-7`       | Numeric values      |
| `text`     | `"Hello"`, `'World'`     | Text strings        |
| `bool`     | `true`, `false`          | Boolean values      |
| `list`     | `[1, 2, 3]`              | Ordered collections |
| `jsondata` | `{"name": "Veyra"}`      | Key-value objects   |
| `function` | Named functions, lambdas | Callable values     |
| `null`     | `null`                   | Absence of a value  |

## Number

The `number` type represents numeric values, including integers and decimal numbers.

```veyra
add age = 27
add price = 4.99
add temperature = -8
add offset = -1.49

say age + 3
say price * 2
```

Veyra uses a single numeric type rather than separate integer and floating-point types.

## Text

The `text` type represents strings.

Both single and double quotes are supported:

```veyra
add greeting = "Hello, World!"
add message = 'Hi, Veyra!'

say greeting
say message
```

### Unicode Support

Veyra supports Unicode text indexing and slicing.

```veyra
add word = "Veyra"

say word[0]
say word[1:4]
```

Strings also support methods such as `upper()`, `lower()`, `trim()`, `split()`, and `replace()`.

### Multiline Strings

Triple quotes allow strings to span multiple lines.

```veyra
add message = """
Hello!
Welcome to Veyra.
This text spans multiple lines.
"""

say message
```

Triple single quotes (`'''`) are also supported.

## Bool

The `bool` type represents two logical values:

```veyra
add active = true
add finished = false

say active
say finished
```

Boolean values are commonly used in conditions:

```veyra
if active
    say "Active"
else
    say "Inactive"
end
```

## List

The `list` type stores an ordered collection of values.

Lists can contain different types:

```veyra
add numbers = [1, 2, 3, 4]
add mixed = [28, "Alex", true, null]
add matrix = [[1, 2], [3, 4]]
```

Elements can be accessed by index:

```veyra
say numbers[0]
say numbers[-1]
say matrix[1][0]
```

Lists support indexing, slicing, and methods such as `append()`, `pop()`, `sort()`, and `reverse()`.

See [Lists](lists.md) for details.

## JSONData

The `jsondata` type represents a collection of key-value pairs.

Keys are strings, while values can be any Veyra value.

```veyra
add user = {
    "name": "Alex",
    "age": 28,
    "active": true,
    "skills": ["Rust", "Veyra"]
}

say user["name"]
say user["skills"][0]
```

Object values can be updated:

```veyra
user["age"] = 29

say user["age"]
```

JSONData objects support methods such as `keys()`, `values()`, `get()`, `has()`, and `remove()`.

The built-in `json` module can also convert JSON text into Veyra values.

## Function

Functions are first-class values in Veyra.

A function can be declared using `add`:

```veyra
add sum(x, y)
    return x + y
end

say sum(7, 14)
```

Functions can also be created using lambda expressions:

```veyra
add square = func(x) => x * x

say square(5)
```

Veyra supports recursion, default parameters, variadic parameters, and functions passed as arguments.

See [Functions](functions.md) for details.

## Null

The `null` type represents the absence of a value.

```veyra
add result = null

say result
```

The `??` operator provides a fallback when a value is `null`:

```veyra
add name = null

say name ?? "Guest"
```

## Type Inspection

Use the built-in `type()` function to inspect the type of a value.

```veyra
say type(42)
say type("Hello")
say type(true)
say type([1, 2, 3])
say type({"name": "Veyra"})
say type(null)
```

Expected output:

```text
number
text
bool
list
jsondata
null
```

## Truthy and Falsy Values

Veyra evaluates values as either truthy or falsy when used in conditions.

The following values are falsy:

- `false`
- `null`
- `0`
- `""` (empty text)
- `[]` (empty list)

Other values are generally truthy.

For example:

```veyra
add items = []

if items
    say "List is not empty"
else
    say "List is empty"
end
```

## Dynamic Typing

Variables can hold different types during execution.

```veyra
add value = 10
say type(value)

value = "Hello"
say type(value)

value = [1, 2, 3]
say type(value)
```

No explicit type annotations are required.
