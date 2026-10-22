# Operators

Operators are symbols or keywords that perform operations on values.

Veyra supports arithmetic, comparison, logical, assignment, membership, and special operators for channels, null handling, and functions.

## What You Will Learn

By the end of this lesson, you will know how to:

- Perform arithmetic operations.
- Compare values.
- Combine and negate conditions.
- Update variables using assignment operators.
- Check collection membership with `in` and `not in`.
- Use `?`, `?=`, and `??`.
- Combine lists.
- Access and slice collections.
- Create lambdas and variadic parameters.

## 1. Arithmetic Operators

Arithmetic operators perform calculations on numbers.

| Operator | Description    |
| -------- | -------------- |
| `+`      | Addition       |
| `-`      | Subtraction    |
| `*`      | Multiplication |
| `/`      | Division       |
| `%`      | Remainder      |

Example:

```veyra
add a = 10
add b = 3

say a + b
say a - b
say a * b
say a / b
say a % b
```

Output:

```text
13
7
30
3.3333333333333335
1
```

### Text Concatenation

The `+` operator also concatenates text.

```veyra
add name = "Veyra"
add version = "0.2.6"

say "Language: " + name
say "Version: " + version
```

Output:

```text
Language: Veyra
Version: 0.2.6
```

Veyra can combine text with other values using `+`.

### List Concatenation

The `+` operator can combine two lists.

```veyra
add first = [1, 2]
add second = [3, 4]

say first + second
```

Output:

```text
[1, 2, 3, 4]
```

The operation returns a new list.

## 2. Comparison Operators

Comparison operators return Boolean values.

| Operator | Description           |
| -------- | --------------------- |
| `==`     | Equal                 |
| `!=`     | Not equal             |
| `<`      | Less than             |
| `>`      | Greater than          |
| `<=`     | Less than or equal    |
| `>=`     | Greater than or equal |

Example:

```veyra
say 5 == 5
say 5 != 3
say 3 < 5
say 8 > 5
say 5 <= 5
say 8 >= 5
```

Output:

```text
true
true
true
true
true
true
```

Comparison operators are commonly used in conditions.

```veyra
add score = 85

if score >= 50
    say "Passed."
else
    say "Try again."
end
```

## 3. Logical Operators

Veyra supports four logical operators.

| Operator | Description             |
| -------- | ----------------------- |
| `and`    | Logical AND             |
| `or`     | Logical OR              |
| `not`    | Logical NOT             |
| `!`      | Alternative logical NOT |

### AND

```veyra
add age = 20
add has_ticket = true

if age >= 18 and has_ticket
    say "Access granted."
end
```

### OR

```veyra
add is_admin = false
add is_owner = true

if is_admin or is_owner
    say "Permission granted."
end
```

### NOT

```veyra
add is_ready = false

say not is_ready
say !is_ready
```

Output:

```text
true
true
```

Logical operations use Veyra's truthiness rules.

Falsy values include `false`, `null`, `0`, `""`, and `[]`.

## 4. Membership Operators

Veyra supports `in` and `not in`.

These operators check whether a value belongs to a collection.

### List Membership

```veyra
add numbers = [10, 20, 30]

say 20 in numbers
say 50 not in numbers
```

Output:

```text
true
true
```

### Text Membership

For strings, membership checks whether a substring exists.

```veyra
add message = "Hello, Veyra!"

say "Veyra" in message
say "Python" not in message
```

Output:

```text
true
true
```

### JSONData Membership

For `jsondata`, membership checks object keys.

```veyra
add user = {
    "name": "Alex",
    "age": 20
}

say "name" in user
say "email" not in user
```

Output:

```text
true
true
```

## 5. Assignment Operators

Assignment operators update variables.

| Operator | Description          |
| -------- | -------------------- |
| `=`      | Assign               |
| `+=`     | Add and assign       |
| `-=`     | Subtract and assign  |
| `*=`     | Multiply and assign  |
| `/=`     | Divide and assign    |
| `%=`     | Remainder and assign |

Example:

```veyra
add x = 10

x += 5
say x

x -= 3
say x

x *= 2
say x
```

Output:

```text
15
12
24
```

Use `add` to declare a mutable variable and assignment operators to update it.

## 6. Conditional Assignment: `?=`

The `?=` operator conditionally assigns a value.

It is commonly used to provide a default for an undefined name.

```veyra
username ?= "guest"

say username
```

Output:

```text
guest
```

An existing value is preserved.

```veyra
add username = "alex"

username ?= "guest"

say username
```

Output:

```text
alex
```

This makes `?=` useful for default initialization.

## 7. Null Coalescing: `??`

The `??` operator provides a fallback when the left operand is `null`.

```veyra
add name = null

say name ?? "Guest"
```

Output:

```text
Guest
```

If the value is not null, the original value is returned.

