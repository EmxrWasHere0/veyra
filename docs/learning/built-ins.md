# Built-in Functions and Modules

Veyra provides built-in functions and modules for common programming tasks.

Built-in functions are available directly, while modules must be imported using `mod`.

Veyra 0.2.7 also includes functional list methods, enhanced JSONData operations, and string interpolation.

## What You Will Learn

By the end of this lesson, you will know how to:

- Call built-in functions.
- Inspect and convert values.
- Work with collection lengths.
- Use built-in collection and text methods.
- Process lists using functional methods.
- Merge JSONData objects.
- Format values using string interpolation.
- Import built-in modules.
- Understand the purpose of each standard module.
- Handle errors from built-in operations.

## 1. Global Built-in Functions

Global built-in functions can be called without importing a module.

### Function Reference

| Function      | Description                              |
| ------------- | ---------------------------------------- |
| `len(value)`  | Return the length of a supported value   |
| `num(value)`  | Convert a compatible value into a number |
| `text(value)` | Convert a value into text                |
| `type(value)` | Return the value's type name             |

### Length

Use `len()` to inspect the size of a collection or text value.

```veyra
add numbers = [10, 20, 30]
add message = "Hello"

say len(numbers)
say len(message)
```

Output:

```text
3
5
```

The `jsondata` type also supports length operations.

```veyra
add user = {
    "name": "Alex",
    "age": 20
}

say user.len()
```

Output:

```text
2
```

### Number Conversion

Use `num()` to convert compatible values into numbers.

```veyra
say num("42")
say num(true)
say num(false)
```

Output:

```text
42
1
0
```

Invalid numeric text causes a runtime error.

### Text Conversion

Use `text()` to convert a value into text.

```veyra
add number = 123

say text(number)
say type(text(number))
```

Output:

```text
123
text
```

### Type Inspection

Use `type()` to inspect a value.

```veyra
say type(42)
say type("Hello")
say type(true)
say type([1, 2, 3])
say type({"name": "Veyra"})
say type(func(x) => x + 1)
say type(null)
```

Output:

```text
number
text
bool
list
jsondata
function
null
```

Veyra supports seven user-visible value types.

## 2. List Methods

Lists provide methods for accessing, modifying, and processing their elements.

| Method                      | Description                          |
| --------------------------- | ------------------------------------ |
| `append(value)`             | Append an element                    |
| `pop(index)`                | Remove an element at an index        |
| `len()`                     | Return the list length               |
| `index(value)`              | Find an element's index              |
| `insert(index, value)`      | Insert an element                    |
| `remove(value)`             | Remove the first matching element    |
| `clear()`                   | Remove all elements                  |
| `sort()`                    | Return a sorted list                 |
| `reverse()`                 | Return a reversed list               |
| `map(callback)`             | Transform each element               |
| `filter(callback)`          | Select matching elements             |
| `reduce(callback, initial)` | Combine elements into a single value |
| `any(callback)`             | Check whether any element matches    |
| `all(callback)`             | Check whether every element matches  |

Example:

```veyra
add numbers = [10, 20]

numbers.append(30)
numbers.insert(1, 15)

say numbers
say numbers.len()
```

Output:

```text
[10, 15, 20, 30]
4
```

### Methods That Return New Lists

`sort()` and `reverse()` return new lists without modifying the original.

```veyra
add numbers = [30, 10, 20]

say numbers.sort()
say numbers.reverse()
say numbers
```

Output:

```text
[10, 20, 30]
[20, 10, 30]
[30, 10, 20]
```

### Functional List Methods

Veyra 0.2.7 introduces five functional list methods.

These methods accept callback functions.

#### map()

`map(callback)` applies a callback to each element and returns a new list.

```veyra
add numbers = [1, 2, 3, 4, 5]

say numbers.map(func(x) => x * 2)
```

Output:

```text
[2, 4, 6, 8, 10]
```

#### filter()

`filter(callback)` returns a new list containing elements whose callback results are truthy.

```veyra
add numbers = [1, 2, 3, 4, 5]

say numbers.filter(func(x) => x % 2 == 0)
```

Output:

```text
[2, 4]
```

#### reduce()

`reduce(callback, initial)` combines list elements into a single value.

The callback receives the accumulator and the current element.

```veyra
add numbers = [1, 2, 3, 4, 5]

say numbers.reduce(func(acc, x) => acc + x, 0)
```

Output:

```text
15
```

The initial value is required.

For an empty list, `reduce()` returns the initial value.

#### any() and all()

`any(callback)` returns `true` when at least one element satisfies the callback.

`all(callback)` returns `true` when every element satisfies the callback.

