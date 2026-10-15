# Lists

Lists allow you to store multiple values in a single variable. In Veyra, lists can contain values of different types, including numbers, strings, Boolean values, and other lists.

Lists are useful when you need to work with collections of data, such as names, scores, or items.

## Creating a List

Create a list by placing values inside square brackets (`[]`), separated by commas.

```text
numbers = [10, 20, 30, 40]
names = ["Alice", "Bob", "Charlie"]
mixed = [10, "hello", true, null]
empty = []
```

- `numbers` contains four numbers.
- `names` contains three strings.
- `mixed` contains values of different types.
- `empty` is an empty list.

You can print a list using `say`.

```text
items = ["apple", "banana", "orange"]
say items
```

## Accessing List Elements

Each element in a list has an index. Veyra uses **zero-based indexing**, which means the first element has index `0`, the second has index `1`, and so on.

```text
colors = ["red", "green", "blue"]

say colors[0]  # red
say colors[1]  # green
say colors[2]  # blue
```

The index identifies the position of an element in the list.

| Index | Value     |
| ----: | --------- |
|   `0` | `"red"`   |
|   `1` | `"green"` |
|   `2` | `"blue"`  |

**Note:** Make sure an index refers to a valid element in the list. Accessing an invalid index may result in an error or another runtime-specific behavior.

## Updating List Elements

You can update an element by assigning a new value to its index.

```text
numbers = [10, 20, 30]

numbers[1] = 99

say numbers  # [10, 99, 30]
```

The element at index `1` changes from `20` to `99`. The other elements remain unchanged.

## Getting the Length of a List

Use `len()` to get the number of elements in a list.

```text
fruits = ["apple", "banana", "orange"]

say len(fruits)  # 3
```

An empty list has a length of `0`.

```text
empty = []
say len(empty)  # 0
```

The length is useful when processing a list with a loop.

## Iterating Over a List

You can use `repeat` and an index to process each element in a list.

```text
fruits = ["apple", "banana", "orange"]
i = 0

repeat while i < len(fruits)
    say fruits[i]
    i += 1
end
```

Output:

```text
apple
banana
orange
```

Here's how it works:

1. `i` starts at `0`.
2. The loop checks whether `i` is smaller than the list's length.
3. `fruits[i]` accesses the current element.
4. `i += 1` advances to the next index.
5. The loop ends when every element has been processed.

Using `len(fruits)` instead of a fixed number makes the loop work with lists of different lengths.

## Nested Lists

A list can contain other lists. These are called _nested lists_.

```text
matrix = [
    [1, 2, 3],
    [4, 5, 6],
    [7, 8, 9]
]

say matrix[0]     # [1, 2, 3]
say matrix[0][1]  # 2
say matrix[2][0]  # 7
```

Use multiple indexes to access elements inside nested lists.

For example, `matrix[0][1]` accesses the second element of the first inner list.

Nested lists can represent grids, tables, matrices, and other structured data.

## List Destructuring

List destructuring allows you to assign list elements to multiple variables in one statement.

```text
[first, second, third] = [10, 20, 30]

say first   # 10
say second  # 20
say third   # 30
```

Each variable receives the value at the corresponding position.

### Collecting Remaining Elements with `...`

You can use `...` to collect the remaining elements into a list.

```text
[first, second, ...rest] = [10, 20, 30, 40, 50]

say first   # 10
say second  # 20
say rest    # [30, 40, 50]
```

In this example, `first` receives `10`, `second` receives `20`, and `rest` contains all remaining elements.

This is useful when you want to extract a few values while keeping the rest together.

## Lists and Functions

Lists can be passed to functions as arguments, allowing you to process collections of values.

```text
add sum_list(nums)
    total = 0
    i = 0

    repeat while i < len(nums)
        total += nums[i]
        i += 1
    end

    total
end

numbers = [10, 20, 30]
say sum_list(numbers)  # 60
```

The function receives the list, adds each element to `total`, and returns the result.

This pattern can be adapted to calculate averages, search for values, or process other kinds of data.

## Summary

| Feature               | Description                                      |
| --------------------- | ------------------------------------------------ |
| `[]`                  | Creates a list                                   |
| `list[index]`         | Accesses an element by its index                 |
| `list[index] = value` | Updates an element                               |
| `len(list)`           | Returns the number of elements                   |
| `repeat while`        | Iterates through a list using an index           |
| Nested lists          | Store lists inside other lists                   |
| List destructuring    | Assigns elements to multiple variables           |
| `...rest`             | Collects remaining elements during destructuring |

Lists are a fundamental part of Veyra. Once you understand indexing, iteration, and destructuring, you can use them to organize and process much larger collections of data.
