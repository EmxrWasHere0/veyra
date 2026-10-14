## Language Overview

### Values

| Type       | Example                               |
| ---------- | ------------------------------------- |
| `number`   | `42`, `3.14`, `-7`                    |
| `text`     | `"hello"`, `'single quotes work too'` |
| `bool`     | `true`, `false`                       |
| `list`     | `[1, 2, 3]`, `[[1,2],[3,4]]`          |
| `function` | user-defined or lambda                |
| `null`     | absence of value                      |

Falsy values: `0`, `""`, `[]`, `null`, `false`. Everything else is truthy.

### Variables and Constants

Regular variables are created by assignment and are **dynamically typed**:

```veyra
x = 5
x = "now a string"
x = [1, 2, 3]
```

Constants are declared with `set` and **never change**:

```veyra
set PI = 3.14159
set MAX = 100
```

### Channels

Writing to a constant does **not** modify it — instead it updates a parallel **channel** of the same name:

```veyra
set port = 80       # constant = 80
port = 8080         # channel["port"] = 8080

say port            # 80    (constant)
say 0 ? port        # 8080  (channel)
```

The `?` operator reads from a channel with a required fallback:

```
default ? channel_name
```

- Returns the **channel value** if it exists
- Returns the **default** otherwise

Channels **follow arguments** when passed to a function:

```veyra
set port = 80
port = 8080

add show(c)
    say c           # 80    (the constant value)
    say 0 ? c       # 8080  (channel tag "port" carried via argument)
end

show(port)
```

### I/O

```veyra
say "Hello"              # print with newline
say 1 + 2                # expressions work
say                      # empty line

get "Your name: "        # prompt + read line
say "Hi, " + _input      # input is always stored in _input (as text)
```

To convert input to a number:

```veyra
get "Age: "
age = num(_input)
say age + 1
```

### Operators

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

`+` also concatenates strings: `"n=" + 42` → `"n=42"`.

### Control Flow

**Conditionals:**

```veyra
if age >= 18
    say "adult"
else if age >= 13
    say "teen"
else
    say "child"
end
```

**Counting loop:**

```veyra
repeat 5
    say "hi"
end
```

**Conditional loop:**

```veyra
i = 0
repeat while i < 10
    if i == 5
        stop         # break
    end
    if i % 2 == 0
        i += 1
        fwd          # continue
    end
    say i
    i += 1
end
```

- `stop` → exit the loop
- `fwd` → next iteration

`stop` and `fwd` are **only valid inside loops**. In functions, use `return`.

Safety limit: **1,000,000 iterations** per loop.

**Labeled loops:**

Labels let you control an outer loop from inside a nested loop.

```veyra
@outer repeat 3
    repeat 3
        stop @outer
    end
end
```

Use `stop @label` to exit the labeled loop and `fwd @label` to continue with the next iteration of that loop.

See [`examples/stage2_test.vy`](examples/stage2_test.vy) for more examples.

### Lists

```veyra
nums = [10, 20, 30]
say nums[0]         # 10
nums[1] = 99
say nums            # [10, 99, 30]
say len(nums)       # 3

matrix = [[1,2],[3,4]]
say matrix[1][0]    # 3
```

### Functions

Defined with `add`:

```veyra
add add_two(a, b)
    a + b           # last expression returns automatically
end

say add_two(3, 4)   # 7
```

**Early return:**

```veyra
add sign(n)
    if n > 0
        return "positive"
    end
    if n < 0
        return "negative"
    end
    "zero"
end
```

**Default parameters** (can reference earlier parameters):

```veyra
add range(start, stop = start + 10)
    stop - start
end
say range(5)        # 10
```

**Variadic** (`...` in any position; becomes a list):

```veyra
add sum_all(...nums)
    t = 0
    i = 0
    repeat while i < len(nums)
        t += nums[i]
        i += 1
    end
    t
end
say sum_all(1, 2, 3, 4, 5)   # 15
```

**Lambdas** (single-expression anonymous functions):

```veyra
square = func(x) => x * x
say square(9)       # 81

add apply(f, x, y)
    f(x, y)
end
say apply(func(a, b) => a - b, 10, 3)   # 7
```

**Closed scope** — functions cannot see outer variables:

```veyra
x = 100
add try()
    x + 1           # ERROR: Undefined variable: x
end
```

Only parameters, locals, other `add` functions (globally registered), and built-ins are visible inside a function. This makes functions predictable and side-effect-free with respect to outer state.

Call stack limit: **1,000** frames.

### Error Handling

Veyra supports `try` / `catch` blocks for handling runtime errors.

```veyra
try
    x = 10 / 0
catch err
    say "caught: " + err
end
```

The catch variable is optional:

```veyra
try
    x = undefined_variable
catch
    say "Something went wrong"
end
```

You can also use `??` to provide a fallback when a value is `null`.

```veyra
value = null
say value ?? "fallback"
```

See [`examples/stage2_test.vy`](examples/stage2_test.vy) for nested error handling and additional examples.

### Built-ins

| Function  | Description                                                                 |
| --------- | --------------------------------------------------------------------------- |
| `len(x)`  | Length of a list or text                                                    |
| `num(x)`  | Convert to number                                                           |
| `text(x)` | Convert to text                                                             |
| `type(x)` | Returns `"number"`, `"text"`, `"bool"`, `"list"`, `"function"`, or `"null"` |

### Modules

Veyra includes built-in modules for interacting with the operating system, environment, files, directories, and external processes.

**Available modules:**

- **`system`** — Operating system information, environment variables, command-line arguments, and process exit control.
- **`io`** — File and directory operations, working directory management, and external command execution.
- **`base`** — Basic system information, including hostname, username, home directory, process ID, and CPU core count.

Import a module with `mod`:

```veyra
mod system
mod io
```

Import all exported names from `base`:

```veyra
mod base *
say hostname()
say pid()
```

Import selected names:

```veyra
mod system [get_env, args]
say get_env("HOME")
```

For more examples, see [`examples/mod_test.vy`](examples/mod_test.vy).

**Note:** Functions that access files, environment variables, or external processes interact with the host system.
