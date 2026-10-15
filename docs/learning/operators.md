# Operators

Operators are symbols or keywords that perform operations on values and variables.

Veyra supports arithmetic, comparison, logical, assignment, and special operators for features such as channels and null-coalescing.

## What You Will Learn

By the end of this lesson, you will know how to:

- Perform arithmetic calculations.
- Compare values.
- Combine conditions with logical operators.
- Update variables using assignment operators.
- Use `??` for fallback values.
- Understand the difference between `?` and `?=`.
- Create lambda expressions with `=>`.
- Work with the variadic and rest operator `...`.

## 1. Arithmetic Operators

Arithmetic operators perform mathematical calculations.

| Operator | Meaning                        | Example  |
| -------- | ------------------------------ | -------- |
| `+`      | Addition or text concatenation | `10 + 5` |
| `-`      | Subtraction                    | `10 - 5` |
| `*`      | Multiplication                 | `10 * 5` |
| `/`      | Division                       | `10 / 5` |
| `%`      | Remainder                      | `10 % 3` |

Example:

```veyra id="v33vbx"
a = 10
b = 3

say a + b
say a - b
say a * b
say a / b
say a % b
```

Output:

```text id="f5rdx2"
13
7
30
3.3333333333333335
1
```

The `%` operator returns the remainder after division.

### Text Concatenation

The `+` operator can also combine text with values.

```veyra id="dzhflp"
name = "Veyra"
version = 1

say "Language: " + name
say "Version: " + version
```

Output:

```text id="t3c56c"
Language: Veyra
Version: 1
```

When working with mixed values, remember that `+` may concatenate text instead of performing numeric addition.

## 2. Comparison Operators

Comparison operators compare two values and produce a Boolean result.

| Operator | Meaning                  | Example  |
| -------- | ------------------------ | -------- |
| `==`     | Equal to                 | `5 == 5` |
| `!=`     | Not equal to             | `5 != 3` |
| `<`      | Less than                | `3 < 5`  |
| `>`      | Greater than             | `8 > 5`  |
| `<=`     | Less than or equal to    | `5 <= 5` |
| `>=`     | Greater than or equal to | `8 >= 5` |

Example:

```veyra id="cq55vm"
say 5 == 5
say 5 != 3
say 3 < 5
say 8 > 5
say 5 <= 5
say 8 >= 5
```

Each expression evaluates to a Boolean value.

Comparison operators are commonly used in conditional statements and loops.

```veyra id="bl6x9a"
score = 85

if score >= 50
    say "Passed."
else
    say "Try again."
end
```

## 3. Logical Operators

Logical operators combine or negate conditions.

| Operator | Meaning                 |
| -------- | ----------------------- |
| `and`    | Logical AND             |
| `or`     | Logical OR              |
| `not`    | Logical NOT             |
| `!`      | Alternative logical NOT |

### Logical AND

`and` evaluates to true when both operands are truthy.

```veyra id="f9thbx"
age = 20
has_ticket = true

if age >= 18 and has_ticket
    say "Access granted."
end
```

### Logical OR

`or` evaluates to true when at least one operand is truthy.

```veyra id="nx9a0j"
is_admin = false
is_owner = true

if is_admin or is_owner
    say "Permission granted."
end
```

### Logical NOT

`not` and `!` negate the truthiness of a value.

```veyra id="s1zjva"
is_ready = false

say not is_ready
say !is_ready
```

Both expressions evaluate to `true`.

Veyra uses truthiness rules when evaluating logical operators. In general, `false`, `null`, numeric zero, and empty text are falsy, while other values are truthy.

## 4. Assignment Operators

Assignment operators store values in variables.

| Operator | Meaning                        | Example  |
| -------- | ------------------------------ | -------- |
| `=`      | Assign a value                 | `x = 10` |
| `+=`     | Add and assign                 | `x += 5` |
| `-=`     | Subtract and assign            | `x -= 2` |
| `*=`     | Multiply and assign            | `x *= 3` |
| `/=`     | Divide and assign              | `x /= 2` |
| `%=`     | Calculate remainder and assign | `x %= 4` |

Example:

```veyra id="b2f2pn"
x = 10

x += 5
say x

x -= 3
say x

x *= 2
say x
```

Output:

```text id="kp84rj"
15
12
24
```

Compound assignment updates the variable using its current value.

## 5. Conditional Assignment: `?=`

The `?=` operator assigns a value only when the target variable has not already been filled or defined.

```veyra id="jj4b0x"
username ?= "guest"

say username
```

Output:

```text id="3q2i8w"
guest
```

If the variable already has a value, the conditional assignment does not overwrite it.

```veyra id="uhbnrn"
username = "alex"
username ?= "guest"

say username
```

Output:

```text id="u1w9z0"
alex
```

This operator is useful when providing defaults without replacing an existing value.

