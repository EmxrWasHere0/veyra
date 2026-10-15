# Control Flow

Control flow determines the order in which a program executes its instructions. Veyra provides conditional statements, loops, and loop-control keywords to help you decide which code to execute and how many times to execute it.

## Conditionals

Conditional statements let your program make decisions based on Boolean expressions. Use `if` to check a condition, `else if` to check another condition when the previous one is false, and `else` to handle all remaining cases.

```text
if age >= 18
    say "adult"
else if age >= 13
    say "teen"
else
    say "child"
end
```

In this example:

1. If `age` is 18 or greater, the program prints `"adult"`.
2. Otherwise, if `age` is 13 or greater, it prints `"teen"`.
3. If neither condition is true, it prints `"child"`.

Veyra checks the conditions in order and executes only the first matching branch. The `end` keyword marks the end of the conditional block.

**Tip:** The order of conditions matters. Check more specific conditions before broader ones to avoid matching the wrong branch.

## Counting Loops

Use `repeat N` to execute a block a specified number of times. This is useful when you know how many repetitions you need.

```text
repeat 5
    say "hi"
end
```

Output:

```text
hi
hi
hi
hi
hi
```

The loop executes its body five times. Each execution is called an _iteration_.

You can use counting loops for tasks such as repeating a message, processing a fixed number of items, or running a simple sequence of operations.

## Conditional Loops

Use `repeat while` to execute a block repeatedly as long as its condition remains true. This is useful when the number of iterations depends on a condition rather than a fixed count.

```text
i = 0

repeat while i < 5
    say i
    i += 1
end
```

Output:

```text
0
1
2
3
4
```

Here's how it works:

1. `i` starts at `0`.
2. Before each iteration, Veyra checks whether `i < 5`.
3. If the condition is true, the loop body runs.
4. `i += 1` increases `i` by one.
5. When `i` reaches `5`, the condition becomes false and the loop ends.

**Important:** Make sure a conditional loop can eventually reach a false condition. Otherwise, it may continue indefinitely unless interrupted.

## Loop Control: `stop` and `fwd`

Veyra provides two keywords for changing the normal execution of a loop:

- `stop` exits the current loop immediately. It works like `break` in many other languages.
- `fwd` skips the rest of the current iteration and moves to the next iteration. It works like `continue` in many other languages.

Both keywords are only valid inside loops.

### Using `stop`

Use `stop` when you want to terminate a loop before its condition becomes false.

```text
i = 0

repeat while i < 10
    if i == 5
        stop
    end

    say i
    i += 1
end
```

Output:

```text
0
1
2
3
4
```

When `i` reaches `5`, `stop` immediately exits the loop. The `say i` statement is not executed for that iteration.

### Using `fwd`

Use `fwd` when you want to skip the remaining statements in the current iteration without terminating the loop.

```text
i = 0

repeat while i < 6
    i += 1

    if i % 2 == 0
        fwd
    end

    say i
end
```

Output:

```text
1
3
5
```

The expression `i % 2 == 0` checks whether `i` is even. When it is, `fwd` skips `say i` and starts the next iteration. As a result, only odd numbers are printed.

**Important:** Update your loop variable before using `fwd` when that update is needed to make progress. Otherwise, you may accidentally create an infinite loop.

In functions, use `return` to return a value or exit the current function. Unlike `stop` and `fwd`, `return` controls function execution rather than loop execution.

## Labeled Loops

Labels allow you to control an outer loop from inside a nested loop. Place `@label` before a loop to give it a name.

```text
@outer repeat 3
    repeat 3
        say "Inside the nested loop"
        stop @outer
    end

    say "After the nested loop"
end

say "Done"
```

Output:

```text
Inside the nested loop
Done
```

Here, `stop @outer` exits the loop labeled `outer`, even though the statement appears inside the inner loop. Therefore, neither loop continues, and execution proceeds to `say "Done"`.

You can also use `fwd` with a label to continue the next iteration of a specific outer loop.

```text
@outer repeat 3
    repeat 3
        fwd @outer
    end

    say "Next outer iteration"
end
```

The `fwd @outer` statement skips the remaining work in the current outer-loop iteration and moves to its next iteration.

Use labeled loops when nested loops need to exit or continue a specific outer loop. For simple loops, plain `stop` and `fwd` are usually easier to read.

## Loop Safety Limit

Veyra enforces a safety limit of **1,000,000 iterations per loop** to help prevent runaway loops from consuming excessive resources.

This limit is a safeguard, not a replacement for correct loop logic. Always ensure that your loop conditions and variable updates behave as intended.
