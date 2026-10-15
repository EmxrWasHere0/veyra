# Functions

Functions are reusable blocks of code that perform a specific task. They help you organize your program, avoid repeating code, and divide complex problems into smaller, manageable parts.

In Veyra, you can define functions, pass arguments to them, return values, use recursion, and create anonymous functions with lambda expressions.

## Defining a Function

Define a function using the `add` keyword, followed by its name and parameter list. Use `end` to mark the end of the function.

```text
add greet()
    say "Hello, world!"
end

greet()
```

Output:

```text
Hello, world!
```

In this example:

1. `add greet()` defines a function named `greet`.
2. The code between `add` and `end` forms the function body.
3. `greet()` calls the function and executes its body.

Defining a function does not necessarily execute its body. The function runs when it is called.

## Parameters and Arguments

**Parameters** are variables declared in a function definition. **Arguments** are the actual values passed to the function when it is called.

```text
add greet(name)
    say "Hello, " + name
end

greet("Alice")
greet("Bob")
```

Output:

```text
Hello, Alice
Hello, Bob
```

Here, `name` is a parameter. `"Alice"` and `"Bob"` are arguments.

You can define multiple parameters by separating them with commas.

```text
add add_numbers(a, b)
    a + b
end

say add_numbers(10, 20)  # 30
```

The function evaluates `a + b` and returns the result.

## Returning Values

Functions can return values that you can store in variables or use in expressions.

```text
add square(x)
    return x * x
end

result = square(5)
say result  # 25
```

The `return` keyword immediately exits the current function and returns the specified value.

Veyra also allows the final expression of a function to provide its result, as shown here:

```text
add square(x)
    x * x
end

say square(5)  # 25
```

This makes simple functions shorter and easier to read.

**Tip:** Use `return` when you want to return early from a function or make the intended return point explicit.

## Default Parameters

Default parameters provide a value when an argument is omitted from a function call.

```text
add greet(name = "Guest")
    say "Hello, " + name
end

greet("Alice")  # Hello, Alice
greet()         # Hello, Guest
```

When you pass `"Alice"`, the function uses that value. When you omit the argument, it uses `"Guest"` instead.

Default parameters are useful when a function has a common value that callers do not always need to specify.

## Variadic Parameters

A variadic parameter allows a function to accept multiple arguments through a single parameter. In Veyra, prefix the parameter name with `...`.

```text
add sum_all(...nums)
    total = 0
    i = 0

    repeat while i < len(nums)
        total += nums[i]
        i += 1
    end

    total
end

say sum_all(1, 2, 3)       # 6
say sum_all(10, 20, 30, 40) # 100
```

The `...nums` parameter collects the supplied arguments into a list named `nums`. The function then iterates over the list and adds its elements together.

Variadic parameters are useful when the number of arguments can change between calls.

## Lambda Functions

A lambda is an anonymous function, meaning it is a function without a traditional named definition. In Veyra, use the `func` keyword and the `=>` operator to create a lambda expression.

```text
square = func(x) => x * x

say square(9)  # 81
```

The expression before `=>` contains the parameters, while the expression after it is the function body.

Lambdas are useful when you want to create a small function and assign it to a variable.

### Passing a Lambda to Another Function

You can pass a lambda as an argument to another function.

```text
add apply(f, x, y)
    f(x, y)
end

subtract = func(a, b) => a - b

say apply(subtract, 10, 3)  # 7
say apply(func(a, b) => a + b, 10, 3)  # 13
```

In this example, `apply` receives a function as its first argument and calls it with `x` and `y`.

This technique allows you to reuse the same function structure with different operations.

**Note:** Veyra's lambdas use closed scope and do not capture local variables from their surrounding scope. If a lambda needs a value, pass it as an argument.

## Recursion

Recursion occurs when a function calls itself. It is useful for problems that can be broken down into smaller versions of the same problem.

For example, a factorial function can be defined recursively:

```text
add factorial(n)
    if n <= 1
        return 1
    end

    return n * factorial(n - 1)
end

say factorial(5)  # 120
```

The function calculates the factorial by multiplying `n` by the factorial of `n - 1`. When `n` reaches `1` or less, the base case returns `1` and stops the recursive calls.

For `factorial(5)`, the calculation is:

`5 * 4 * 3 * 2 * 1 = 120`

**Important:** Every recursive function needs a base case that eventually stops the recursion. Without one, the function may continue calling itself until a runtime limit or error occurs.

## Nested Functions

Veyra supports defining functions inside other functions.

```text
add outer()
    add inner()
        say "Inside inner"
    end

    inner()
end

outer()
```

Output:

```text
Inside inner
```

Here, `inner` is defined inside `outer` and called from within it.

Nested functions can help organize helper logic that belongs to a larger operation.

## Multiple Return Values

Veyra supports returning multiple values from a function. You can use list destructuring to assign those values to separate variables.

```text
add get_coordinates()
    return 10, 20
end

[x, y] = get_coordinates()

say x  # 10
say y  # 20
```

This example returns two values and assigns them to `x` and `y`.

Multiple return values are useful when a function needs to provide several related results.

## Summary

| Feature                 | Description                                   |
| ----------------------- | --------------------------------------------- |
| `add name()`            | Defines a named function                      |
| `name()`                | Calls a function                              |
| Parameters              | Receive arguments passed to a function        |
| `return`                | Exits a function and returns a value          |
| Default parameters      | Provide fallback values for omitted arguments |
| `...args`               | Collects multiple arguments into a list       |
| `func(x) => expression` | Creates an anonymous lambda function          |
| Recursion               | Allows a function to call itself              |
| Nested functions        | Defines a function inside another function    |
| Multiple return values  | Returns several values from one function      |

Functions are one of the most important building blocks in Veyra. By combining parameters, return values, lambdas, and recursion, you can write code that is easier to reuse, test, and maintain.
