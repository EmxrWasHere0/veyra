# Error Handling

Errors can occur while a Veyra program is running. A file may not exist, an operation may fail, or an expression may produce a runtime error.

Veyra provides `try` and `catch` to handle runtime errors and keep error-handling logic separate from normal program execution.

## What You Will Learn

By the end of this guide, you will understand:

- What runtime errors are.
- How to use `try` and `catch`.
- How to read error messages.
- How to keep error-handling code organized.
- How to debug common Veyra problems.
- How to avoid hiding errors accidentally.

## 1. Understanding Errors

An error is a problem that prevents an operation from completing normally.

Common causes include:

- Dividing by zero.
- Reading a file that does not exist.
- Using an invalid value in an operation.
- Calling a function with incorrect arguments.
- Accessing a resource that is unavailable.

For example:

```veyra
result = 10 / 0
say result
```

This operation produces a runtime error. The program cannot complete the calculation normally.

When an error occurs, Veyra reports it with an error message. Understanding that message is the first step toward fixing the problem.

## 2. Using `try` and `catch`

Use `try` to mark a block of code where a runtime error may occur. Use `catch` to handle an error raised while executing that block.

General structure:

```veyra
try
    # Code that may fail
catch err
    # Handle the error
end
```

The variable following `catch` receives the runtime error message.

Example:

```veyra
try
    result = 10 / 0
    say result
catch err
    say "An error occurred: " + err
end
```

If the calculation fails, Veyra runs the `catch` block and makes the error message available through `err`.

This allows the program to report the problem rather than leaving it unhandled.

## 3. Handling File Errors

File operations can fail when a file is missing or the current process does not have permission to access it.

Import the `io` module before using its file functions.

```veyra
mod io

try
    content = io.read("notes.txt")
    say content
catch err
    say "Could not read the file: " + err
end
```

If the operation succeeds, the program displays the file's contents. If it fails, the error is handled by the `catch` block.

### Writing Files

File writes can also fail.

```veyra
mod io

try
    io.write("output.txt", "Hello from Veyra!")
    say "File written successfully."
catch err
    say "Could not write the file: " + err
end
```

Only display a success message after the operation completes without raising an error.

## 4. Keeping Normal Logic Separate

A useful practice is to keep the code that may fail inside `try` and put the response to failure inside `catch`.

```veyra
mod io

try
    content = io.read("config.txt")
    say "Configuration loaded."
    say content
catch err
    say "Configuration could not be loaded."
    say "Details: " + err
end
```

This makes it easier to distinguish normal program behavior from error handling.

Avoid wrapping large sections of unrelated code in one `try` block if doing so would make it difficult to identify which operation failed.

## 5. Understanding Error Messages

An error message provides clues about what went wrong.

When investigating an error, ask yourself:

1. Which operation was running?
2. Which values were being used?
3. Was a file or other resource available?
4. Does the error point to an invalid operation or argument?
5. Can the problem be reproduced with a smaller example?

For example, if a file read fails, verify the path and check whether the file is accessible.

```veyra
mod io

if io.exists("notes.txt")
    content = io.read("notes.txt")
    say content
else
    say "notes.txt was not found."
end
```

Checking whether a path exists can help, but it does not guarantee that a later operation will succeed. Permissions or other conditions may still cause an error, so `try/catch` can remain useful.

## 6. Errors and Conditional Logic

Not every unexpected situation is a runtime error. Some situations can be handled using conditions.

For example, you can check a value before performing an operation.

```veyra
denominator = 5

if denominator != 0
    result = 10 / denominator
    say result
else
    say "The denominator must not be zero."
end
```

This checks the denominator before dividing.

Use conditional logic when the program can detect and respond to a situation in advance. Use `try/catch` when an operation may still fail at runtime.

## 7. Avoiding Silent Failures

A `catch` block should do something useful with an error.

Avoid catching an error and then continuing as if everything succeeded.

For example, this is misleading:

```veyra
try
    result = 10 / 0
catch err
    say "Everything worked!"
end
```

The message does not describe what happened.

A better approach is to report the failure honestly:

```veyra
try
    result = 10 / 0
catch err
    say "The calculation failed: " + err
end
```

If a program cannot continue safely after an operation fails, do not silently continue with an invalid or incomplete result.

## 8. Common Mistakes

### Forgetting `end`

Veyra uses `end` to close the `try/catch` structure.

```veyra
try
    result = 10 / 0
catch err
    say err
end
```

### Using an Error Variable Outside Its Intended Purpose

The variable after `catch` receives the error message. Use it inside the handler to explain the failure.

```veyra
try
    result = 10 / 0
catch err
    say "Error: " + err
end
```

### Assuming Every Failure Can Be Recovered From

Catching an error does not automatically make the failed operation successful. Check whether the program has enough information to continue safely.

### Ignoring the Original Error

A generic message can be useful for users, but during development the original error message often provides important diagnostic information.

```veyra
try
    content = io.read("config.txt")
catch err
    say "Unable to load configuration."
    say "Debug details: " + err
end
```

## 9. A Complete Example

The following program tries to load a file and handles a failure.

```veyra
mod io

filename = "message.txt"

try
    message = io.read(filename)
    say "Message loaded:"
    say message
catch err
    say "Unable to load " + filename
    say "Reason: " + err
end
```

If `message.txt` is available and readable, its contents are displayed. Otherwise, the program reports the error.

## 10. Practice Exercises

### Exercise 1: Division

Write a program that performs a division inside `try` and reports any runtime error in `catch`.

### Exercise 2: File Reading

Try to read a file and display its contents. If reading fails, display a useful error message.

### Exercise 3: File Writing

Write text to a file and display a success message only if the operation completes successfully.

### Exercise 4: Validate Before Calculating

Use an `if` statement to check whether a denominator is zero before performing division.

### Exercise 5: Improve an Error Message

Take a generic error handler and improve it so the user knows which operation failed while the original error remains available for debugging.

## What's Next?

You now know how to handle common runtime errors in Veyra.

Continue learning:

- [Modules](modules.md)
- [Input and Output](io.md)
- [Operators](operators.md)
- [Control Flow](control-flow.md)
- [Functions](functions.md)
