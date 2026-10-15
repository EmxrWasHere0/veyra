# Functions

Functions allow you to organize code into reusable blocks. Instead of writing the same instructions repeatedly, you can define a function once and call it whenever you need it.

Veyra supports named functions, parameters, default parameter values, variadic parameters, and lambda expressions.

## What You Will Learn

By the end of this lesson, you will know how to:

- Define and call functions.
- Pass arguments to functions.
- Return values with `return`.
- Use default parameter values.
- Accept multiple arguments with variadic parameters.
- Create lambda expressions.
- Understand function scope.

## 1. Defining a Function

Use `add` to define a named function.

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

Here's how it works:

- `add` begins the function definition.
- `greet` is the function's name.
- `()` contains the parameter list.
- `end` closes the function definition.
- `greet()` calls the function.

Defining a function does not execute its body. The body runs when the function is called.

## 2. Using Parameters

Parameters allow functions to receive values from their callers.

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

In this example, `name` is a parameter. When the function is called, the supplied argument becomes the parameter's value.

You can define multiple parameters:

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

Arguments are supplied in the order of the parameters.

## 3. Returning Values

Use `return` to send a value back to the caller.

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

The expression `number * number` is returned by the function and assigned to `result`.

You can use a returned value directly:

```veyra
add add_numbers(a, b)
    return a + b
end

say add_numbers(10, 20)
```

Output:

```text
30
```

Returning a value is useful when a function calculates something that another part of your program needs.

## 4. Functions Without an Explicit Return Value

A function does not have to return a meaningful result.

```veyra
add show_message(message)
    say message
end

show_message("Welcome to Veyra!")
```

This function displays a message rather than calculating a result.

If you need a function's result in another expression, explicitly return the value you want to use.

## 5. Default Parameter Values

Veyra supports default values for parameters. A default is used when the corresponding argument is omitted.

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

The first call uses the default value `"Guest"`. The second supplies `"Alex"`.

Default parameters are useful when a function should work even when the caller does not provide every optional argument.

## 6. Variadic Parameters

Variadic parameters allow a function to accept multiple arguments through a single parameter.

Veyra uses `...` for variadic parameters.

```veyra
add show_items(...items)
    say items
end

show_items("apple", "banana", "orange")
```

The variadic parameter collects the supplied arguments into a list.

You can then process that list:

```veyra
add sum_numbers(...numbers)
    index = 0
    total = 0

    repeat while index < len(numbers)
        total += numbers[index]
        index += 1
    end

    return total
end

say sum_numbers(10, 20, 30)
```

Output:

```text
60
```

Variadic parameters are useful when the number of arguments is not known in advance.

## 7. Lambda Expressions

A lambda is an anonymous function expression.

Veyra supports the `func` keyword and the `=>` operator for creating lambdas.

```veyra
double = func(x) => x * 2

say double(5)
```

Output:

```text
10
```

The expression `func(x) => x * 2` creates a function that receives `x` and evaluates `x * 2`.

You can assign a lambda to a variable and call it like a named function.

```veyra
add_numbers = func(a, b) => a + b

say add_numbers(3, 7)
```

Output:

```text
10
```

Lambda expressions are convenient for short functions.

## 8. Passing Functions Around

Functions can be stored in variables and passed as values.

```veyra
add apply_twice(operation, value)
    return operation(operation(value))
end

double = func(x) => x * 2

say apply_twice(double, 3)
```

Output:

```text
12
```

The function `apply_twice` receives another function as an argument and calls it twice.

This technique can help you reuse behavior without duplicating code.

## 9. Understanding Function Scope

Function scope determines which variables a function can access.

Veyra functions use a closed scope rather than automatically capturing variables from their surrounding scope. If a function needs a value, pass it as a parameter or define it within the function.

For example:

```veyra
add multiply(a, b)
    return a * b
end

say multiply(4, 5)
```

Output:

```text
20
```

The function receives both values explicitly, so it does not depend on variables from its caller's surrounding scope.

**Tip:** Passing required values as parameters makes functions easier to understand, test, and reuse.

## 10. Common Mistakes

### Defining a Function but Never Calling It

```veyra
add greet()
    say "Hello!"
end
```

This defines the function, but its body does not execute until you call it:

```veyra
greet()
```

### Forgetting to Return a Result

```veyra
add square(number)
    number * number
end
```

If you want to use the calculated result, write:

```veyra
add square(number)
    return number * number
end
```

### Passing the Wrong Number of Arguments

Check the function's parameter list and make sure the required arguments are supplied. Default and variadic parameters can provide additional flexibility.

### Confusing Parameters and Arguments

In this definition:

```veyra
add greet(name)
    say name
end
```

`name` is a parameter.

In this call:

```veyra
greet("Alex")
```

`"Alex"` is an argument.

### Expecting Automatic Closure Capture

Do not assume that a function automatically inherits variables from its surrounding scope. Pass required values explicitly.

## 11. Practice Exercises

### Exercise 1: Greeting Function

Create a function named `greet` that accepts a name and prints a greeting.

### Exercise 2: Calculator Function

Create a function named `multiply` that accepts two numbers and returns their product.

### Exercise 3: Default Greeting

Create a function with a default name of `"Guest"`. Test it with and without an argument.

### Exercise 4: Variadic Sum

Create a function that accepts any number of numeric arguments and returns their sum.

### Exercise 5: Lambda

Create a lambda that triples a number. Store it in a variable and call it with several values.

### Exercise 6: Function Composition

Create two functions: one that doubles a number and another that adds five. Pass the first function into a function that calls its operation before adding five.

### Exercise 7: Scope

Write a function that receives two values through parameters and returns their result after a calculation. Avoid relying on variables from the surrounding scope.

## What's Next?

You now know the fundamentals of functions in Veyra.

Continue with:

- [Variables and Types](variables.md)
- [Lists](lists.md)
- [Control Flow](control-flow.md)
- [Operators](operators.md)
- [Modules](modules.md)
- [Error Handling](error-handling.md)