```veyra
add numbers = [2, 4, 6, 8]

say numbers.any(func(x) => x > 5)
say numbers.all(func(x) => x % 2 == 0)
```

Output:

```text
true
true
```

For empty lists:

- `any()` returns `false`.
- `all()` returns `true`.

Functional list methods do not modify the original list.

Callbacks can be lambda expressions, named Veyra functions, or compatible native functions.

For details, see [Lists](lists.md).

## 3. Text Methods

Text values support common string operations.

| Method               | Description                           |
| -------------------- | ------------------------------------- |
| `len()`              | Return text length                    |
| `upper()`            | Convert to uppercase                  |
| `lower()`            | Convert to lowercase                  |
| `trim()`             | Remove surrounding whitespace         |
| `trim_start()`       | Remove leading whitespace             |
| `trim_end()`         | Remove trailing whitespace            |
| `contains(value)`    | Check for a substring                 |
| `starts_with(value)` | Check the beginning of text           |
| `ends_with(value)`   | Check the end of text                 |
| `replace(old, new)`  | Replace matching text                 |
| `split(separator)`   | Split text into a list                |
| `join(list)`         | Join a list using this text separator |

Example:

```veyra
add message = "  Hello, Veyra!  "

say message.trim()
say message.upper()
say message.contains("Veyra")
```

Output:

```text
Hello, Veyra!
  HELLO, VEYRA!
true
```

### Splitting Text

```veyra
add colors = "red,green,blue"

say colors.split(",")
```

Output:

```text
["red", "green", "blue"]
```

### Joining Text

Call `join()` on the separator string.

```veyra
add colors = ["red", "green", "blue"]

say ", ".join(colors)
```

Output:

```text
red, green, blue
```

### String Interpolation

Veyra 0.2.7 supports interpolated strings.

Use the `f` prefix and `{expression}` syntax.

```veyra
add name = "Alex"
add score = 95

say f"Player: {name}, Score: {score}"
say f"Next score: {score + 5}"
```

Output:

```text
Player: Alex, Score: 95
Next score: 100
```

Interpolation supports single-quoted, double-quoted, and triple-quoted strings.

Use `{{` and `}}` to display literal braces.

Text indexing and slicing are Unicode-aware.

## 4. JSONData Methods

The `jsondata` type stores key-value pairs.

| Method         | Description                               |
| -------------- | ----------------------------------------- |
| `keys()`       | Return object keys                        |
| `values()`     | Return object values                      |
| `get(key)`     | Retrieve a value                          |
| `has(key)`     | Check whether a key exists                |
| `remove(key)`  | Remove a key                              |
| `len()`        | Return the number of entries              |
| `merge(other)` | Return a shallow merge of two objects     |
| `items()`      | Return key-value pairs as a list of lists |

Example:

```veyra
add user = {
    "name": "Alex",
    "age": 20
}

say user.keys()
say user.values()
say user.get("name")
say user.has("email")
```

JSONData values also support indexed access:

```veyra
say user["name"]

user["age"] = 21
```

### Merging JSONData Objects

Veyra 0.2.7 introduces `merge()` and the `+` operator for JSONData objects.

```veyra
add user = {
    "name": "Alex",
    "age": 20
}

add updates = {
    "age": 21,
    "active": true
}

say user.merge(updates)
say user + updates
```

Output:

```text
{"name": "Alex", "age": 21, "active": true}
{"name": "Alex", "age": 21, "active": true}
```

Both operations perform a shallow merge.

When keys overlap, values from the right-hand object take priority.

Existing key positions are preserved, and new keys are appended.

The original objects remain unchanged.

### Getting Key-Value Pairs

Use `items()` to obtain a list of key-value pairs.

```veyra
add user = {
    "name": "Alex",
    "age": 20
}

say user.items()
```

Output:

```text
[["name", "Alex"], ["age", 20]]
```

You can iterate over object keys using `repeat key in object`.

## 5. Built-in Modules

Veyra includes ten built-in modules implemented in Rust.

Import a module using `mod`:

```veyra
mod math

say math.sqrt(144)
```

### Module Reference

| Module      | Purpose                                      |
| ----------- | -------------------------------------------- |
| `system`    | Operating system and environment information |
| `io`        | File, directory, and process operations      |
| `base`      | General system information                   |
| `json`      | JSON parsing and serialization               |
| `http`      | HTTP requests                                |
| `websocket` | WebSocket connections                        |
| `math`      | Mathematical operations                      |
| `time`      | Date and time utilities                      |
| `fs`        | Filesystem operations                        |
| `random`    | Pseudorandom number generation               |

## 6. System Module

