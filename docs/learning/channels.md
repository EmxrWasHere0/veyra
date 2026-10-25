# Channels

Channels are one of Veyra's distinctive language features.

They allow constants to preserve their original values while storing separate runtime overrides.

Channel values can also be carried through function arguments.

Channels are available in Veyra 0.2.7 and work alongside regular variables, functions, and string interpolation.

## What You Will Learn

By the end of this lesson, you will know how to:

- Declare constants using `set`.
- Assign values to channels.
- Read channels using `?`.
- Provide fallback values.
- Pass channel information through function arguments.
- Use channels for runtime configuration.
- Combine channels with string interpolation.
- Understand the differences between `?`, `?=`, and `??`.

## 1. Declaring Constants

Use `set` to declare a constant.

```veyra
set app_name = "Veyra"

say app_name
```

Output:

```text
Veyra
```

A constant preserves its original value throughout execution.

## 2. Assigning Channel Values

Assigning a new value to a constant does not replace the constant.

Instead, Veyra updates its associated channel.

```veyra
set app_name = "Veyra"

app_name = "My Application"

say app_name
```

Output:

```text
Veyra
```

The channel contains `"My Application"`, while the original constant remains `"Veyra"`.

Read the channel using `?`:

```veyra
say "default" ? app_name
```

Output:

```text
My Application
```

## 3. Reading Channels

The general syntax is:

```text
fallback ? channel_name
```

If the channel has an assigned value, Veyra returns it.

Otherwise, Veyra returns the fallback expression.

### Without an Override

```veyra
set port = 80

say 0 ? port
```

Output:

```text
0
```

The fallback is returned because no channel value has been assigned.

### With an Override

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

The constant and its channel are independent values.

## 4. Channels and Function Arguments

Channel information can travel through function arguments.

This allows a function to receive both the original constant value and its associated runtime override.

```veyra
set port = 8080

port = 3000

add show_port(target)
    say target
    say 0 ? target
end

show_port(port)
```

Output:

```text
8080
3000
```

Here's what happens:

1. `set port = 8080` creates the constant.
2. `port = 3000` assigns its channel value.
3. `show_port(port)` passes the original value and its channel tag.
4. Inside the function, `target` reads the original value.
5. `0 ? target` retrieves the carried channel value.

This behavior makes it possible to preserve the original value while allowing explicit access to its override.

### Passing Ordinary Values

A function can also receive values that do not carry a channel override.

```veyra
add show(value)
    say value
    say "fallback" ? value
end

show(42)
```

Output:

```text
42
fallback
```

Without a channel override, the `?` expression returns its fallback.

## 5. Channels for Configuration

Channels can separate default configuration values from runtime overrides.

```veyra
set server_host = "localhost"
set server_port = 80

server_host = "127.0.0.1"
server_port = 8080

add host = "localhost" ? server_host
add port = 80 ? server_port

say f"Host: {host}"
say f"Port: {port}"
```

Output:

```text
Host: 127.0.0.1
Port: 8080
```

The original constants remain unchanged.

This can be useful for runtime configuration and explicit fallback behavior.

### Reading Configuration Inside Functions

Channels can also be passed into functions.

```veyra
set default_port = 80

default_port = 8080

add show_configuration(port)
    say f"Default port: {port}"
    say f"Active port: {0 ? port}"
end

show_configuration(default_port)
```

Output:

```text
Default port: 80
Active port: 8080
```

The original constant value and its associated channel override remain separate.

## 6. Channels vs. Mutable Variables

Mutable variables are declared with `add`.

```veyra
add name = "Alex"

name = "Sam"

say name
```

Output:

```text
Sam
```

Assignment replaces the variable's value.

Constants declared with `set` behave differently.

```veyra
set name = "Alex"

name = "Sam"

say name
say "" ? name
```

Output:

```text
Alex
Sam
```

The original constant is preserved, and the channel stores the override.

## 7. Channel Operator vs. Conditional Assignment

The `?`, `?=`, and `??` operators serve different purposes.

| Operator | Description                    |
| -------- | ------------------------------ |
| `?`      | Read a channel with a fallback |
| `?=`     | Conditionally assign a value   |
| `??`     | Provide a fallback for `null`  |

### Conditional Assignment

Use `?=` when assigning a default to an undefined name.

```veyra
username ?= "guest"

say username
```

Output:

```text
guest
```

If a value already exists, it is preserved.

```veyra
add username = "alex"

username ?= "guest"

say username
```

Output:

```text
alex
```

### Null Coalescing

The `??` operator checks whether a value is `null`.

```veyra
add username = null

say username ?? "guest"
```

Output:

```text
guest
```

Unlike `?`, null coalescing does not access channels.

In Veyra 0.2.7, both operands of `??` are evaluated. The operator does not currently provide short-circuit evaluation.

## 8. Channels and String Interpolation

Veyra 0.2.7 supports string interpolation.

You can use interpolated strings to display channel-related values.

```veyra
set app_version = "0.2.7"

app_version = "development"

add original = app_version
add current = "" ? app_version

say f"Original version: {original}"
say f"Channel value: {current}"
```

Output:

```text
Original version: 0.2.7
Channel value: development
```

Interpolation is useful when formatting configuration values or debugging channel behavior.

You can also place expressions inside interpolated strings:

```veyra
set port = 80

port = 8080

say f"Default: {port}"
say f"Active: {0 ? port}"
```

Output:

```text
Default: 80
Active: 8080
```

## 9. Common Mistakes

### Expecting a Constant to Change

```veyra
set mode = "safe"

mode = "fast"

say mode
```

Output:

```text
safe
```

Use `?` to read the override.

```veyra
say "" ? mode
```

Output:

```text
fast
```

### Forgetting the Fallback

Channel reads require a fallback expression.

Correct:

```veyra
say 0 ? port
```

### Confusing Channels with Null Values

A missing channel override and a `null` value are different situations.

Use `?` for channels and `??` for null coalescing.

### Assuming Every Argument Has a Channel

Ordinary values do not necessarily carry channel information.

When no override is available, the fallback is returned.

### Using Channels Like Mutable Variables

A `set` constant cannot be updated in the same way as an `add` variable.

```veyra
set value = 10

value = 20

say value
say 0 ? value
```

Output:

```text
10
20
```

Use `add` when you want regular mutable assignment behavior.

## 10. Practice Exercises

### Exercise 1: Application Name

Declare a constant named `app_name`.

Assign an override and print both its original value and its channel value.

### Exercise 2: Default Port

Declare `port` with a constant value of `80`.

Read its channel before and after assigning an override.

### Exercise 3: Function Argument

Create a function that accepts a parameter and displays both its regular value and its channel value.

Pass a constant with an assigned channel to the function.

### Exercise 4: Configuration

Create constants for a server host and port.

Assign runtime overrides and read them using channel expressions.

Use string interpolation to display the configuration.

### Exercise 5: Operator Differences

Write examples demonstrating:

- `?`
- `?=`
- `??`

Explain the difference between their results.

### Exercise 6: Interpolated Channel Values

Create a constant named `language`.

Assign a channel override.

Use interpolated strings to display the original value and the channel value.

## What's Next?

Continue learning Veyra:

- [Variables and Types](variables.md)
- [Operators](operators.md)
- [Functions](functions.md)
- [Modules](modules.md)
- [Error Handling](error-handling.md)
