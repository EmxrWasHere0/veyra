# Control Flow

Control flow determines which instructions a program executes and how many times it executes them.

In Veyra, you can use conditions to make decisions, loops to repeat instructions, and loop-control statements to change how a loop executes.

## What You Will Learn

By the end of this lesson, you will know how to:

- Use `if`, `else if`, and `else`.
- Repeat instructions with `repeat`.
- Create condition-based loops with `repeat while`.
- Stop a loop using `stop`.
- Skip to the next iteration using `fwd`.
- Control nested loops with labels.

## 1. Conditional Statements

Conditional statements allow a program to make decisions.

### The `if` Statement

Use `if` to execute a block when a condition is true.

```veyra
age = 18

if age >= 18
    say "You are an adult."
end
```

Output:

```text
You are an adult.
```

If the condition is false, the block is skipped.

```veyra
age = 15

if age >= 18
    say "You are an adult."
end

say "Program finished."
```

Output:

```text
Program finished.
```

The `end` keyword closes the conditional block.

### The `else` Statement

Use `else` to execute an alternative block when the condition is false.

```veyra
temperature = 12

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

Only one of the two branches executes.

### The `else if` Statement

Use `else if` to check multiple conditions in sequence.

```veyra
score = 75

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

Veyra checks the conditions from top to bottom. Once a matching branch is found, the remaining branches are skipped.

**Tip:** Put more specific or higher-priority conditions first when their order affects the result.

## 2. Comparison Operators

Conditions commonly use comparison operators.

| Operator | Meaning                  | Example   |
| -------- | ------------------------ | --------- |
| `==`     | Equal to                 | `x == 10` |
| `!=`     | Not equal to             | `x != 10` |
| `<`      | Less than                | `x < 10`  |
| `>`      | Greater than             | `x > 10`  |
| `<=`     | Less than or equal to    | `x <= 10` |
| `>=`     | Greater than or equal to | `x >= 10` |

Example:

```veyra
password = "veyra"

if password == "veyra"
    say "Correct value."
else
    say "Incorrect value."
end
```

These operators are useful for checking numbers, comparing text, and controlling program behavior.

## 3. Logical Operators

Logical operators let you combine conditions.

| Operator | Meaning                             |
| -------- | ----------------------------------- |
| `and`    | Both conditions must be true        |
| `or`     | At least one condition must be true |
| `not`    | Negates a condition                 |
| `!`      | Another form of logical negation    |

Example using `and`:

```veyra
age = 20
has_ticket = true

if age >= 18 and has_ticket
    say "Access granted."
else
    say "Access denied."
end
```

Example using `or`:

```veyra
is_admin = false
is_owner = true

if is_admin or is_owner
    say "You can manage this resource."
end
```

Example using `not`:

```veyra
is_ready = false

if not is_ready
    say "The program is not ready."
end
```

Veyra evaluates logical operators using its truthiness rules. Values such as `false`, `null`, numeric zero, and empty text are falsy; other values are truthy.

## 4. Repeating Instructions with `repeat`

Use `repeat` to execute a block a specified number of times.

```veyra
repeat 5
    say "Hello!"
end
```

Output:

```text
Hello!
Hello!
Hello!
Hello!
Hello!
```

The block executes five times.

### Using a Variable as the Count

You can also provide a variable containing the number of repetitions.

```veyra
count = 3

repeat count
    say "Veyra"
end
```

Output:

```text
Veyra
Veyra
Veyra
```

### Counting with a Variable

To control the value displayed on each iteration, update a variable inside the loop.

```veyra
counter = 1

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

The repetition count is fixed when the loop begins. Changing `counter` affects the displayed value, not the number of repetitions.

## 5. Condition-Based Loops with `repeat while`

Use `repeat while` to execute a block as long as a condition remains true.

```veyra
counter = 1

repeat while counter <= 5
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

Before each iteration, Veyra checks the condition.

If the condition is false before the first iteration, the loop body does not run.

### Avoiding Infinite Loops

Make sure a loop's condition can eventually become false.

For example:

```veyra
counter = 1

repeat while counter <= 3
    say counter
    counter += 1
end
```

Here, `counter` increases on every iteration, so the condition eventually becomes false.

If you forget to update the variable, a loop may continue indefinitely or until Veyra's loop execution limit is reached.

## 6. Stopping a Loop with `stop`

Use `stop` to exit the current loop before it finishes naturally.

```veyra
counter = 1

repeat while counter <= 10
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

When `counter` reaches `4`, `stop` exits the loop immediately.

Instructions after the loop continue to execute.

```veyra
repeat 10
    stop
