# Variables and Types

Veyra is a dynamically typed language. Variables can store different kinds of values without requiring explicit type declarations.

In this lesson, you will learn how to declare variables, update their values, inspect types, and work with constants.

## What You Will Learn

By the end of this guide, you will know how to:

- Declare mutable variables using `add`.
- Update variables using assignment operators.
- Understand Veyra's seven value types.
- Convert values using `num()` and `text()`.
- Inspect values using `type()`.
- Declare constants using `set`.
- Use conditional assignment with `?=`.
- Understand the difference between constants and channels.

## 1. Creating Variables

Use `add` to declare a mutable variable.

```veyra
add name = "Alex"
add age = 20
add is_learning = true

say name
say age
say is_learning
```

Output:

```text
Alex
20
true
```

Veyra determines the type of each value at runtime.

### Updating Variables

Use `=` to update an existing variable.

```veyra
add score = 10

say score

score = 25

say score
```

Output:

```text
10
25
```

Variables can also change types:

```veyra
add value = 42

value = "Hello"
value = [1, 2, 3]

say value
```

### Compound Assignment

Veyra supports compound assignment operators:

```veyra
add score = 10

score += 5
score -= 3
score *= 2
score /= 4
score %= 3

say score
```

| Operator | Description                        |
| -------- | ---------------------------------- |
| `=`      | Assign a value                     |
| `+=`     | Add and assign                     |
| `-=`     | Subtract and assign                |
| `*=`     | Multiply and assign                |
| `/=`     | Divide and assign                  |
| `%=`     | Calculate the remainder and assign |

## 2. Understanding Data Types

Veyra supports seven value types.

| Type       | Description         | Example            |
| ---------- | ------------------- | ------------------ |
| `number`   | Numeric values      | `42`, `3.14`       |
| `text`     | Text strings        | `"Hello"`          |
| `bool`     | Boolean values      | `true`, `false`    |
| `list`     | Ordered collections | `[1, 2, 3]`        |
| `jsondata` | Key-value objects   | `{"name": "Alex"}` |
| `function` | Callable values     | `func(x) => x + 1` |
| `null`     | Absence of a value  | `null`             |

Use `type()` to inspect a value.

```veyra
say type(42)
say type("Hello")
say type(true)
say type([1, 2, 3])
say type({"name": "Alex"})
say type(null)
```

Output:

```text
number
text
bool
list
jsondata
null
```

Because Veyra is dynamically typed, the same variable can hold different types during execution.

```veyra
add value = 42

say type(value)

value = "Hello"

say type(value)
```

Output:

```text
number
text
```

## 3. Working with Numbers

Veyra uses a single numeric type called `number`.

```veyra
add a = 12
add b = 5

say a + b
say a - b
say a * b
say a / b
say a % b
```

Output:

```text
17
7
60
2.4
2
```

### Converting Values to Numbers

Use `num()` to convert compatible values into numbers.

```veyra
add first = num("42")
add second = num(true)
add third = num(false)

say first
say second
say third
```

Output:

```text
42
1
0
```

Invalid numeric text causes a runtime error.

```veyra
num("hello")
```

Veyra reports an error because `"hello"` cannot be converted into a number.

## 4. Working with Text

Text values can use single or double quotes.

```veyra
add first_name = "Alex"
add last_name = 'Morgan'

add full_name = first_name + " " + last_name

say full_name
```

Output:

```text
Alex Morgan
```

Veyra supports combining text with other values:

```veyra
add age = 20

say "Age: " + age
```

Output:

```text
Age: 20
```

### Converting Values to Text

Use `text()` to convert a value into text.

```veyra
add score = 100
add message = text(score)

say message
say type(message)
```

Output:

```text
100
text
```

Text values also support methods such as `upper()`, `lower()`, `trim()`, `split()`, and `replace()`.

## 5. Dynamic Typing

A variable's type depends on its current value.

