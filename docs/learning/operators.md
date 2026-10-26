# Operators

Operators are symbols or keywords that perform operations on values.

Veyra supports arithmetic, comparison, logical, assignment, membership, and special operators for channels, null handling, and functions.

Veyra 0.2.8 introduces compound assignment operators and short-circuit evaluation for logical and null-coalescing expressions.

## What You Will Learn

By the end of this lesson, you will know how to:

- Perform arithmetic operations.
- Compare values.
- Combine and negate conditions.
- Understand short-circuit evaluation.
- Update variables using regular and compound assignment.
- Check collection membership with `in` and `not in`.
- Use `?`, `?=`, and `??`.
- Combine lists and JSONData objects.
- Access and slice collections.
- Create lambdas and variadic parameters.
- Use operators inside interpolated strings.

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

The `+` operator also concatenates two text values.

```veyra
add name = "Veyra"
add version = "0.2.8"

say "Language: " + name
say "Version: " + version
```

Output:

```text
Language: Veyra
Version: 0.2.8
```

Both operands must be text.

To combine text with numbers or other values, use string interpolation:

```veyra
add age = 20

say f"Age: {age}"
say f"Next year: {age + 1}"
```

Output:

```text
Age: 20
Next year: 21
```

Alternatively, convert the value explicitly:

```veyra
say "Age: " + text(age)
```

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

### JSONData Merging

Veyra 0.2.7 introduced JSONData merging with `+`.

```veyra
add user = {"name": "Alex", "age": 20}
add updates = {"age": 21, "active": true}

add merged = user + updates

say merged
```

Output:

```text
{"name": "Alex", "age": 21, "active": true}
```

When both objects contain the same key, the value from the right-hand object takes precedence.

The operation creates a new object without modifying either original.

JSONData merging is shallow. Nested objects are replaced rather than recursively combined.

The equivalent method is:

```veyra
add merged = user.merge(updates)
```

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

Equality operators also support lists and JSONData objects.

```veyra
say [1, 2] == [1, 2]
say {"x": 1} == {"x": 1}
```

Output:

```text
true
true
```

Ordering comparisons such as `<` and `>` support numbers and text.

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

Falsy values include `false`, `null`, `0`, `""`, `[]`, and `{}`.

The `and` and `or` operators return Boolean values.

### Short-Circuit Evaluation

**New in Veyra 0.2.8**

The `and` and `or` operators support short-circuit evaluation.

This means the right operand is evaluated only when necessary.

For `and`, if the left operand is falsy, the right operand is skipped.

```veyra
say false and (10 / 0)
```

Output:

```text
false
```

For `or`, if the left operand is truthy, the right operand is skipped.

```veyra
say true or (10 / 0)
```

Output:

```text
true
```

Neither expression produces a division-by-zero error because the right operand is not evaluated.

Short-circuit evaluation also prevents unnecessary function calls.

```veyra
add dangerous()
    throw "This function should not run"
end

say true or dangerous()
say false and dangerous()
```

Output:

```text
true
false
```

The skipped expression must still be syntactically valid.

For example:

```veyra
say true or (10 + )
```

This produces a syntax error even though the right operand would not be evaluated.

Short-circuit evaluation skips execution, not syntax validation.

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

Use `=` to update an existing mutable variable.

```veyra
add x = 10

x = x + 5
say x

x = x - 3
say x

x = x * 2
say x
```

Output:

```text
15
12
24
```

Use `add` to declare a mutable variable and `=` to update it.

### Compound Assignment Operators

**New in Veyra 0.2.8**

Compound assignment combines an arithmetic operation with assignment.

| Operator | Equivalent operation |
| -------- | -------------------- |
| `+=`     | `x = x + value`      |
| `-=`     | `x = x - value`      |
| `*=`     | `x = x * value`      |
| `/=`     | `x = x / value`      |
| `%=`     | `x = x % value`      |

Example:

```veyra
add score = 10

score += 5
say score

score -= 3
say score

score *= 2
say score

score /= 4
say score

score %= 4
say score
```

Output:

```text
15
12
24
6
2
```

Compound assignment follows the same type rules as the corresponding arithmetic operator.

For example, `+=` supports numbers, text, lists, and JSONData objects.

### Text Compound Assignment

```veyra
add message = "Hello"

message += ", Veyra!"

say message
```

Output:

```text
Hello, Veyra!
```

### List Compound Assignment

```veyra
add numbers = [1, 2]

numbers += [3, 4]

say numbers
```

Output:

```text
[1, 2, 3, 4]
```

The variable receives the resulting list.

### JSONData Compound Assignment

```veyra
add user = {"name": "Alex"}

user += {"active": true}

say user
```

Output:

```text
{"name": "Alex", "active": true}
```

The merge is shallow, and values from the right-hand object take precedence.

### Assignment to Constants

Assigning to a constant declared using `set` does not modify its original value.

Instead, the assignment updates its channel.

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

### Compound Assignment to Constants

Compound assignment also respects channel semantics.

When the target is a constant, the operation uses its current channel value and writes the result back to the channel.

