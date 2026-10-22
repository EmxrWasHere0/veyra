# Error Handling

Errors can occur while a Veyra program is running. A calculation may fail, a file may not exist, or a function may receive an invalid value.

Veyra provides `try`, `catch`, and `throw` for handling runtime errors.

Since version 0.2.6, runtime errors also include source locations and function tracebacks to make debugging easier.

## What You Will Learn

By the end of this guide, you will know how to:

- Understand runtime errors.
- Handle errors using `try` and `catch`.
- Raise errors explicitly using `throw`.
- Use optional catch variables.
- Handle errors in nested blocks.
- Read source locations and function tracebacks.
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
    say "An error occurred: " + err
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
    say "Calculation failed: " + err
end
```

### Throwing Expressions

`throw` accepts an expression.

```veyra
add name = "example.txt"

throw "Unable to open file: " + name
```

This is useful for producing descriptive error messages.

## 4. Handling File Errors

File operations may fail because a path does not exist or permissions are insufficient.

Import the `io` module before using its functions.

```veyra
mod io

try
    add content = io.read("notes.txt")
    say content
catch err
    say "Could not read the file: " + err
end
```

### Writing Files

```veyra
mod io

try
    io.write("output.txt", "Hello from Veyra!")
    say "File written successfully."
catch err
    say "Could not write the file: " + err
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
        say "Handled inner error: " + inner_error
    end

    say "Continuing operation"
catch outer_error
    say "Outer error: " + outer_error
end
```

If the inner handler successfully handles the error, execution can continue in the outer block.

Nested handlers are useful when different operations require different recovery strategies.

## 6. Runtime Error Locations

Veyra 0.2.6 includes source information in runtime errors.

Consider a file named `example.vey`:

```veyra
add x = 10
add y = 0

say x / y
```

Running the file produces an error similar to:

```text
Runtime Error: example.vey:4: Division by zero

  4 | say x / y
    | ^^^^^^^^^
```

The error report includes:

- The source filename.
- The line number.
- The error description.
- The source line.
- A marker identifying the relevant line.

The marker currently highlights the line rather than the exact expression.

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
Runtime Error: example.vey:2: Division by zero

  2 |     return a / b
    | ^^^^^^^^^^^^^^^^

  at example.vey:6 in divide()
  at example.vey:9 in calculate()
```

The traceback shows where the functions were called.

This is especially useful when debugging nested function calls or module callbacks.

## 8. Preventing Errors with Conditions

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

## 9. Avoiding Silent Failures

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
    say "Operation failed: " + err
end
```

When appropriate, you can raise another error from inside a handler.

```veyra
try
    throw "Connection failed"
catch err
    throw "Unable to continue: " + err
end
```

This allows the failure to propagate to another handler or terminate execution.

## 10. Common Mistakes

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
    say "Configuration error: " + err
end
```

## 11. Complete Example

The following program attempts to read a file and handles failures.

```veyra
mod io

add filename = "message.txt"

try
    add message = io.read(filename)

    say "Message loaded:"
    say message
catch err
    say "Unable to load " + filename
    say "Reason: " + err
end
```

If the file exists and is readable, its contents are displayed.

Otherwise, the error is reported.

## 12. Practice Exercises

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

## What's Next?

Continue learning Veyra:

- [Modules](modules.md)
- [Input and Output](io.md)
- [Operators](operators.md)
- [Control Flow](control-flow.md)
- [Functions](functions.md)
