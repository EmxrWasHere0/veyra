# Functions

Functions allow you to organize code into reusable blocks.

Veyra supports named functions, recursion, nested definitions, default parameters, variadic parameters, lambdas, and first-class function values.

## What You Will Learn

By the end of this lesson, you will know how to:

- Define and call functions.
- Pass arguments and return values.
- Use explicit and implicit returns.
- Define default and variadic parameters.
- Return multiple values.
- Create lambda expressions.
- Pass functions as arguments.
- Write recursive functions.
- Understand closed function scopes.
- Read function tracebacks.

## 1. Defining Functions

Use `add` to declare a named function.

```veyra
add greet()
    say "Hello, World!"
end

greet()
```

Output:

```text
Hello, World!
```

A function definition begins with `add name(...)` and ends with `end`.

The function body runs only when the function is called.

## 2. Parameters and Arguments

Parameters receive values from function calls.

```veyra
add greet(name)
    say "Hello, " + name + "!"
end

greet("Alex")
greet("Sam")
```

Output:

```text
Hello, Alex!
Hello, Sam!
```

Functions can accept multiple parameters.

```veyra
add introduce(name, age)
    say "Name: " + name
    say "Age: " + age
end

introduce("Alex", 20)
```

Output:

```text
Name: Alex
Age: 20
```

## 3. Returning Values

Use `return` to explicitly return a value.

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

### Implicit Return

Veyra also supports implicit returns.

If the final expression in a function produces a value, that value can be returned automatically.

```veyra
add square(number)
    number * number
end

say square(5)
```

Output:

```text
25
```

Explicit `return` is useful for early exits and makes the return value clear.

### Early Return

```veyra
add sign(number)
    if number > 0
        return "positive"
    end

    if number < 0
        return "negative"
    end

    return "zero"
end

say sign(-5)
```

Output:

```text
negative
```

## 4. Multiple Return Values

Veyra supports returning multiple values using commas.

```veyra
add calculate(a, b)
    return a + b, a * b
end

add results = calculate(3, 4)

say results
```

Output:

```text
[7, 12]
```

Multiple returned values are represented as a list.

### Destructuring Results

List destructuring can extract the returned values.

```veyra
[sum, product] = calculate(3, 4)

say sum
say product
```

Output:

```text
7
12
```

## 5. Default Parameters

Default parameters provide values when arguments are omitted.

```veyra
add greet(name = "Guest")
    say "Hello, " + name + "!"
end

greet()
greet("Alex")
```

Output:

```text
Hello, Guest!
Hello, Alex!
```

Defaults can also reference earlier parameters.

```veyra
add calculate(start, finish = start + 10)
    return finish - start
end

say calculate(5)
```

Output:

```text
10
```

Default expressions are evaluated when needed.

## 6. Variadic Parameters

Variadic parameters collect multiple arguments into a list.

Use `...` before the parameter name.

```veyra
add show_items(...items)
    say items
end

show_items("apple", "banana", "orange")
```

### Variadic Sum

```veyra
add sum_numbers(...numbers)
    add total = 0

    repeat number in numbers
        total += number
    end

    return total
end

say sum_numbers(10, 20, 30)
```

Output:

```text
60
```

Veyra supports variadic parameters in different positions within the parameter list.

For example:

```veyra
add show(first, ...middle, last)
    say first
    say middle
    say last
end

show(1, 2, 3, 4, 5)
```

The variadic parameter collects the arguments between the fixed parameters.

## 7. Lambda Expressions

Lambdas are anonymous functions created using `func` and `=>`.

```veyra
add double = func(x) => x * 2

say double(5)
```

Output:

```text
10
```

Lambdas can accept multiple parameters.

```veyra
add add_numbers = func(a, b) => a + b

say add_numbers(3, 7)
```

Output:

```text
10
```

They can also be passed directly as arguments.

## 8. First-Class Functions

Functions can be stored in variables and passed to other functions.