```veyra
set score = 10

score += 5

say score
say 0 ? score
```

Output:

```text
10
15
```

The original constant remains unchanged.

Repeated compound assignments use the latest channel value.

```veyra
set counter = 10

counter += 5
counter += 3

say counter
say 0 ? counter
```

Output:

```text
10
18
```

For mutable variables and regular function parameters, compound assignment updates the variable itself.

Passing a constant to a function does not automatically make a mutable parameter's compound assignment modify the original channel.

### Compound Assignment Limitations

Compound assignment currently supports direct variable names.

For example:

```veyra
add numbers = [10, 20]

numbers[0] = 15
```

Indexed regular assignment is supported, but indexed compound assignment such as `numbers[0] += 5` is not currently supported.

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

### Short-Circuit Null Coalescing

**New in Veyra 0.2.8**

The `??` operator now evaluates its right operand only when the left operand is `null`.

```veyra
say 42 ?? (10 / 0)
```

Output:

```text
42
```

The division is skipped because `42` is not `null`.

When the left operand is `null`, the fallback expression is evaluated.

```veyra
say null ?? (10 + 20)
```

Output:

```text
30
```

This is useful when a fallback expression is expensive or may have side effects.

As with logical short-circuiting, skipped expressions must still be syntactically valid.

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

## 10. Operators in String Interpolation

Veyra 0.2.7 introduced expressions inside interpolated strings.

Use the `f` prefix and place expressions inside `{}`.

```veyra
add a = 10
add b = 20

say f"Sum: {a + b}"
say f"Product: {a * b}"
say f"Equal: {a == b}"
```

Output:

```text
Sum: 30
Product: 200
Equal: false
```

You can also use indexing and function calls:

```veyra
add numbers = [10, 20, 30]

say f"First: {numbers[0]}"
say f"Length: {len(numbers)}"
```

Output:

```text
First: 10
Length: 3
```

Interpolation converts evaluated values into their text representations.

Normal strings without the `f` prefix do not evaluate expressions inside braces.

## 11. Lambda Operator: `=>`

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

Lambdas can be passed to functional list methods:

```veyra
add numbers = [1, 2, 3]

say numbers.map(func(x) => x * 2)
```

Output:

```text
[2, 4, 6]
```

## 12. Variadic and Rest Operator: `...`

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

## 13. Operator Reference

| Category               | Operators                        |
| ---------------------- | -------------------------------- |
| Arithmetic             | `+`, `-`, `*`, `/`, `%`          |
| Comparison             | `==`, `!=`, `<`, `>`, `<=`, `>=` |
| Logical                | `and`, `or`, `not`, `!`          |
| Membership             | `in`, `not in`                   |
| Assignment             | `=`                              |
| Compound assignment    | `+=`, `-=`, `*=`, `/=`, `%=`     |
| Conditional assignment | `?=`                             |
| Null coalescing        | `??`                             |
| Channel access         | `?`                              |
| Indexing               | `[]`                             |
| Slicing                | `[start:end:step]`               |
| Lambda                 | `=>`                             |
| Variadic/rest          | `...`                            |

The `+` operator supports number addition, text concatenation, list concatenation, and JSONData merging.

The `and`, `or`, and `??` operators support short-circuit evaluation.

## 14. Common Mistakes

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

### Expecting Compound Assignment to Modify Constants

```veyra
set score = 10

score += 5

say score
```

Output:

```text
10
```

The original constant is unchanged. The updated channel value is available through:

```veyra
say 0 ? score
```

Output:

```text
15
```

### Combining Text and Numbers with `+`

The `+` operator does not automatically convert numbers to text.

This expression is invalid:

```veyra
add age = 20

say "Age: " + age
```

Use interpolation instead:

```veyra
say f"Age: {age}"
```

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

The same rule applies to JSONData merging.

### Invalid Slice Step

A slice step cannot be zero.

```veyra
add numbers = [1, 2, 3]

say numbers[::0]
```

This causes a runtime error.

### Assuming Short-Circuiting Ignores Syntax Errors

Short-circuit evaluation skips execution, not syntax validation.

This expression is invalid:

```veyra
say true or (10 + )
```

Even though the left operand is truthy, the right operand must be syntactically correct.

## 15. Practice Exercises

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

### Exercise 11: JSONData Merging

Create two JSONData objects containing one overlapping key.

Merge them using `+` and inspect the result.

### Exercise 12: String Interpolation

Create two numeric variables and display their sum, product, and comparison results using interpolated strings.

### Exercise 13: Compound Assignment

Create a mutable numeric variable and update it using all five compound assignment operators.

### Exercise 14: Short-Circuit Evaluation

Write expressions using `and`, `or`, and `??` that safely skip a division-by-zero operation.

### Exercise 15: Constant Channels

Declare a constant, update its channel using `+=`, and compare the original value with the channel value.

## What's Next?

Continue learning Veyra:

- [Variables and Types](variables.md)
- [Control Flow](control-flow.md)
- [Lists](lists.md)
- [Functions](functions.md)
- [Channels](channels.md)
- [Modules](modules.md)
