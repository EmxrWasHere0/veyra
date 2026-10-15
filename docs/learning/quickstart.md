# Veyra Quick Start

Welcome to your first Veyra lesson!

In this guide, you will write and run a simple program, work with variables, perform calculations, create a function, and receive input from the user.

You should have the Veyra interpreter installed. If you have not installed it yet, follow the [Installation Guide](installation.md).

## 1. Create Your First Program

Create a file named `hello.vey`.

Write the following code:

```veyra
say "Hello, World!"
```

The `say` command prints a value to the terminal.

Run your program from the directory containing the file:

```bash
veyra hello.vey
```

If you are running Veyra from its source repository instead, use:

```bash
cargo run -- hello.vey
```

Output:

```text
Hello, World!
```

That's your first Veyra program!

## 2. Write Comments

Comments help explain what your code does. In Veyra, `#` starts a comment outside a quoted string.

```veyra
# This is a comment.
say "Hello!"  # Print a greeting.
```

Comments are ignored when the program executes.

## 3. Work with Variables

Variables let you store values and use them later.

```veyra
name = "Veyra"
age = 15

say name
say age
```

Output:

```text
Veyra
15
```

In this example:

- `name` stores text.
- `age` stores a number.
- `say` displays the value of each variable.

Veyra is dynamically typed, so you do not need to declare a variable's type before assigning a value.

## 4. Perform Calculations

Veyra supports arithmetic operators such as `+`, `-`, `*`, `/`, and `%`.

```veyra
a = 10
b = 3

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

The `%` operator returns the remainder of a division.

You can also use `+` to combine text with other values:

```veyra
name = "Veyra"
version = 1

say "Language: " + name
say "Version: " + version
```

Output:

```text
Language: Veyra
Version: 1
```

## 5. Make Decisions

Use `if` and `else` to execute different blocks of code depending on a condition.

```veyra
age = 18

if age >= 18
    say "You are an adult."
else
    say "You are not an adult yet."
end
```

The condition is evaluated first. If it is true, the first block runs. Otherwise, the `else` block runs.

Every block in this example is closed with `end`.

## 6. Repeat Instructions

Use `repeat` to execute code multiple times.

```veyra
repeat 5
    say "Hello!"
end
```

This prints `Hello!` five times.

You can also use a condition with `repeat while`:

```veyra
counter = 1

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

The variable `counter` increases by one on each iteration. When the condition becomes false, the loop stops.

## 7. Create a Function

Functions let you group instructions and reuse them.

```veyra
add greet(name)
    say "Hello, " + name + "!"
end

greet("Alex")
greet("Veyra")
```

Output:

```text
Hello, Alex!
Hello, Veyra!
```

Here's how it works:

- `add greet(name)` defines a function named `greet`.
- `name` is a parameter.
- `end` closes the function definition.
- `greet("Alex")` calls the function with an argument.

You can also write functions that return values:

```veyra
add square(number)
    return number * number
end

result = square(5)
say result
```

Output:

```text
25
```

The `return` statement sends a value back to the caller.

## 8. Receive Input from the User

Use `get` to ask the user for input. The entered value is stored as text in `_input`.

```veyra
get "What is your name? "
name = _input

say "Hello, " + name + "!"
```

If the user enters `Alex`, the output will look like this:

```text
What is your name? Alex
Hello, Alex!
```

To convert text into a number, use the built-in `num()` function:

```veyra
get "Enter a number: "
number = num(_input)

say number * 2
```

If the user enters `5`, the program prints `10`.

If the input cannot be converted into a number, the conversion reports an error.

## 9. Work with Lists

Lists store multiple values in one variable.

```veyra
colors = ["red", "green", "blue"]

say colors[0]
say colors[1]
say len(colors)
```

Output:

```text
red
green
3
```

List indexes start at `0`, so the first item is `colors[0]`.

The built-in `len()` function returns the number of items in a list. It also works with text.

## 10. Understand Constants and Channels

The `set` command declares a constant. Assigning to a constant does not replace its original value; it writes to its associated channel instead.

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

Here, `port` remains `80`. The expression `0 ? port` reads the channel value and uses `0` as the default if no channel value has been set.

Channels are an advanced Veyra feature. You can learn more in the [Channels Guide](channels.md).

## 11. Try It Yourself

Create a file named `practice.vey` and complete these exercises.

### Exercise 1: A Personal Greeting

Ask the user for their name and print a greeting.

Expected behavior:

```text
What is your name? Sam
Hello, Sam!
```

### Exercise 2: A Number Calculator

Ask the user for a number, convert it using `num()`, and print the number multiplied by three.

### Exercise 3: Count to Ten

Use a `repeat while` loop to print the numbers from `1` through `10`.

### Exercise 4: A Reusable Function

Create a function named `double` that accepts a number and returns twice its value.

For example:

```veyra
say double(7)
```

Expected output:

```text
14
```

### Exercise 5: Find the Number of Items

Create a list of at least four items and use `len()` to print the number of items.

## What's Next?

You now know the basic structure of a Veyra program!

Continue learning with these guides:

- [Installation](installation.md)
- [Variables and Types](variables.md)
- [Input and Output](io.md)
- [Control Flow](control-flow.md)
- [Lists](lists.md)
- [Functions](functions.md)
- [Operators](operators.md)
- [Modules](modules.md)
