# Error Handling

Error handling allows your program to respond to errors instead of stopping unexpectedly. It helps you detect problems, handle exceptional situations, and make your programs more reliable.

Veyra provides `try` and `catch` blocks for handling errors that occur during execution.

## The `try` and `catch` Blocks

Use `try` to mark a block of code that may produce an error, and `catch` to define what happens if an error is caught.

```text
try
    say "Before the error"
    missing_function()
catch
    say "An error occurred"
end
```

If the undefined function call produces a catchable runtime error, execution transfers to the `catch` block.

The program can then handle the error instead of allowing that error to propagate normally.

The structure is:

- `try` begins the block in which an error may occur.
- `catch` begins the error-handling block.
- `end` marks the end of the entire structure.

**Note:** Only errors that Veyra's error-handling mechanism can catch will be handled by `catch`.

## Handling Errors in Functions

Error handling is particularly useful when calling functions that may fail.

```text
add divide(a, b)
    if b == 0
        throw "Division by zero"
    end

    return a / b
end

try
    result = divide(10, 0)
    say result
catch
    say "Unable to perform the division"
end
```

In this example, `divide` checks whether the divisor is zero. If it is, the function raises an error rather than continuing with the division.

The surrounding `try/catch` handles the error and prints a message.

This example assumes that `throw` is supported by your Veyra implementation. If your current version uses a different mechanism to raise errors, replace that statement with the supported syntax.

## Handling Errors Without Hiding Them

A `catch` block should help the program recover or explain what went wrong. Avoid silently ignoring errors, because doing so can make problems difficult to diagnose.

For example:

```text
try
    result = calculate_total()
    say result
catch
    say "Could not calculate the total"
end
```

This provides a clear message when the calculation fails.

For larger applications, you may want to include additional context about the operation that failed, provided your error-handling features expose that information.

## Error Handling and Program Flow

When an error is caught, execution moves from the failing operation to the appropriate `catch` block. Statements after the failing operation in the `try` block are skipped when the error interrupts normal execution.

```text
try
    say "Step 1"
    missing_function()
    say "Step 2"
catch
    say "An error was caught"
end

say "Program continues"
```

If the function call raises a catchable error, the output is:

```text
Step 1
An error was caught
Program continues
```

`"Step 2"` is not printed because execution transfers to `catch`. After the error-handling block finishes, the program continues with the statement following `end`.

## Best Practices

When writing error-resistant Veyra programs, keep these principles in mind:

- **Handle expected failures:** Use error handling around operations that can reasonably fail.
- **Keep error messages useful:** Explain what operation failed instead of printing a vague message.
- **Avoid overly broad handling:** Do not catch errors only to ignore them.
- **Validate inputs:** Check values before using them when possible.
- **Keep `try` blocks focused:** Handle errors close to the operations that may produce them.

## Summary

| Keyword | Purpose                               |
| ------- | ------------------------------------- |
| `try`   | Starts a block where errors may occur |
| `catch` | Defines how caught errors are handled |
| `end`   | Closes the error-handling structure   |

Error handling makes programs more robust by allowing them to respond to failures in a controlled way. Combined with functions and conditional statements, it helps you build Veyra programs that are easier to debug and maintain.