end

say "The loop has ended."
```

Output:

```text
The loop has ended.
```

## 7. Skipping an Iteration with `fwd`

Use `fwd` to skip the remainder of the current iteration and continue with the next one.

```veyra
counter = 0

repeat while counter < 5
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

When `counter` reaches `3`, `fwd` skips the `say counter` instruction for that iteration. The loop then continues.

**Important:** In a `repeat while` loop, make sure the condition's variables are updated before `fwd` when necessary. Otherwise, the loop may fail to make progress.

## 8. Nested Loops

A nested loop is a loop inside another loop.

```veyra
repeat 3
    repeat 2
        say "Hello!"
    end
end
```

Output:

```text
Hello!
Hello!
Hello!
Hello!
Hello!
Hello!
```

The inner loop executes twice for every iteration of the outer loop. Therefore, the message is printed six times.

Nested loops are useful for working with grids, combinations, and multi-dimensional data.

## 9. Controlling Nested Loops with Labels

Sometimes you want to stop or skip an iteration of an outer loop from inside a nested loop.

Veyra supports loop labels for this purpose.

### Labeling a Loop

Place a label immediately before a loop using `@label`.

```veyra
@outer repeat 3
    say "Outer loop"

    repeat 3
        say "Inner loop"
        stop @outer
    end
end

say "Finished."
```

Output:

```text
Outer loop
Inner loop
Finished.
```

The statement `stop @outer` exits the loop labeled `outer`, even though the statement appears inside the inner loop.

### Skipping an Outer Loop Iteration

You can also use `fwd` with a label.

```veyra
@outer repeat 3
    repeat 3
        fwd @outer
    end

    say "This line is skipped."
end

say "Finished."
```

The labeled `fwd` skips the remainder of the current iteration of the outer loop, including any remaining instructions inside it.

This is useful when a condition inside a nested loop should control the outer loop directly.

## 10. Understanding Loop Limits

Veyra enforces a maximum loop execution count to help prevent runaway loops.

The current runtime uses a limit of 1,000,000 loop iterations. Deeply nested or excessively long loops may reach this limit.

Keep loops efficient and ensure that condition-based loops make progress toward termination.

## 11. Common Mistakes

### Forgetting `end`

Incorrect:

```veyra
if score >= 50
    say "Passed."
```

Correct:

```veyra
if score >= 50
    say "Passed."
end
```

Always close the block with `end`.

### Using Assignment Instead of Comparison

Use `==` to compare values.

```veyra
if score == 100
    say "Perfect score!"
end
```

Use `=` when assigning a value to a variable.

### Forgetting to Update a Loop Variable

Incorrect:

```veyra
counter = 1

repeat while counter <= 5
    say counter
end
```

The condition remains true because `counter` never changes.

Correct:

```veyra
counter = 1

repeat while counter <= 5
    say counter
    counter += 1
end
```

### Using `fwd` Without Making Progress

In a condition-based loop, skipping the rest of the body may prevent a required update from running.

Place essential updates before `fwd`, or restructure the loop so that its condition can eventually become false.

### Confusing `stop` and `fwd`

- `stop` exits the targeted loop.
- `fwd` skips the remainder of the targeted iteration and continues with the next iteration.

## 12. Practice Exercises

### Exercise 1: Number Classification

Create a variable named `number`.

Use `if` and `else` to print whether the number is greater than or equal to zero.

### Exercise 2: Grade Calculator

Create a `score` variable and use `else if` to display a grade based on its value.

### Exercise 3: Count to Twenty

Use `repeat while` to display the numbers from `1` through `20`.

### Exercise 4: Print Only Even Numbers

Use a loop and a conditional statement to display the even numbers between `1` and `20`.

Hint: Use the `%` operator to check the remainder after division by `2`.

### Exercise 5: Stop at a Target

Create a loop that counts upward and uses `stop` to exit when the counter reaches a chosen value.

### Exercise 6: Skip a Number

Print the numbers from `1` through `10`, but skip `5` using `fwd`.

### Exercise 7: Nested Loops

Use two nested loops to print a simple grid of messages.

### Exercise 8: Labeled Loop Control

Create nested loops and use a label with `stop` to exit the outer loop from inside the inner loop.

## What's Next?

You now know how to control the execution of a Veyra program.

Continue with:

- [Variables and Types](variables.md)
- [Input and Output](io.md)
- [Operators](operators.md)
- [Lists](lists.md)
- [Functions](functions.md)
- [Error Handling](error-handling.md)
