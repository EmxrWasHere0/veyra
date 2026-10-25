# Veyra Quick Start

Welcome to Veyra!

In this guide, you will write your first program, work with variables, use functions and loops, read user input, and explore collections.

You will also learn about string interpolation, functional list methods, JSONData objects, and Veyra's channel system.

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

Text values can be combined using `+`.

```veyra
add name = "Veyra"
add version = "0.2.7"

say "Language: " + name
say "Version: " + version
```

Output:

```text
Language: Veyra
Version: 0.2.7
```

To combine text with numbers or other values, use string interpolation or explicit conversion.

## 5. String Interpolation

Veyra 0.2.7 introduces string interpolation.

Prefix a string with `f` to embed expressions using `{expression}`.

```veyra
add name = "Alex"
add age = 20

say f"Hello, {name}!"
say f"Next year, you will be {age + 1}."
```

Output:

```text
Hello, Alex!
Next year, you will be 21.
```

Interpolation supports expressions, arithmetic, function calls, and other Veyra values.

```veyra
add a = 10
add b = 5

say f"Sum: {a + b}"
say f"Product: {a * b}"
```

Output:

```text
Sum: 15
Product: 50
```

### Supported String Formats

Interpolation works with single-quoted, double-quoted, and multiline strings.

```veyra
add name = "Alex"

say f"Hello, {name}!"
say f'Welcome, {name}!'

say f"""
User Information:
Name: {name}
Language: Veyra
"""
```

### Literal Braces

Use `{{` and `}}` to include literal braces.

```veyra
add value = 42

say f"Value: {{ {value} }}"
```

Output:

```text
Value: { 42 }
```

String interpolation is especially useful for messages, output formatting, and debugging.

## 6. Conditions

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

## 7. Loops

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
    counter = counter + 1
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

## 8. Functions

Functions allow you to reuse code.

```veyra
add greet(name)
    say f"Hello, {name}!"
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

### Lambda Expressions

Lambda expressions are anonymous functions.

```veyra
add double = func(x) => x * 2

say double(5)
```

Output:

```text
10
```

Lambda expressions are particularly useful with functional list methods.

For more information, see [Functions](functions.md).

## 9. User Input

Use `get` to read user input.

```veyra
get name "What is your name? "

say f"Hello, {name}!"
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

say f"Double: {number * 2}"
```

If the user enters `5`, the program displays:

```text
Double: 10
```

For more information, see [Input and Output](io.md).

## 10. Lists

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

Output:

```text
[10, 20, 30, 40, 50]
```

Lists also provide methods such as `pop()`, `insert()`, `remove()`, `clear()`, `sort()`, and `reverse()`.

For more information, see [Lists](lists.md).

## 11. Functional List Methods

Veyra 0.2.7 introduces functional list methods.

These methods allow you to transform, filter, and process lists using callback functions.

### map()

`map(callback)` applies a function to each element and returns a new list.

```veyra
add numbers = [1, 2, 3, 4, 5]

add doubled = numbers.map(func(x) => x * 2)

say doubled
```

Output:

```text
[2, 4, 6, 8, 10]
```

### filter()

`filter(callback)` returns a new list containing only elements that satisfy a condition.

```veyra
add numbers = [1, 2, 3, 4, 5]

add even = numbers.filter(func(x) => x % 2 == 0)

say even
```

Output:

```text
[2, 4]
```

### reduce()

`reduce(callback, initial)` combines list elements into a single value.

The callback receives the accumulator and the current element.

```veyra
add numbers = [1, 2, 3, 4, 5]

add total = numbers.reduce(func(acc, x) => acc + x, 0)

say total
```

Output:

```text
15
```

The initial value is required.

### any() and all()

`any(callback)` returns `true` if at least one element satisfies the condition.

`all(callback)` returns `true` if every element satisfies the condition.

```veyra
add numbers = [2, 4, 6, 8]

say numbers.any(func(x) => x > 5)
say numbers.all(func(x) => x % 2 == 0)
```

Output:

```text
true
true
```

### Using Named Functions

Functional list methods also accept named functions.

```veyra
add square(x)
    return x * x
end

add numbers = [1, 2, 3, 4]

say numbers.map(square)
```

Output:

```text
[1, 4, 9, 16]
```

These functional methods return new values without modifying the original list.

For more information, see [Lists](lists.md).

## 12. JSONData Objects

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

### Merging JSONData Objects

Veyra 0.2.7 introduces `merge()` and the `+` operator for JSONData objects.

```veyra
add user = {
    "name": "Alex",
    "age": 20
}

add updates = {
    "age": 21,
    "active": true
}

add result = user.merge(updates)

say result
```

Output:

```text
{"name": "Alex", "age": 21, "active": true}
```

You can also use `+`:

```veyra
add combined = user + updates

say combined
```

Output:

```text
{"name": "Alex", "age": 21, "active": true}
```

Both operations perform a shallow merge.

When keys overlap, values from the right-hand object take priority.

The original objects remain unchanged.

### Getting JSONData Items

Use `items()` to get key-value pairs as a list.

```veyra
add user = {
    "name": "Alex",
    "age": 20
}

say user.items()
```

Output:

```text
[["name", "Alex"], ["age", 20]]
```

JSONData also supports methods such as `keys()`, `values()`, `get()`, `has()`, `remove()`, and `len()`.

## 13. Constants and Channels

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

## 14. Interactive REPL

Veyra includes an interactive REPL, introduced in version 0.2.6.

Start it without specifying a file:

```bash
veyra
```

Example:

```text
>>> add x = 10
>>> say x * 2
20
>>> say f"Double: {x * 2}"
Double: 20
>>> add double(n)
...     return n * 2
... end
>>> say double(5)
10
```

Variables, functions, and imported modules remain available throughout the session.

You can also experiment with the functional list methods introduced in Veyra 0.2.7.

```text
>>> add numbers = [1, 2, 3, 4]
>>> say numbers.map(func(x) => x * 2)
[2, 4, 6, 8]
```

Type `exit` or `quit` to leave the REPL.

## 15. Practice Exercises

### Exercise 1: Greeting

Ask the user for their name and print a personalized greeting using string interpolation.

### Exercise 2: Calculator

Read two numbers and print their sum.

Use interpolation to format the result.

### Exercise 3: Counting

Use `while` to print numbers from 1 to 10.

### Exercise 4: Functions

Create a function named `double` that returns twice its argument.

### Exercise 5: Lists

Create a list of five numbers and print the first and last elements.

### Exercise 6: Collection Iteration

Use `repeat item in collection` to calculate the sum of a list.

### Exercise 7: Functional Lists

Create a list of numbers from 1 to 10.

Use:

- `map()` to double every number.
- `filter()` to select even numbers.
- `reduce()` to calculate their sum.
- `any()` to check whether a number is greater than 8.
- `all()` to check whether every number is positive.

### Exercise 8: JSONData

Create a `jsondata` object containing a name and age.

Print both values and update the age.

### Exercise 9: JSONData Merging

Create two JSONData objects.

Merge them using `merge()` and `+`.

Use `items()` to display the key-value pairs.

### Exercise 10: Channels

Declare a constant, assign a channel override, and display both values.

### Exercise 11: String Interpolation

Create variables for a person's name, age, and favorite programming language.

Display a formatted message using an interpolated string.

Include at least one calculated expression.

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
