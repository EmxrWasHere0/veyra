# Veyra Quick Start

Welcome to Veyra!

In this guide, you will write your first program, work with variables, use functions and loops, read user input, and explore collections.

Veyra is a dynamically typed, interpreted programming language written in Rust.

If you have not installed the interpreter yet, follow the [Installation Guide](installation.md).

## 1. Your First Program

Create a file named `hello.vey`:

```veyra
say "Hello, World!"
```

Run it:

```bash
veyra hello.vey
```

Output:

```text
Hello, World!
```

You can also run programs from the Veyra source directory:

```bash
cargo run -- hello.vey
```

## 2. Comments

Comments begin with `#`.

```veyra
# This is a comment.
say "Hello!"  # Print a greeting.
```

Comments are ignored during execution.

## 3. Variables

Use `add` to declare mutable variables.

```veyra
add name = "Veyra"
add age = 15

say name
say age
```

Output:

```text
Veyra
15
```

Variables can be updated using `=`.

```veyra
add score = 10

score = 25

say score
```

Output:

```text
25
```

Veyra is dynamically typed, so variables do not require type annotations.

## 4. Calculations

Veyra supports common arithmetic operators.

```veyra
add a = 10
add b = 3

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

Text can also be combined using `+`.

```veyra
add name = "Veyra"
add version = "0.2.6"

say "Language: " + name
say "Version: " + version
```

Output:

```text
Language: Veyra
Version: 0.2.6
```

## 5. Conditions

Use `if` and `else` to make decisions.

```veyra
add age = 18

if age >= 18
    say "You are an adult."
else
    say "You are not an adult yet."
end
```

Blocks are closed using `end`.

## 6. Loops

Use `repeat` to execute a block a specific number of times.

```veyra
repeat 3
    say "Hello!"
end
```

### While Loops

Use `while` to repeat while a condition is true.

```veyra
add counter = 1

while counter <= 3
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

### Collection Loops

Use `repeat item in collection` to iterate directly over a collection.

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

For more information, see [Control Flow](control-flow.md).

## 7. Functions

Functions allow you to reuse code.

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

Functions can return values using `return`.

```veyra
add square(number)
    return number * number
end

add result = square(5)

say result
```

Output:

```text
25
```

Veyra also supports default parameters, recursion, variadic parameters, and lambda expressions.

## 8. User Input

Use `get` to read user input.

```veyra
get name "What is your name? "

say "Hello, " + name + "!"
```

Example interaction:

```text
What is your name? Alex
Hello, Alex!
```

Input is read as text.

Use `num()` when you need a number.

```veyra
get value "Enter a number: "

add number = num(value)

say number * 2
```

If the user enters `5`, the program displays `10`.

## 9. Lists

Lists store multiple values.

```veyra
add colors = ["red", "green", "blue"]

say colors[0]
say colors[-1]
say len(colors)
```

Output:

```text
red
blue
3
```

Lists also support slicing.

```veyra
add numbers = [10, 20, 30, 40]

say numbers[1:3]
```

Output:

```text
[20, 30]
```

You can add elements using `append()`.

```veyra
numbers.append(50)

say numbers
```

For more information, see [Lists](lists.md).

## 10. JSONData Objects

Veyra provides a `jsondata` type for key-value collections.

```veyra
add user = {
    "name": "Alex",
    "age": 20,
    "active": true
}

say user["name"]
say user["age"]
```

Output:

```text
Alex
20
```

Object values can be updated.

```veyra
user["age"] = 21

say user["age"]
```

Output:

```text
21
```

## 11. Constants and Channels

Use `set` to declare a constant.

Assigning to a constant updates its channel rather than changing the original value.

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

The `?` operator reads a channel value using a fallback.

For details, see [Channels](channels.md).

## 12. Interactive REPL

Veyra 0.2.6 includes an interactive REPL.

Start it without specifying a file:

```bash
veyra
```

Example:

```text
>>> add x = 10
>>> say x * 2
20
>>> add double(n)
...     return n * 2
... end
>>> say double(5)
10
```

Variables and functions remain available throughout the session.

Type `exit` or `quit` to leave the REPL.

## 13. Practice Exercises

### Exercise 1: Greeting

Ask the user for their name and print a personalized greeting.

### Exercise 2: Calculator

Read two numbers and print their sum.

### Exercise 3: Counting

Use `while` to print numbers from 1 to 10.

### Exercise 4: Functions

Create a function named `double` that returns twice its argument.

### Exercise 5: Lists

Create a list of five numbers and print the first and last elements.

### Exercise 6: Collection Iteration

Use `repeat item in collection` to calculate the sum of a list.

### Exercise 7: JSONData

Create a `jsondata` object containing a name and age.

Print both values and update the age.

### Exercise 8: Channels

Declare a constant, assign a channel override, and display both values.

## What's Next?

Continue learning Veyra:

- [Variables and Types](variables.md)
- [Input and Output](io.md)
- [Control Flow](control-flow.md)
- [Lists](lists.md)
- [Functions](functions.md)
- [Operators](operators.md)
- [Modules](modules.md)
- [Channels](channels.md)
