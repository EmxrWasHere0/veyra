# Control Flow

Control flow determines which instructions a program executes and how many times they execute.

Veyra supports conditional statements, counted loops, conditional loops, collection iteration, and labeled loop control.

## What You Will Learn

By the end of this lesson, you will know how to:

- Use `if`, `else if`, and `else`.
- Combine conditions with logical operators.
- Repeat instructions using `repeat`.
- Create conditional loops using `while` and `repeat while`.
- Iterate over collections using `repeat item in collection`.
- Exit loops using `stop`.
- Skip iterations using `fwd`.
- Control nested loops using labels.

## 1. Conditional Statements

### If

Use `if` to execute a block when a condition is truthy.

```veyra
add age = 18

if age >= 18
    say "You are an adult."
end
```

Output:

```text
You are an adult.
```

Every conditional block ends with `end`.

### Else

Use `else` to execute an alternative block.

```veyra
add temperature = 12

if temperature > 20
    say "It is warm."
else
    say "It is cool."
end
```

Output:

```text
It is cool.
```

### Else If

Use `else if` to check multiple conditions.

```veyra
add score = 75

if score >= 90
    say "Grade: A"
else if score >= 70
    say "Grade: B"
else if score >= 50
    say "Grade: C"
else
    say "Grade: F"
end
```

Output:

```text
Grade: B
```

Conditions are evaluated from top to bottom. Only the first matching branch executes.

## 2. Comparison Operators

Comparison operators evaluate relationships between values.

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
add password = "veyra"

if password == "veyra"
    say "Access granted."
else
    say "Access denied."
end
```

## 3. Logical Operators

Veyra supports `and`, `or`, `not`, and `!`.

```veyra
add age = 20
add has_ticket = true

if age >= 18 and has_ticket
    say "Access granted."
end
```

Use `or` when at least one condition should be true:

```veyra
add is_admin = false
add is_owner = true

if is_admin or is_owner
    say "Permission granted."
end
```

Use `not` or `!` to negate a condition:

```veyra
add is_ready = false

if not is_ready
    say "Not ready."
end
```

### Truthy and Falsy Values

The following values are falsy:

- `false`
- `null`
- `0`
- `""`
- `[]`

Other values are generally truthy.

## 4. Membership Operators

The `in` and `not in` operators check whether a value belongs to a collection.

### Lists

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

### Text

For text values, membership checks for a substring.

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

### JSONData

For `jsondata` objects, membership checks whether a key exists.

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

## 5. Counted Loops

Use `repeat` to execute a block a specific number of times.

```veyra
repeat 5
    say "Hello!"
end
```

A variable can determine the repetition count:

```veyra
add count = 3

repeat count
    say "Veyra"
end
```

### Counting Manually

```veyra
add counter = 1

repeat 5
    say counter
    counter += 1
end
```

Output:

```text
1
2
3
4
5
```

The repetition count is determined when the loop begins.

## 6. Conditional Loops

Veyra supports conditional loops using `while`.

```veyra
add counter = 1

while counter <= 5
    say counter
    counter += 1
end
```

Output:

```text
1
2
3
4
5
```

The condition is checked before every iteration.

If the condition is initially false, the body does not execute.

### Repeat While

Veyra also supports the `repeat while` form:

```veyra
add counter = 1

repeat while counter <= 3
    say counter
    counter += 1
end
```

Output:

```text
1
2
3
```

Both forms provide condition-based repetition.

### Avoiding Infinite Loops

Always ensure that a conditional loop can make progress.

```veyra
add counter = 0

while counter < 3
    say counter
    counter += 1
end
```

Without the increment, the condition would remain true until the runtime iteration limit is reached.

## 7. Collection Iteration

Use `repeat item in collection` to iterate directly over a collection.

### Lists

```veyra
add colors = ["red", "green", "blue"]

repeat color in colors
    say color
end
```

Output:

```text
red
green
blue
```

### Text

Text values can also be iterated over.

```veyra
add word = "Veyra"

