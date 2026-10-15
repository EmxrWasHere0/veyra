# Operators

Veyra supports **9 categories of operators**:

| Category               | Operators          |
| ---------------------- | ------------------ |
| Arithmetic             | `+ - * / %`        |
| Comparison             | `== != < > <= >=`  |
| Logical                | `and or not !`     |
| Assignment             | `= += -= *= /= %=` |
| Channel                | `?`                |
| Lambda arrow           | `=>`               |
| Spread                 | `...`              |
| Conditional assignment | `?=`               |
| Null coalescing        | `??`               |

The `+` operator also supports string concatenation. For example, `"n=" + 42` evaluates to `"n=42"`.

## Arithmetic Operators

Arithmetic operators perform basic mathematical operations.

```text
1 + 5    # 6
7 - 3    # 4
6 * 8    # 48
9 / 3    # 3
8 % 3    # 2
```

## Comparison Operators

Comparison operators compare two values and return a Boolean result (`true` or `false`).

```text
3 == 5    # false
7 != 4    # true
7 >= 7    # true
6 > 6     # false
```

## Logical Operators

Logical operators combine or negate Boolean conditions.

```text
a or b
x and y
not k
!m
```

- `and` evaluates to `true` when both operands are truthy.
- `or` evaluates to `true` when at least one operand is truthy.
- `not` and `!` negate the truthiness of an expression.

## Assignment Operators

Assignment operators assign values to variables or update their existing values.

```text
x = 10    # Assign 10 to x
x += 1    # Equivalent to x = x + 1
x -= 2    # Equivalent to x = x - 2
x *= 4    # Equivalent to x = x * 4
x /= 2    # Equivalent to x = x / 2
x %= 5    # Equivalent to x = x % 5
```

## Channel Operator

The `?` operator retrieves a constant's runtime channel value, falling back to a specified default if no channel override exists.

```text
set x = 10
x = 20

say 0 ? x    # 20
say x        # 10
```

In this example, `x` remains `10`, while assigning `20` to the constant updates its channel. The expression `0 ? x` returns the channel value when available, or `0` otherwise.

## Lambda Arrow Operator

The `=>` operator separates a lambda's parameter list from its expression body.

```text
square = func(x) => x * x
say square(9)    # 81
```

Lambdas are anonymous functions that evaluate a single expression.

## Spread Operator

The `...` operator is used to declare a variadic function parameter, allowing the function to accept multiple arguments. The collected arguments are available as a list.

```text
add sum_all(...nums)
    t = 0
    i = 0

    repeat while i < len(nums)
        t += nums[i]
        i += 1
    end

    t
end

say sum_all(10, 20, 30)    # 60
```

Here, `nums` contains `[10, 20, 30]`, and the function returns their sum.

The `...` syntax is also supported in list destructuring to collect the remaining elements into a list.

## Conditional Assignment Operator

The `?=` operator assigns a value only if the target variable has not already been assigned a value.

```text
x = 10
x ?= 20
say x       # 10

y ?= 30
say y       # 30
```

Since `x` already has a value, its assignment is skipped. Because `y` is not yet defined, it is assigned `30`.

**Note:** Conditional assignment checks whether a value is already assigned; it does not simply check whether the value is `null`.

## Null Coalescing Operator

The `??` operator returns the left-hand value if it is not `null`. Otherwise, it returns the right-hand value.

```text
x = null
y = 10

say x ?? y    # 10

x = 20

say x ?? y    # 20
```

Initially, `x` is `null`, so `x ?? y` evaluates to `10`. After `x` is assigned `20`, the expression evaluates to `20`.
