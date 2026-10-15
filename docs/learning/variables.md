# Variables and Types

In this lesson, you will learn how to store values in variables, understand Veyra's data types, convert values between types, and work with constants.

## What You Will Learn

By the end of this guide, you will know how to:

- Create and update variables.
- Understand Veyra's built-in value types.
- Convert values using `num()`, `text()`, and `type()`.
- Declare constants with `set`.
- Use conditional assignment with `?=`.
- Avoid common variable-related mistakes.

## 1. Creating Variables

A variable is a name that refers to a value.

In Veyra, use `=` to assign a value to a variable.

```veyra
name = "Alex"
age = 20
is_learning = true

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

You do not need to declare a variable's type before assigning a value. Veyra determines the value's type at runtime.

### Updating a Variable

You can assign a new value to an existing variable.

```veyra
score = 10
say score

score = 25
say score
```

Output:

```text
10
25
```

The second assignment replaces the value stored in `score`.

### Assignment Shorthand

Veyra supports compound assignment operators:

```veyra
score = 10

score += 5
say score

score -= 3
say score

score *= 2
say score

score /= 4
say score

score %= 3
say score
```

These operators perform an operation and assign the result back to the same variable.

| Operator | Meaning                            |
| -------- | ---------------------------------- |
| `=`      | Assign a value                     |
| `+=`     | Add and assign                     |
| `-=`     | Subtract and assign                |
| `*=`     | Multiply and assign                |
| `/=`     | Divide and assign                  |
| `%=`     | Calculate the remainder and assign |

## 2. Understanding Data Types

Veyra supports several built-in value types.

| Type name  | Description            | Example            |
| ---------- | ---------------------- | ------------------ |
| `num`      | A number               | `42`               |
| `text`     | Text                   | `"Hello"`          |
| `bool`     | A Boolean value        | `true`             |
| `list`     | A collection of values | `[1, 2, 3]`        |
| `function` | A function value       | `func(x) => x + 1` |
| `null`     | The absence of a value | `null`             |

The `type()` function returns the type name of a value as text.

```veyra
say type(42)
say type("Hello")
say type(true)
say type([1, 2, 3])
say type(null)
```

Output:

```text
num
text
bool
list
null
```

You can also check the type of a variable:

```veyra
value = 42

say type(value)

value = "Now I am text."

say type(value)
```

Output:

```text
num
text
```

Because Veyra is dynamically typed, a variable can hold values of different types at different times.

## 3. Working with Numbers

Numbers can be used in calculations.

```veyra
a = 12
b = 5

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

The `num()` function can convert compatible values into numbers.

```veyra
first = num("42")
second = num(true)
third = num(false)

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

A Boolean value converts to `1` when true and `0` when false.

**Important:** A text value that cannot be converted into a number causes a runtime error.

## 4. Working with Text

Text values are written inside quotation marks.

```veyra
first_name = "Alex"
last_name = "Morgan"

full_name = first_name + " " + last_name

say full_name
```

Output:

```text
Alex Morgan
```

The `+` operator concatenates text.

It can also combine text with other value types:

```veyra
age = 20

say "Age: " + age
```

Output:

```text
Age: 20
```

### Converting Values to Text

Use `text()` to convert a value to its text representation.

```veyra
score = 100
message = text(score)

say message
say type(message)
```

Output:

```text
100
text
```

This is useful when you need to combine a value with a text message or store its textual representation.

## 5. Checking Types with `type()`

The `type()` function returns the type name of a value.

```veyra
username = "guest"
attempts = 3
active = true

say type(username)
say type(attempts)
say type(active)
```

Output:

```text
text
num
bool
```

You can use this function while debugging to understand what kind of value a variable currently contains.

Remember that `type()` returns text, so the result for a number is `"num"`, not `"number"`.

## 6. Declaring Constants with `set`

Use `set` to declare a constant.

```veyra
set app_name = "Veyra"
set version = 1

say app_name
say version
```

Output:

```text
Veyra
1
```

A constant's original value is preserved. However, Veyra has a special feature: assigning to a constant creates or updates its associated **channel value** rather than replacing the constant itself.

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

- `greeting` reads the original constant value.
- `greeting = "Welcome!"` writes to the channel associated with `greeting`.
- `0 ? greeting` reads the channel value, using `0` as the default if the channel has no value.

This behavior is different from ordinary constant declarations in many other programming languages.

For a deeper explanation, see the [Channels Guide](channels.md).

## 7. Conditional Assignment with `?=`

Veyra supports the `?=` operator for conditional assignment.

It assigns a value only when the target variable has not already been filled or defined.

```veyra
username ?= "guest"

say username
```

Output:

```text
guest
```

This can be useful when providing a default value that should not overwrite an existing variable.

For example:

```veyra
username = "alex"
username ?= "guest"

say username
```

Output:

```text
alex
```

Because `username` already has a value, the second assignment does not replace it.

Do not confuse `?=` with `??`:

- `?=` conditionally assigns to a variable.
- `??` returns the left operand unless it is `null`; otherwise, it returns the right operand.

## 8. Understanding `null`

`null` represents the absence of a value.

```veyra
result = null

say result
say type(result)
```

Output:

```text
null
null
```

The first `null` is the displayed value. The second is the type name returned by `type()`.

You can use the `??` operator to provide a fallback when a value is `null`.

```veyra
nickname = null

display_name = nickname ?? "Anonymous"

say display_name
```

Output:

```text
Anonymous
```

If `nickname` contains a non-null value, that value is used instead.

## 9. Common Mistakes

### Using the Wrong Type Name

Incorrect expectation:

```veyra
say type(42)
```

Expected result:

```text
number
```

Actual result:

```text
num
```

Veyra uses `num` as the type name for numbers.

### Converting Invalid Text to a Number

```veyra
value = num("hello")
```

The text `"hello"` cannot be converted into a number, so this operation causes a runtime error.

Use `num()` with values that can be converted successfully.

### Expecting `set` to Behave Like an Ordinary Variable

```veyra
set mode = "safe"
mode = "fast"

say mode
```

The final output remains:

```text
safe
```

The assignment to `mode` updates its channel rather than replacing its constant value. Use ordinary assignment when you want a variable that can be updated normally.

### Confusing `?=` and `??`

These operators have different purposes.

```veyra
value = null
say value ?? "default"
```

The `??` operator provides a fallback value. The `?=` operator conditionally assigns to a variable.

## 10. Practice Exercises

Try solving these exercises without looking at the earlier examples.

### Exercise 1: Personal Information

Create variables for a name, age, and country. Print all three values.

### Exercise 2: Type Detection

Create one variable of each of these types:

- Number
- Text
- Boolean
- List
- Null

Use `type()` to display each variable's type.

### Exercise 3: Temperature Conversion

Create a variable named `celsius` and convert it to Fahrenheit using this formula:

`Fahrenheit = Celsius * 9 / 5 + 32`

Print the result.

### Exercise 4: Text Conversion

Create a numeric variable, convert it to text using `text()`, and print its type.

### Exercise 5: Default Values

Use `?=` to provide a default username. Then assign a username before the conditional assignment and observe the difference.

### Exercise 6: Constants and Channels

Declare a constant using `set`, assign a different value to its channel, and print both the constant and its channel value.

## What's Next?

You now understand Veyra's variables, data types, and assignment behavior.

Continue with:

- [Input and Output](io.md) — receive input and display information.
- [Operators](operators.md) — explore arithmetic, comparison, and logical operators.
- [Control Flow](control-flow.md) — make decisions and repeat instructions.
- [Channels](channels.md) — learn more about constants and channel values.