The `system` module provides runtime and operating system information.

```veyra
mod system

say system.os
say system.arch
say system.version
```

It also provides functions for environment variables and command-line arguments.

```veyra
mod system

say system.args()
say system.get_env("HOME")
```

Module constants are accessed without parentheses, while functions are called with parentheses.

## 7. IO Module

The `io` module provides file and process operations.

```veyra
mod io

io.write("example.txt", "Hello from Veyra!")

say io.read("example.txt")
```

Common operations include:

- `read`, `write`, `append`
- `exists`, `remove`, `rename`
- `mkdir`, `rmdir`, `listdir`
- `cwd`, `chdir`
- `run`, `shell`

Example:

```veyra
mod io

add result = io.run("echo", ["Hello"])

say result
```

The result contains the exit code, standard output, and standard error.

For more information, see [Input and Output](io.md).

## 8. Base Module

The `base` module provides system information.

```veyra
mod base

say base.hostname()
say base.username()
say base.home_dir()
say base.pid()
```

It also includes functions for obtaining the executable path and processor-core count.

## 9. JSON Module

The `json` module converts between JSON text and Veyra values.

```veyra
mod json

add user = json.parse("{\"name\":\"Alex\",\"age\":20}")

say user["name"]
```

JSON objects are represented as `jsondata` values.

This allows parsed objects to use the same indexing and methods as object literals.

## 10. HTTP and WebSocket Modules

Veyra includes networking modules.

### HTTP

```veyra
mod http
```

The `http` module provides HTTP client operations, including support for request headers and response status codes.

### WebSocket

```veyra
mod websocket
```

The `websocket` module provides WebSocket client functionality.

Network operations may fail due to connection problems, timeouts, or invalid responses.

Use `try/catch` when appropriate.

## 11. Math Module

The `math` module provides mathematical operations.

```veyra
mod math

say math.sqrt(144)
```

Output:

```text
12
```

The module also provides additional mathematical functions and constants.

## 12. Time Module

The `time` module provides date, time, and timing operations.

```veyra
mod time

say time.now()
```

The module includes utilities for working with timestamps and date-time values.

Local and UTC time operations may produce different results depending on the system's timezone.

## 13. FS Module

The `fs` module provides filesystem utilities.

```veyra
mod fs
```

It complements the file operations available through `io`.

Some operations can modify or permanently delete files and directories.

For example, `fs.rmtree()` performs recursive directory removal.

Use destructive operations carefully.

## 14. Random Module

The `random` module provides pseudorandom operations.

```veyra
mod random
```

These operations are intended for general-purpose randomness, simulations, and similar tasks.

The module is not designed for cryptographic or security-sensitive randomness.

## 15. Built-in vs. User-Defined Functions

Built-in functions are implemented by the interpreter.

User-defined functions are written in Veyra.

```veyra
add double(x)
    return x * 2
end

say double(5)
say len([1, 2, 3])
```

Output:

```text
10
3
```

Both can be used in expressions and function calls.

User-defined modules can also export functions for use in other Veyra programs.

## 16. Handling Errors

Built-in operations may fail when given invalid values or when external resources are unavailable.

Use `try/catch` to handle runtime errors.

```veyra
mod io

try
    add content = io.read("missing.txt")
    say content
catch err
    say f"Operation failed: {err}"
end
```

Veyra provides source locations and function tracebacks to help diagnose unhandled errors.

## 17. Practice Exercises

### Exercise 1: Type Inspection

Create values of different types and inspect them using `type()`.

### Exercise 2: Text Processing

Create a text value and use `upper()`, `trim()`, `split()`, and `join()`.

### Exercise 3: List Methods

Create a list and experiment with `append()`, `insert()`, and `sort()`.

### Exercise 4: Functional Lists

Create a list of numbers.

Use `map()` to double the numbers, `filter()` to select even numbers, and `reduce()` to calculate their sum.

### Exercise 5: JSONData

Create a `jsondata` object and display its keys and items.

### Exercise 6: JSONData Merging

Create two JSONData objects and combine them using `merge()` and `+`.

### Exercise 7: Math

Import `math` and calculate the square root of a number.

### Exercise 8: File Operations

Use `io` to write and read a text file.

### Exercise 9: JSON Parsing

Use `json.parse()` to convert JSON text into a Veyra object.

### Exercise 10: String Interpolation

Create variables containing a name and a number.

Use an interpolated string to display both values and include a calculation.

## What's Next?

Continue learning Veyra:

- [Modules](modules.md)
- [Lists](lists.md)
- [Values](values.md)
- [Input and Output](io.md)
- [Error Handling](error-handling.md)
