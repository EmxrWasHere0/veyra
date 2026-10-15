# Built-in Functions

Built-in functions are functions provided by Veyra that you can use without defining them yourself. They help you perform common tasks such as working with data, handling input and output, and interacting with the system.

Veyra also provides built-in modules that group related functionality together.

## Using Built-in Functions

You can call a built-in function in the same way you call a regular function: write its name followed by parentheses and pass any required arguments.

For example, `len()` returns the number of elements in a list.

```text
items = ["apple", "banana", "orange"]

say len(items)  # 3
```

Here, `len(items)` returns the number of elements in `items`, and `say` prints the result.

Built-in functions save you from having to implement common operations yourself.

## The `len()` Function

The `len()` function returns the length of a supported value, such as a list.

```text
numbers = [10, 20, 30, 40]

say len(numbers)  # 4
```

This is especially useful when iterating over lists.

```text
numbers = [10, 20, 30]
i = 0

repeat while i < len(numbers)
    say numbers[i]
    i += 1
end
```

Using `len()` means the loop can adapt to the list's length instead of relying on a hard-coded number.

## Built-in Modules

Veyra organizes additional functionality into modules. The available standard modules include `base`, `io`, and `system`.

These modules group related operations so that programs can use the functionality they need.

### The `base` Module

The `base` module provides general-purpose functionality for common programming tasks.

Use it when you need functionality exposed by Veyra's standard base library.

### The `io` Module

The `io` module provides input and output functionality.

I/O operations are useful when a program needs to interact with external data or perform supported input/output tasks.

### The `system` Module

The `system` module provides system-related functionality.

Depending on the functionality exposed by the current Veyra runtime, this module can be used for supported interactions with the environment in which a program runs.

**Note:** Consult the module's implementation or API reference for the exact names, parameters, and return values of its available operations.

## Built-in Functions vs. User-defined Functions

Built-in functions are supplied by Veyra, while user-defined functions are written by the programmer.

For example, the following code defines a custom function:

```text
add double(x)
    return x * 2
end

say double(5)  # 10
```

In this example, `double()` is a user-defined function. By contrast, `len()` is provided by Veyra.

Both types of functions can be called from your program, provided they are available in the current execution context.

## Best Practices

- **Use existing functionality:** Prefer built-in operations when they already provide what your program needs.
- **Check expected arguments:** Pass values of the types supported by each function.
- **Read module documentation:** Learn which operations a module provides before relying on it.
- **Handle errors:** Built-in operations may fail when given invalid input or when an external operation cannot be completed.
- **Keep code readable:** Use descriptive names for your own functions to distinguish them from standard functionality.

## Summary

| Component          | Purpose                                                         |
| ------------------ | --------------------------------------------------------------- |
| Built-in functions | Provide functionality without requiring your own implementation |
| `len()`            | Returns the length of a supported value, such as a list         |
| `base`             | Groups general-purpose functionality                            |
| `io`               | Groups input/output functionality                               |
| `system`           | Groups system-related functionality                             |

Built-in functions and modules make Veyra programs easier to write by providing reusable functionality for common tasks. Refer to the API documentation for the complete list of available operations and their exact behavior.
