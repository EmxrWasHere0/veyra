# Values

In Veyra, we have 6 different values to append:

| Type       | Example                              |
| ---------- | ------------------------------------ |
| `number`   | `42`, `3.14`, `-7`                   |
| `text`     | `"double quotes"`, `'single quoted'` |
| `bool`     | `true`, `false`                      |
| `list`     | `[1, 2, 3]`, `[[1,2],[3,4]]`         |
| `function` | user-defined or lambda               |
| `null`     | absence of value                     |

## `number`

`number` type is made to sum up every type of numbers (`ℝ -> ℝ`). They can have values given down below as an example:

```text
27
4.9
-8
-1.49
```

## `text`

`text` type is for strings. They can have values given down below as an example:

```text
"Hello, World!"
'Hi, Veyra!'
```

## `bool`

`bool` type is to determine if a variable's status is `true`, or `false`. They can have values given down below as an example:

```text
true
false
```

## `list`

`list` type gets many types in it and lets the code control them. They can have values given down as an example:

```text
[1,2,3,4]
[28, "Alex", true]
[null, false]
```

## `function`

`function` type determines a new function to your code. User needs to define what will the function do and result beforehand using it:

```text
add sum(x, y)
    return x + y
end

say sum(7, 14)   # 21
```

## `null`

`null` type is for variables that won't have any types given above. They can have only `null` as a value.
