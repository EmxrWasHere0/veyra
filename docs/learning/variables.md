# Variables and Types

Veyra is a dynamically typed language. Variables can store different kinds of values without requiring explicit type declarations.

In this lesson, you will learn how to declare variables, update their values, inspect types, work with constants, and format values using string interpolation.

## What You Will Learn

By the end of this guide, you will know how to:

- Declare mutable variables using `add`.
- Update variables using `=`.
- Understand Veyra's seven value types.
- Convert values using `num()` and `text()`.
- Inspect values using `type()`.
- Use string interpolation to format values.
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

Output:

```text
[1, 2, 3]
```

### Updating Values with Expressions

You can use expressions when updating variables.

```veyra
add score = 10

score = score + 5
score = score - 3
score = score * 2

say score
```

Output:

```text
24
```

Compound assignment operators such as `+=`, `-=`, `*=`, `/=`, and `%=` are planned for Veyra 0.2.8 and are not available in 0.2.7.

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

Text values can be combined with other text values using `+`.

To include numbers or other types in text, use string interpolation or `text()`.

### String Interpolation

Veyra 0.2.7 supports interpolated strings.

Prefix a string with `f` and place expressions inside `{}`.

```veyra
add name = "Alex"
add age = 20

say f"Name: {name}"
say f"Age: {age}"
say f"Next year: {age + 1}"
```

Output:

```text
Name: Alex
Age: 20
Next year: 21
```

Interpolation can evaluate expressions:

```veyra
add price = 25
add quantity = 4

say f"Total: {price * quantity}"
```

Output:

```text
Total: 100
```

You can also use single-quoted and multiline interpolated strings.

```veyra
add language = "Veyra"

say f'Welcome to {language}!'

say f"""
Language: {language}
Version: 0.2.7
"""
```

To include literal braces, use `{{` and `}}`.

```veyra
add value = 42

say f"Result: {{ {value} }}"
```

Output:

```text
Result: { 42 }
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
set version = "0.2.7"

say app_name
say version
```

Output:

```text
Veyra
0.2.7
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

### Combining Text and Numbers Directly

The `+` operator does not automatically convert numbers into text.

```veyra
add age = 20

say "Age: " + age
```

This causes a runtime error.

Use interpolation instead:

```veyra
say f"Age: {age}"
```

Or convert the number explicitly:

```veyra
say "Age: " + text(age)
```

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

Create one variable for each of Veyra's seven value types.

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

### Exercise 5: String Interpolation

Create variables for a person's name, age, and favorite programming language.

Display a formatted message using an interpolated string.

Include at least one arithmetic expression.

### Exercise 6: Default Values

Use `?=` to assign a default username.

Then repeat the exercise with an already initialized variable.

### Exercise 7: Constants and Channels

Declare a constant using `set`.

Assign a new value to its channel, then print both the original constant and the channel value.

## What's Next?

Continue learning Veyra:

- [Input and Output](io.md)
- [Operators](operators.md)
- [Control Flow](control-flow.md)
- [Channels](channels.md)
- [Values](values.md)
