# Error Handling

Errors can occur while a Veyra program is running. A calculation may fail, a file may not exist, or a function may receive an invalid value.

Veyra provides `try`, `catch`, and `throw` for handling runtime errors.

Since version 0.2.6, runtime errors include source locations and function tracebacks to make debugging easier.

Veyra 0.2.7 introduced string interpolation, making it easier to construct descriptive error messages.

**Veyra 0.2.8 improves error reporting with column-based source locations and more precise token highlighting.** It also introduces short-circuit evaluation for `and`, `or`, and `??`.

## What You Will Learn

By the end of this guide, you will know how to:

- Understand runtime errors.
- Handle errors using `try` and `catch`.
- Raise errors explicitly using `throw`.
- Use optional catch variables.
- Handle errors in nested blocks.
- Construct error messages using string interpolation.
- Read file, line, and column information.
- Understand function and lambda tracebacks.
- Prevent unnecessary runtime errors with short-circuit evaluation.
- Avoid hiding important failures.

## 1. Understanding Runtime Errors

A runtime error occurs when an operation cannot complete normally.

Common examples include:

- Division by zero.
- Invalid operations between values.
- Undefined variables.
- Invalid list indexes.
- Incorrect function arguments.
- File or network failures.

Example:

```veyra
add result = 10 / 0

say result
```

This causes a division-by-zero error.

If the error is not handled, Veyra stops execution and reports the failure.

## 2. Using Try and Catch

Use `try` to execute code that may fail.

If an error occurs, Veyra transfers execution to the `catch` block.

```veyra
try
    add result = 10 / 0
    say result
catch err
    say f"An error occurred: {err}"
end
```

The `err` variable contains the error message.

When an error occurs, the remaining statements in the `try` block are skipped.

### Catch Without a Variable

The catch variable is optional.

```veyra
try
    throw "Example error"
catch
    say "An error was handled."
end
```

Use this form when you do not need the error message.

## 3. Throwing Errors

Use `throw` to raise a runtime error explicitly.

```veyra
throw "Something went wrong"
```

You can throw errors based on conditions.

```veyra
add divide(a, b)
    if b == 0
        throw "The denominator cannot be zero"
    end

    return a / b
end
```

Handle the error with `try/catch`:

```veyra
try
    say divide(10, 0)
catch err
    say f"Calculation failed: {err}"
end
```

### Throwing Expressions

`throw` accepts an expression.

```veyra
add name = "example.txt"

throw f"Unable to open file: {name}"
```

This is useful for producing descriptive error messages.

### Throwing Interpolated Strings

Veyra 0.2.7 introduced evaluating expressions inside interpolated strings.

```veyra
add expected = 10
add received = 5

if received != expected
    throw f"Expected {expected}, but received {received}"
end
```

You can also include function calls and calculations in error messages.

```veyra
add validate_age(age)
    if age < 0
        throw f"Invalid age: {age}"
    end

    return age
end
```

## 4. Handling File Errors

File operations may fail because a path does not exist or permissions are insufficient.

Import the `io` module before using its functions.

```veyra
mod io

try
    add content = io.read("notes.txt")
    say content
catch err
    say f"Could not read the file: {err}"
end
```

### Writing Files

```veyra
mod io

try
    io.write("output.txt", "Hello from Veyra!")
    say "File written successfully."
catch err
    say f"Could not write the file: {err}"
end
```

Only report success after the operation finishes without raising an error.

## 5. Nested Error Handling

A `try/catch` block can contain another `try/catch` block.

```veyra
try
    say "Starting operation"

    try
        throw "Inner error"
    catch inner_error
        say f"Handled inner error: {inner_error}"
    end

    say "Continuing operation"
catch outer_error
    say f"Outer error: {outer_error}"
end
```

If the inner handler successfully handles the error, execution can continue in the outer block.

Nested handlers are useful when different operations require different recovery strategies.

## 6. Runtime Error Locations

Veyra 0.2.6 introduced source locations in runtime errors.

**Veyra 0.2.8 extends this system with column-based diagnostics.**

Errors can now include:

- The source filename.
- The line number.
- The column number.
- The error description.
- The relevant source line.
- A marker highlighting the problematic token or operator.

### Column-Based Error Messages

Consider a file named `example.vey`:

```veyra
say 10 / 0
```

Running the file produces an error similar to:

```text
Runtime Error: example.vey:1:8: Division by zero

  1 | say 10 / 0
    |        ^
```

The location uses the following format:

```text
filename:line:column
```

In this example:

- `example.vey` is the source file.
- `1` is the line number.
- `8` is the column containing the `/` operator.

The `^` marker points to the operator responsible for the error.

### Undefined Variables

Veyra can also highlight undefined identifiers.

```veyra
say 10 + unknown
```

Example error:

```text
Runtime Error: example.vey:1:10: Undefined 'unknown'

  1 | say 10 + unknown
    |          ^^^^^^^
```

The marker covers the identifier rather than the entire line.

### Multiline Expressions

Source locations also work with multiline expressions.

```veyra
add numbers = [
    10,
    20,
    unknown
]
```

Example error:

```text
Runtime Error: example.vey:4:5: Undefined 'unknown'

  4 |     unknown
    |     ^^^^^^^
```

The error points to the original physical source line, even when Veyra processes the expression as a single logical statement.

### Indexing Errors

Veyra can identify invalid indexes.

```veyra
add numbers = [10, 20, 30]

say numbers[999]
```

Example error:

```text
Runtime Error: example.vey:3:13: Index out of bounds

  3 | say numbers[999]
    |             ^^^
```

Column-based diagnostics also improve errors involving JSONData keys, invalid slicing arguments, and unsupported indexing operations.

### Diagnostic Limitations

Column-based diagnostics cover many common runtime and expression errors.

However, not every error can be associated with an exact token.

Some errors may still point to the beginning of a statement or expression.

Error messages originating from external modules may also have less precise locations.

## 7. Function Tracebacks

When an error occurs inside a function, Veyra can display the function call chain.

Example:

```veyra
add divide(a, b)
    return a / b
end

add calculate()
    return divide(10, 0)
end

calculate()
```

Example error:

```text
Runtime Error: example.vey:2:14: Division by zero

  2 |     return a / b
    |              ^
  at example.vey:6 in divide()
  at example.vey:9 in calculate()
```

The traceback shows where the functions were called.

The primary error location identifies the operation that failed, while traceback entries identify the calls that led to the failure.

This is especially useful when debugging nested function calls or module callbacks.

### Function Argument Errors

Function call errors can also identify the call site.

```veyra
add multiply(a, b)
    return a * b
end

multiply(10)
```

Example error:

```text
Runtime Error: example.vey:5:1: Missing parameter 'b' for function 'multiply'

  5 | multiply(10)
    | ^^^^^^^^
  at example.vey:5 in multiply()
```

This helps distinguish errors in function arguments from errors that occur inside the function body.

## 8. Lambda Error Locations

**Improved in Veyra 0.2.8**

Lambda expressions preserve their original source locations when reporting runtime errors.

Example:

```veyra
add divide = func(x) => 10 / x

say divide(0)
```

Example error:

```text
Runtime Error: example.vey:1:28: Division by zero

  1 | add divide = func(x) => 10 / x
    |                            ^
  at example.vey:3 in <lambda>()
```

The error points to the `/` operator inside the lambda expression.

The traceback identifies the lambda invocation.

This also works with lambdas passed as callbacks to supported functions and list methods.

## 9. Preventing Errors with Conditions

Not every invalid situation needs to become a runtime error.

You can check values before performing an operation.

```veyra
add denominator = 5

if denominator != 0
    say 10 / denominator
else
    say "The denominator must not be zero."
end
```

Use conditions for situations that can be checked in advance.

Use `try/catch` for operations that may still fail during execution.

### Short-Circuit Evaluation

**New in Veyra 0.2.8**

The `and`, `or`, and `??` operators support short-circuit evaluation.

This means the right operand is evaluated only when necessary.

For example:

```veyra
add denominator = 0

if denominator != 0 and 10 / denominator > 2
    say "Valid result"
end
```

The division is not evaluated because `denominator != 0` is false.