## 6. Null-Coalescing Operator: `??`

The `??` operator returns its left operand unless that operand is `null`. If the left operand is `null`, it returns the right operand.

```veyra id="p7qugo"
name = null

say name ?? "Guest"
```

Output:

```text id="7n0gqd"
Guest
```

If the value is not null, the fallback is not needed.

```veyra id="x9bdxk"
name = "Alex"

say name ?? "Guest"
```

Output:

```text id="2l9h2q"
Alex
```

### `?=` vs. `??`

These operators serve different purposes.

| Operator | Purpose                                                  |
| -------- | -------------------------------------------------------- |
| `?=`     | Conditionally assigns to a variable                      |
| `??`     | Chooses a fallback value when the left operand is `null` |

Use `?=` when you want to initialize a variable only if it has not already been defined or filled. Use `??` when you want an expression to provide a fallback for `null`.

## 7. Channel Operator: `?`

Veyra's `?` operator reads a channel value with a default.

This feature is associated with constants declared using `set`.

```veyra id="j7b71s"
set port = 80

port = 8080

say port
say 0 ? port
```

Output:

```text id="f8yrcv"
80
8080
```

The constant `port` retains its original value, while the assignment updates its channel.

In `0 ? port`, the left operand `0` is the default value. The expression returns the channel value if one exists; otherwise, it returns the default.

For more information, see the [Channels Guide](channels.md).

## 8. Lambda Operator: `=>`

The `=>` operator is used to define a lambda expression with `func`.

```veyra id="v5e4hw"
double = func(x) => x * 2

say double(5)
```

Output:

```text id="7f04fa"
10
```

A lambda is an anonymous function that can be stored in a variable and called later.

You can also define lambdas with multiple parameters.

```veyra id="r2xbz7"
add_numbers = func(a, b) => a + b

say add_numbers(3, 4)
```

Output:

```text id="9v6f0m"
7
```

## 9. Variadic and Rest Operator: `...`

Veyra uses `...` to work with a variable number of function arguments and to capture remaining list elements during destructuring.

### Variadic Parameters

```veyra id="zv1q2v"
add show_values(...values)
    say values
end

show_values(10, 20, 30)
```

The variadic parameter collects the arguments into a list.

### List Destructuring

```veyra id="3ch0d8"
[first, ...rest] = [10, 20, 30, 40]

say first
say rest
```

Output:

```text id="6ov4hx"
10
[20, 30, 40]
```

The first element is assigned to `first`, while the remaining elements are collected into `rest`.

## 10. Operator Reference

The following table summarizes the operators covered in this guide.

| Category               | Operators                         |
| ---------------------- | --------------------------------- |
| Arithmetic             | `+`, `-`, `*`, `/`, `%`           |
| Comparison             | `==`, `!=`, `<`, `>`, `<=`, `>=`  |
| Logical                | `and`, `or`, `not`, `!`           |
| Assignment             | `=`, `+=`, `-=`, `*=`, `/=`, `%=` |
| Conditional assignment | `?=`                              |
| Null coalescing        | `??`                              |
| Channel access         | `?`                               |
| Lambda                 | `=>`                              |
| Variadic/rest          | `...`                             |

## 11. Common Mistakes

### Using `=` Instead of `==`

Use `=` to assign a value and `==` to compare values.

```veyra id="n3i2nd"
score = 100

if score == 100
    say "Perfect!"
end
```

### Confusing `?=` and `??`

`?=` conditionally assigns a variable. `??` returns a fallback when the left operand is `null`.

### Treating `?` as a Boolean Operator

In Veyra, the `?` operator is used for channel access with a default value. It is not a replacement for `if`, `and`, or `or`.

### Mixing Text and Numeric Operations

The `+` operator can concatenate text. If you intend to perform arithmetic, make sure the values are numeric.

## 12. Practice Exercises

### Exercise 1: Arithmetic

Create two numeric variables and display their sum, difference, product, quotient, and remainder.

### Exercise 2: Comparisons

Compare two numbers using all six comparison operators.

### Exercise 3: Logical Conditions

Create two Boolean variables and experiment with `and`, `or`, and `not`.

### Exercise 4: Conditional Assignment

Provide a default username using `?=`. Test what happens when the variable already contains a value.

### Exercise 5: Null Fallback

Create a variable with the value `null` and use `??` to provide a fallback.

### Exercise 6: Lambda

Create a lambda that squares a number and call it with different arguments.

### Exercise 7: Variadic Parameters

Write a function that accepts multiple arguments and displays them as a list.

## What's Next?

You now know the main operators available in Veyra.

Continue with:

- [Variables and Types](variables.md)
- [Control Flow](control-flow.md)
- [Lists](lists.md)
- [Functions](functions.md)
- [Channels](channels.md)
- [Modules](modules.md)