```veyra
add name = "Alex"

say name ?? "Guest"
```

Output:

```text
Alex
```

Unlike `?=`, this operator does not assign a new value to a variable.

## 8. Channel Operator: `?`

The `?` operator reads a channel value with a fallback.

```veyra
set port = 80

port = 8080

say port
say 0 ? port
```

Output:

```text
80
8080
```

The original constant remains unchanged.

The general form is:

```text
fallback ? channel_name
```

If a channel value exists, it is returned. Otherwise, the fallback is returned.

Channels can also be carried through function arguments.

See [Channels](channels.md) for details.

## 9. Indexing and Slicing

Square brackets provide access to elements in collections.

### Indexing

```veyra
add numbers = [10, 20, 30]

say numbers[0]
say numbers[-1]
```

Output:

```text
10
30
```

Strings also support indexing.

```veyra
add word = "Veyra"

say word[0]
```

Output:

```text
V
```

JSONData objects support key-based access.

```veyra
add user = {"name": "Alex"}

say user["name"]
```

Output:

```text
Alex
```

### Slicing

Use slicing to extract a range from a list or string.

Syntax:

```text
collection[start:end:step]
```

Example:

```veyra
add numbers = [10, 20, 30, 40, 50]

say numbers[1:4]
say numbers[::2]
say numbers[::-1]
```

Output:

```text
[20, 30, 40]
[10, 30, 50]
[50, 40, 30, 20, 10]
```

The ending index is exclusive.

For more examples, see [Lists](lists.md).

## 10. Lambda Operator: `=>`

The `=>` operator creates a lambda expression together with `func`.

```veyra
add double = func(x) => x * 2

say double(5)
```

Output:

```text
10
```

Lambdas can also accept multiple parameters.

```veyra
add add_numbers = func(a, b) => a + b

say add_numbers(3, 4)
```

Output:

```text
7
```

## 11. Variadic and Rest Operator: `...`

The `...` syntax is used with variadic parameters and list destructuring.

### Variadic Parameters

```veyra
add show_values(...values)
    say values
end

show_values(10, 20, 30)
```

The arguments are collected into a list.

### List Destructuring

```veyra
[first, ...rest] = [10, 20, 30, 40]

say first
say rest
```

Output:

```text
10
[20, 30, 40]
```

## 12. Operator Reference

| Category               | Operators                         |
| ---------------------- | --------------------------------- |
| Arithmetic             | `+`, `-`, `*`, `/`, `%`           |
| Comparison             | `==`, `!=`, `<`, `>`, `<=`, `>=`  |
| Logical                | `and`, `or`, `not`, `!`           |
| Membership             | `in`, `not in`                    |
| Assignment             | `=`, `+=`, `-=`, `*=`, `/=`, `%=` |
| Conditional assignment | `?=`                              |
| Null coalescing        | `??`                              |
| Channel access         | `?`                               |
| Indexing               | `[]`                              |
| Slicing                | `[start:end:step]`                |
| Lambda                 | `=>`                              |
| Variadic/rest          | `...`                             |

## 13. Common Mistakes

### Assignment vs. Comparison

Use `=` for assignment and `==` for comparison.

```veyra
add score = 100

if score == 100
    say "Perfect!"
end
```

### Confusing Channel and Null Operators

The three operators have different purposes:

- `?` reads a channel.
- `?=` conditionally assigns a value.
- `??` provides a fallback for `null`.

### Expecting List Concatenation to Modify the Original

```veyra
add first = [1, 2]
add second = [3, 4]

first + second
```

This creates a new list but does not modify `first`.

Use assignment to keep the result:

```veyra
first = first + second
```

### Invalid Slice Step

A slice step cannot be zero.

```veyra
add numbers = [1, 2, 3]

say numbers[::0]
```

This causes a runtime error.

## 14. Practice Exercises

### Exercise 1: Arithmetic

Create two numbers and perform all five arithmetic operations.

### Exercise 2: Comparisons

Compare two values using comparison operators.

### Exercise 3: Logical Conditions

Use `and`, `or`, and `not` in conditional statements.

### Exercise 4: Membership

Check whether a list contains a number and whether a string contains a substring.

### Exercise 5: Conditional Assignment

Use `?=` to initialize a name only when it has not already been defined.

### Exercise 6: Null Fallback

Use `??` to replace a `null` value with a default.

### Exercise 7: List Concatenation

Combine two lists using `+`.

### Exercise 8: Slicing

Create a list and extract every second element.

### Exercise 9: Lambda

Create a lambda that squares a number.

### Exercise 10: Channels

Declare a constant with `set`, update its channel, and read the channel using `?`.

## What's Next?

Continue learning Veyra:

- [Variables and Types](variables.md)
- [Control Flow](control-flow.md)
- [Lists](lists.md)
- [Functions](functions.md)
- [Channels](channels.md)
- [Modules](modules.md)