Short-circuit evaluation can therefore prevent unnecessary runtime errors.

### Logical AND

```veyra
say false and (10 / 0)
```

Output:

```text
false
```

The right operand is skipped when the left operand is falsy.

### Logical OR

```veyra
say true or (10 / 0)
```

Output:

```text
true
```

The right operand is skipped when the left operand is truthy.

### Null Coalescing

```veyra
say 42 ?? (10 / 0)
```

Output:

```text
42
```

The right operand is skipped when the left operand is not `null`.

### Syntax Validation

Short-circuit evaluation skips execution, but it does not ignore syntax errors.

For example:

```veyra
say true or (10 + )
```

This expression produces a syntax error.

The skipped operand must still contain valid Veyra syntax.

For more details, see [Operators](operators.md).

## 10. Avoiding Silent Failures

Avoid catching errors without handling them meaningfully.

Misleading:

```veyra
try
    throw "Operation failed"
catch err
    say "Everything worked!"
end
```

Better:

```veyra
try
    throw "Operation failed"
catch err
    say f"Operation failed: {err}"
end
```

When appropriate, you can raise another error from inside a handler.

```veyra
try
    throw "Connection failed"
catch err
    throw f"Unable to continue: {err}"
end
```

This allows the failure to propagate to another handler or terminate execution.

## 11. Common Mistakes

### Forgetting End

Every `try/catch` structure must end with `end`.

```veyra
try
    throw "Example"
catch err
    say err
end
```

### Expecting Execution to Resume Inside Try

When an error occurs, Veyra jumps to the corresponding catch block.

```veyra
try
    throw "Failed"
    say "This will not execute"
catch err
    say err
end
```

### Catching Too Much Code

Avoid wrapping unrelated operations in a single large `try` block.

Smaller blocks make failures easier to identify.

### Ignoring Error Details

During development, include the original error message when possible.

```veyra
try
    throw "Invalid configuration"
catch err
    say f"Configuration error: {err}"
end
```

### Assuming Short-Circuiting Ignores Syntax Errors

This expression is invalid:

```veyra
say true or (10 + )
```

Although the right operand is not evaluated, its syntax must still be valid.

### Combining Text and Error Values

String interpolation is the recommended way to construct messages containing runtime values.

```veyra
add code = 404

say f"Request failed with status {code}"
```

The `+` operator does not automatically convert numbers into text.

## 12. Complete Example

The following program attempts to read a file and handles failures.

```veyra
mod io

add filename = "message.txt"

try
    add message = io.read(filename)

    say "Message loaded:"
    say message
catch err
    say f"Unable to load {filename}"
    say f"Reason: {err}"
end
```

If the file exists and is readable, its contents are displayed.

Otherwise, the error is reported.

## 13. Practice Exercises

### Exercise 1: Division

Perform a division inside `try` and handle a division-by-zero error.

### Exercise 2: Custom Error

Create a function that uses `throw` when an argument is invalid.

### Exercise 3: File Reading

Read a file and display a descriptive error if the operation fails.

### Exercise 4: Nested Handlers

Create nested `try/catch` blocks and handle an error in the inner block.

### Exercise 5: Traceback

Create three functions that call one another.

Trigger an error in the deepest function and inspect the traceback.

### Exercise 6: Error Propagation

Catch an error and throw a new error containing the original message.

### Exercise 7: Interpolated Error Messages

Create a function that validates a number.

If the number is outside an allowed range, throw an interpolated error message containing the invalid value.

### Exercise 8: Column-Based Diagnostics

Write a program that attempts to access an invalid list index.

Inspect the line, column, and source marker in the resulting error.

### Exercise 9: Lambda Traceback

Create a lambda that divides a number by its argument.

Call it with zero and inspect the error location and traceback.

### Exercise 10: Short-Circuit Error Prevention

Use `and`, `or`, and `??` to safely skip expressions that would otherwise produce division-by-zero errors.

## What's Next?

Continue learning Veyra:

- [Modules](modules.md)
- [Input and Output](io.md)
- [Operators](operators.md)
- [Control Flow](control-flow.md)
- [Functions](functions.md)
