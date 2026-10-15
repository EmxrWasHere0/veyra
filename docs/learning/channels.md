# Channels

Channels are a feature of Veyra that allow values assigned to constants to be overridden through a separate channel.

They let you preserve a constant's original value while providing an alternative value for later use.

## What You Will Learn

By the end of this guide, you will understand:

- How to declare constants with `set`.
- How channel assignments work.
- How to read channel values with `?`.
- How to provide default values for channels.
- How channels differ from ordinary variables and conditional assignment.

## 1. Declaring a Constant

Use `set` to declare a constant.

```veyra
set app_name = "Veyra"

say app_name
```

Output:

```text
Veyra
```

A constant retains its original value when you assign a new value to its name.

## 2. Assigning to a Constant's Channel

When you assign a value to a constant that has already been declared, Veyra creates or updates a separate channel value instead of replacing the original constant.

```veyra
set app_name = "Veyra"

app_name = "My Application"

say app_name
```

Output:

```text
Veyra
```

The original constant remains unchanged.

The new value is available through the constant's channel.

```veyra
set app_name = "Veyra"

app_name = "My Application"

say app_name
say "Veyra channel: " + ("default" ? app_name)
```

Output:

```text
Veyra
Veyra channel: My Application
```

This allows you to preserve a default value while providing an override.

## 3. Reading a Channel with `?`

The `?` operator reads a channel value using a default value.

Its general form is:

```veyra
default_value ? channel_name
```

- If the channel has a value, the expression returns that value.
- If the channel has no value, the expression returns the default value.

Example:

```veyra
set port = 80

say 0 ? port
```

Output:

```text
0
```

Because the channel has not been assigned a value, the default value `0` is returned.

Now assign a channel value:

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

The first expression reads the original constant. The second expression reads the channel, falling back to `0` if no channel value exists.

## 4. Using Channels for Configuration

Channels can be useful when a program needs default configuration values that can be overridden separately.

```veyra
set server_host = "localhost"
set server_port = 80

server_host = "127.0.0.1"
server_port = 8080

host = "localhost" ? server_host
port = 80 ? server_port

say "Host: " + host
say "Port: " + port
```

Output:

```text
Host: 127.0.0.1
Port: 8080
```

The constants retain their original defaults, while the program reads the overridden values through their channels.

This pattern can help keep default settings separate from runtime overrides.

## 5. Providing a Default Value

A channel might not have an override. That's why the default value is important.

```veyra
set theme = "light"

say "light" ? theme
```

Output:

```text
light
```

If the channel has not been assigned, the expression returns `"light"`.

If an override is added, the channel value takes precedence:

```veyra
set theme = "light"

theme = "dark"

say "light" ? theme
```

Output:

```text
dark
```

Choose a default that makes sense for the value you expect the channel to provide.

## 6. Channels vs. Ordinary Variables

Ordinary variables and constants behave differently.

### Ordinary Variable

```veyra
name = "Alex"
name = "Sam"

say name
```

Output:

```text
Sam
```

An ordinary variable is updated directly.

### Constant with a Channel

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

The constant keeps its original value, while the channel stores the override.

## 7. Channels vs. `?=`

The `?` and `?=` operators have different purposes.

| Feature | Purpose                                                                   |
| ------- | ------------------------------------------------------------------------- |
| `?`     | Reads a channel value, using a default if no override exists              |
| `?=`    | Assigns a value only if the target has not already been filled or defined |

For example, `?=` is useful when initializing an ordinary variable:

```veyra
username ?= "guest"

say username
```

By contrast, `?` reads a channel:

```veyra
set username = "guest"

username = "alex"

say "guest" ? username
```

Output:

```text
alex
```

Use the operator that matches the behavior you need.

## 8. Common Mistakes

### Expecting a Channel Assignment to Change the Constant

```veyra
set mode = "default"

mode = "custom"

say mode
```

The output remains `default`. To read the override, use the channel operator:

```veyra
say "default" ? mode
```

### Forgetting the Default Value

The channel expression starts with a default value:

```veyra
"default" ? mode
```

The left side is the fallback; the right side identifies the channel.

### Confusing Channels with Null Coalescing

The `??` operator provides a fallback when the left operand is `null`.

The `?` operator reads a channel and uses its left operand as the default when no channel override exists.

These are separate features.

## 9. Practice Exercises

### Exercise 1: Application Name

Declare a constant called `app_name` with the value `"Veyra"`. Assign a different value to its channel and print both values.

### Exercise 2: Default Port

Declare a constant called `port` with the value `80`. Read its channel using `80` as the default, then assign `8080` and read it again.

### Exercise 3: Theme Configuration

Declare a `theme` constant with the value `"light"`. Add an override of `"dark"` and read the channel with `"light"` as the default.

### Exercise 4: Explain the Difference

Write a short explanation of the differences between:

- `set value = 10`
- `value = 20`
- `0 ? value`
- `value ?= 30`

## What's Next?

You now understand the basics of Veyra's channel system.

Continue learning:

- [Variables and Types](variables.md)
- [Operators](operators.md)
- [Functions](functions.md)
- [Modules](modules.md)
- [Error Handling](error-handling.md)