```veyra
add data = 10

say type(data)

data = "Veyra"

say type(data)

data = [1, 2, 3]

say type(data)
```

Output:

```text
number
text
list
```

There are no mandatory type annotations.

## 6. Constants with `set`

Use `set` to declare a constant.

```veyra
set app_name = "Veyra"
set version = "0.2.6"

say app_name
say version
```

Output:

```text
Veyra
0.2.6
```

Unlike mutable variables, constants preserve their original values.

However, Veyra has a special behavior for assignments to constants.

### Constants and Channels

When assigning a new value to a constant, Veyra updates its associated channel instead.

```veyra
set greeting = "Hello"

greeting = "Welcome!"

say greeting
say 0 ? greeting
```

Output:

```text
Hello
Welcome!
```

Here:

- `greeting` returns the original constant value.
- `greeting = "Welcome!"` updates its channel.
- `0 ? greeting` reads the channel value.

The left side of `?` acts as a fallback when the channel has no value.

For a detailed explanation, see [Channels](channels.md).

## 7. Conditional Assignment with `?=`

The `?=` operator assigns a value only when its target meets Veyra's conditional assignment rules.

For example, it can provide a default value for an undefined name:

```veyra
username ?= "guest"

say username
```

Output:

```text
guest
```

If the variable already has a value, the conditional assignment does not replace it.

```veyra
add username = "alex"

username ?= "guest"

say username
```

Output:

```text
alex
```

### Difference Between `?=` and `??`

These operators serve different purposes.

`?=` conditionally assigns a value:

```veyra
username ?= "guest"
```

`??` provides a fallback when an expression evaluates to `null`:

```veyra
add nickname = null

say nickname ?? "Anonymous"
```

Output:

```text
Anonymous
```

## 8. Understanding `null`

The `null` value represents the absence of a value.

```veyra
add result = null

say result
say type(result)
```

Output:

```text
null
null
```

Use `??` when you want to replace a `null` result with another value.

```veyra
add nickname = null

add display_name = nickname ?? "Anonymous"

say display_name
```

Output:

```text
Anonymous
```

## 9. Common Mistakes

### Using the Wrong Type Name

The numeric type is called `number`, not `num`.

```veyra
say type(42)
```

Output:

```text
number
```

`num()` is a conversion function, while `"number"` is the type name.

### Converting Invalid Text

```veyra
add value = num("hello")
```

This causes a runtime error because the text cannot be converted into a number.

### Expecting Constants to Change

```veyra
set mode = "safe"

mode = "fast"

say mode
say "" ? mode
```

Output:

```text
safe
fast
```

The original constant remains unchanged.

### Confusing `?=` with `??`

```veyra
add value = null

say value ?? "default"
```

The `??` operator evaluates an expression and provides a fallback.

The `?=` operator conditionally assigns a value.

## 10. Practice Exercises

### Exercise 1: Personal Information

Create mutable variables for your name, age, and country using `add`.

Print their values.

### Exercise 2: Type Detection

Create one variable for each of these types:

- Number
- Text
- Boolean
- List
- JSONData
- Null

Use `type()` to display their types.

### Exercise 3: Temperature Conversion

Create a variable named `celsius`.

Convert it to Fahrenheit using:

```text
Fahrenheit = Celsius * 9 / 5 + 32
```

Print the result.

### Exercise 4: Text Conversion

Create a numeric variable, convert it to text using `text()`, and inspect the result with `type()`.

### Exercise 5: Default Values

Use `?=` to assign a default username.

Then repeat the exercise with an already initialized variable.

### Exercise 6: Constants and Channels

Declare a constant using `set`.

Assign a new value to its channel, then print both the original constant and the channel value.

## What's Next?

Continue learning Veyra:

- [Input and Output](io.md)
- [Operators](operators.md)
- [Control Flow](control-flow.md)
- [Channels](channels.md)
- [Values](values.md)