repeat character in word
    say character
end
```

Each iteration processes one character.

### JSONData

When iterating over a `jsondata` object, the loop variable receives a key.

```veyra
add user = {
    "name": "Alex",
    "age": 20
}

repeat key in user
    say key
    say user[key]
end
```

This allows you to process the object's keys and values.

## 8. Stopping Loops

Use `stop` to exit a loop.

```veyra
add counter = 1

while counter <= 10
    if counter == 4
        stop
    end

    say counter
    counter += 1
end
```

Output:

```text
1
2
3
```

Execution continues after the loop.

## 9. Skipping Iterations

Use `fwd` to skip the remainder of the current iteration.

```veyra
add counter = 0

while counter < 5
    counter += 1

    if counter == 3
        fwd
    end

    say counter
end
```

Output:

```text
1
2
4
5
```

When using `fwd` in conditional loops, make sure required updates happen before skipping the iteration.

## 10. Nested Loops

A loop can contain another loop.

```veyra
repeat 3
    repeat 2
        say "Hello!"
    end
end
```

The inner loop executes twice for each outer iteration.

Nested loops are useful for processing grids, matrices, and combinations.

## 11. Labeled Loops

Labels allow `stop` and `fwd` to target an outer loop.

Only `repeat` loops can have labels.

### Labeled Stop

```veyra
@outer repeat 3
    repeat 3
        stop @outer
    end
end

say "Finished."
```

Output:

```text
Finished.
```

The labeled `stop` exits the outer loop directly.

### Labeled Fwd

```veyra
@outer repeat 3
    repeat 3
        fwd @outer
    end

    say "Skipped"
end

say "Finished."
```

Output:

```text
Finished.
```

The labeled `fwd` skips the rest of the outer loop's current iteration.

A label must refer to an active labeled loop.

## 12. Loop Safety Limits

Veyra enforces a maximum of **1,000,000 iterations per loop**.

This limit prevents programs from running indefinitely due to incorrect loop conditions.

Loops should still be written with clear termination conditions.

## 13. Common Mistakes

### Forgetting End

Incorrect:

```veyra
if true
    say "Hello"
```

Correct:

```veyra
if true
    say "Hello"
end
```

### Using Assignment Instead of Comparison

Use `==` when comparing values.

```veyra
add score = 100

if score == 100
    say "Perfect!"
end
```

### Forgetting to Update a Loop Variable

Incorrect:

```veyra
add counter = 0

while counter < 5
    say counter
end
```

The condition never changes.

Correct:

```veyra
add counter = 0

while counter < 5
    say counter
    counter += 1
end
```

### Incorrect Label Placement

Labels belong to `repeat` loops.

Valid:

```veyra
@outer repeat 5
    stop @outer
end
```

Do not attach labels to `if`, `while`, or function definitions.

### Confusing Stop and Fwd

- `stop` exits the targeted loop.
- `fwd` skips the rest of the targeted iteration.

## 14. Practice Exercises

### Exercise 1: Number Classification

Create a number and determine whether it is positive, negative, or zero.

### Exercise 2: Grade Calculator

Use `if`, `else if`, and `else` to assign a grade based on a score.

### Exercise 3: Count to Twenty

Use `while` to print numbers from 1 to 20.

### Exercise 4: Even Numbers

Print all even numbers between 1 and 20.

### Exercise 5: Collection Iteration

Create a list of names and print each name using `repeat item in collection`.

### Exercise 6: Membership

Check whether a list contains a specific value using `in`.

### Exercise 7: Nested Loops

Create two nested loops and print a message during each iteration.

### Exercise 8: Labeled Loop Control

Use `stop @label` to exit an outer loop from an inner loop.

## What's Next?

Continue learning Veyra:

- [Variables and Types](variables.md)
- [Operators](operators.md)
- [Lists](lists.md)
- [Functions](functions.md)
- [Error Handling](error-handling.md)
