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

### String Interpolation

Veyra 0.2.7 introduces string interpolation using the `f` prefix.

Interpolated strings can contain variables and expressions inside `{}`.

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

Interpolation supports arithmetic, comparisons, indexing, and function calls.

Use `{{` and `}}` to include literal braces:

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

Supported forms include:

- `f"..."`
- `f'...'`
- `f"""..."""`
- `f'''...'''`

Normal strings without the `f` prefix do not evaluate expressions inside braces.

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

### Functional List Methods

Veyra 0.2.7 adds methods for processing lists with callback functions.

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

Other functional methods include:

- `any(callback)` — Returns `true` if at least one element satisfies the callback.
- `all(callback)` — Returns `true` if every element satisfies the callback.

These operations do not modify the original list.

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

JSONData objects support the following methods:

- `keys()` — Returns a list of keys.
- `values()` — Returns a list of values.
- `items()` — Returns key-value pairs as nested lists.
- `get(key)` — Returns the value associated with a key, or `null` if missing.
- `has(key)` — Checks whether a key exists.
- `remove(key)` — Removes a key and returns its value.
- `len()` — Returns the number of entries.
- `merge(other)` — Returns a new JSONData object containing entries from both objects.

### JSONData Merging

Veyra 0.2.7 supports merging JSONData objects using `merge()` or `+`.

```veyra
add user = {"name": "Alex", "age": 20}
add updates = {"age": 21, "active": true}

add merged = user.merge(updates)

say merged
say user + updates
```

Output:

```text
{"name": "Alex", "age": 21, "active": true}
{"name": "Alex", "age": 21, "active": true}
```

Both operations return a new object without modifying the originals.

If the same key appears in both objects, the right-hand value takes precedence.

Merging is shallow: nested objects are replaced rather than recursively merged.

### JSONData Items

The `items()` method returns a list containing key-value pairs.

```veyra
add user = {"name": "Alex", "age": 20}

say user.items()
```

Output:

```text
[["name", "Alex"], ["age", 20]]
```

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

Functional list methods can also accept functions as callbacks:

```veyra
add numbers = [1, 2, 3]

add square(x)
    return x * x
end

add squared = numbers.map(square)

say squared
```

Output:

```text
[1, 4, 9]
```

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
- `{}` (empty JSONData object)

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