```veyra
add apply_twice(operation, value)
    return operation(operation(value))
end

add double = func(x) => x * 2

say apply_twice(double, 3)
```

Output:

```text
12
```

This allows functions to be used as reusable operations.

## 9. Recursive Functions

A recursive function calls itself.

```veyra
add factorial(n)
    if n <= 1
        return 1
    end

    return n * factorial(n - 1)
end

say factorial(5)
```

Output:

```text
120
```

Every recursive function should have a termination condition.

Veyra limits function call depth to **1,000** to prevent unbounded recursion.

## 10. Nested Functions

Functions can be defined inside other functions.

```veyra
add outer()
    add inner(value)
        return value * 2
    end

    return inner(10)
end

say outer()
```

Output:

```text
20
```

Nested functions follow Veyra's closed-scope rules.

They do not automatically capture variables from the surrounding function.

## 11. Closed Function Scopes

Veyra functions use closed scopes.

A function can access its parameters, local variables, and available functions or built-ins, but it does not implicitly capture surrounding local variables.

For example:

```veyra
add outer()
    add value = 10

    add inner()
        return value
    end

    return inner()
end
```

Calling `outer()` causes an error because `inner()` does not automatically capture `value`.

Pass values explicitly instead:

```veyra
add outer()
    add value = 10

    add inner(number)
        return number * 2
    end

    return inner(value)
end

say outer()
```

Output:

```text
20
```

Closed scopes make function dependencies explicit.

## 12. Function Tracebacks

Veyra 0.2.6 provides function tracebacks when runtime errors occur.

Consider:

```veyra
add divide(a, b)
    return a / b
end

add calculate()
    return divide(10, 0)
end

calculate()
```

The runtime reports the error and includes the function call chain.

Example:

```text
Runtime Error: example.vey:2: Division by zero

  2 |     return a / b
    | ^^^^^^^^^^^^^^^^

  at example.vey:6 in divide()
  at example.vey:9 in calculate()
```

Tracebacks help identify how execution reached the failing function.

Error formatting may vary slightly depending on the execution context.

## 13. Common Mistakes

### Defining a Function Without Calling It

```veyra
add greet()
    say "Hello!"
end
```

The function does not execute until `greet()` is called.

### Missing Required Arguments

```veyra
add greet(name)
    say name
end

greet()
```

Required parameters must receive arguments unless a default value is available.

### Forgetting a Recursive Base Case

```veyra
add countdown(n)
    return countdown(n - 1)
end
```

This function has no termination condition and will eventually reach the function call depth limit.

### Expecting Closure Capture

Nested functions do not automatically inherit surrounding local variables.

Use parameters to provide the values they need.

### Confusing Explicit and Implicit Returns

Both of these are valid:

```veyra
add explicit_square(n)
    return n * n
end

add implicit_square(n)
    n * n
end
```

Use explicit returns when the function has multiple branches or needs an early exit.

## 14. Practice Exercises

### Exercise 1: Greeting Function

Create a function that accepts a name and prints a greeting.

### Exercise 2: Calculator

Create a function that accepts two numbers and returns their sum and product.

### Exercise 3: Default Parameters

Create a function with a default greeting.

### Exercise 4: Variadic Sum

Create a function that accepts any number of values and calculates their sum.

### Exercise 5: Lambda

Create a lambda that triples a number.

### Exercise 6: Function Composition

Write a function that receives another function and applies it twice.

### Exercise 7: Recursion

Write a recursive Fibonacci function.

### Exercise 8: Closed Scope

Create two nested functions and pass a value from the outer function to the inner function explicitly.

### Exercise 9: Traceback

Create a function that divides by zero, call it through another function, and inspect the resulting traceback.

## What's Next?

Continue learning Veyra:

- [Variables and Types](variables.md)
- [Lists](lists.md)
- [Control Flow](control-flow.md)
- [Operators](operators.md)
- [Modules](modules.md)
- [Error Handling](error-handling.md)
