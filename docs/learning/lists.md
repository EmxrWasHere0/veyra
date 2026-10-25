# Lists

Lists allow you to store multiple values in a single variable. They are useful for organizing data, processing collections, and working with groups of related values.

Veyra lists are dynamically typed and can contain values of different types.

## What You Will Learn

By the end of this lesson, you will know how to:

- Create and update lists.
- Access elements using positive and negative indexes.
- Extract portions of lists using slicing.
- Work with nested lists.
- Use list methods.
- Process lists with `map()`, `filter()`, `reduce()`, `any()`, and `all()`.
- Combine lists.
- Destructure lists using `...rest`.
- Iterate over lists using loops.

## 1. Creating Lists

Create a list using square brackets.

```veyra
add numbers = [10, 20, 30, 40]
add colors = ["red", "green", "blue"]
add empty = []

say numbers
say colors
say empty
```

Lists can contain different types:

```veyra
add data = [42, "Hello", true, null]

say data
```

A list can also contain other lists or `jsondata` objects.

```veyra
add users = [
    {"name": "Alex", "age": 20},
    {"name": "Alice", "age": 25}
]

say users[0]["name"]
```

## 2. Accessing Elements

List indexes start at `0`.

```veyra
add colors = ["red", "green", "blue"]

say colors[0]
say colors[1]
say colors[2]
```

Output:

```text
red
green
blue
```

### Negative Indexes

Negative indexes count backward from the end of the list.

```veyra
add numbers = [10, 20, 30, 40]

say numbers[-1]
say numbers[-2]
```

Output:

```text
40
30
```

### Using Variables as Indexes

```veyra
add numbers = [10, 20, 30]
add index = 1

say numbers[index]
```

Output:

```text
20
```

An index outside the valid range causes a runtime error.

## 3. Updating List Elements

Assign a value to an index to replace an element.

```veyra
add colors = ["red", "green", "blue"]

colors[1] = "yellow"

say colors
```

Output:

```text
["red", "yellow", "blue"]
```

Negative indexes can also be used when updating elements.

```veyra
colors[-1] = "purple"

say colors
```

Output:

```text
["red", "yellow", "purple"]
```

### Nested Assignment

Veyra supports updating values inside nested collections.

```veyra
add matrix = [
    [1, 2],
    [3, 4]
]

matrix[1][0] = 99

say matrix
```

Output:

```text
[[1, 2], [99, 4]]
```

## 4. List Length

Use `len()` to find the number of elements.

```veyra
add numbers = [10, 20, 30, 40]

say len(numbers)
say numbers.len()
```

Output:

```text
4
4
```

An empty list has a length of zero.

```veyra
add items = []

say items.len()
```

Output:

```text
0
```

## 5. List Slicing

Slicing extracts a portion of a list.

The general syntax is:

```text
list[start:end:step]
```

- `start` — Starting index.
- `end` — Ending index (exclusive).
- `step` — Distance between selected elements.

All three components can be omitted.

```veyra
add numbers = [10, 20, 30, 40, 50]

say numbers[1:4]
say numbers[:3]
say numbers[2:]
say numbers[::2]
```

Output:

```text
[20, 30, 40]
[10, 20, 30]
[30, 40, 50]
[10, 30, 50]
```

Negative indexes and negative steps are also supported.

```veyra
say numbers[-3:]
say numbers[::-1]
```

Output:

```text
[30, 40, 50]
[50, 40, 30, 20, 10]
```

Slicing returns a new list rather than changing the original.

## 6. List Methods

Veyra provides built-in methods for common list operations.

### Append

Add an element to the end of a list.

```veyra
add numbers = [10, 20]

numbers.append(30)

say numbers
```

Output:

```text
[10, 20, 30]
```

### Pop

Remove an element from a list and return the removed value.

```veyra
add numbers = [10, 20, 30]

add removed = numbers.pop()

say removed
say numbers
```

Output:

```text
30
[10, 20]
```

You can also provide an index:

```veyra
add numbers = [10, 20, 30]

say numbers.pop(0)
say numbers
```

Output:

```text
10
[20, 30]
```

### Insert

Insert an element at a specific index.

```veyra
add numbers = [10, 30]

numbers.insert(1, 20)

say numbers
```

Output:

```text
[10, 20, 30]
```

### Remove

Remove the first element equal to the given value.

```veyra
add numbers = [10, 20, 30]

numbers.remove(20)

say numbers
```

Output:

```text
[10, 30]
```

Unlike `pop()`, `remove()` searches for a value rather than accepting an index.

### Clear

Remove all elements from a list.

```veyra
add numbers = [10, 20, 30]

numbers.clear()

say numbers
```

Output:

```text
[]
```

### Sort

Return a sorted list.

```veyra
add numbers = [30, 10, 20]

add sorted = numbers.sort()

say sorted
say numbers
```

Output:

```text
[10, 20, 30]
[30, 10, 20]
```

`sort()` returns a new list without changing the original.

Sorting supports lists containing numbers or text values of the same type.

### Reverse

Return a reversed list.

```veyra
add numbers = [10, 20, 30]

add reversed = numbers.reverse()

say reversed
say numbers
```

Output:

```text
[30, 20, 10]
[10, 20, 30]
```

`reverse()` also returns a new list.

### Index

Find the index of an element.

```veyra
add colors = ["red", "green", "blue"]

say colors.index("green")
say colors.index("purple")
```

Output:

```text
1
-1
```

If the value is not found, `index()` returns `-1`.

## 7. Functional List Methods

Veyra 0.2.7 introduces functional list methods.

These methods accept callback functions, allowing you to transform, filter, and analyze lists without manually writing loops.

You can use named functions, lambdas, and compatible native functions as callbacks.

### Map

`map(callback)` applies a function to every element and returns a new list.

```veyra
add numbers = [1, 2, 3, 4, 5]

add doubled = numbers.map(func(x) => x * 2)

say doubled
say numbers
```

Output:

```text
[2, 4, 6, 8, 10]
[1, 2, 3, 4, 5]
```

The original list remains unchanged.

You can also use a native function:

```veyra
add words = ["hello", "veyra"]

add lengths = words.map(len)

say lengths
```

Output:

```text
[5, 5]
```

### Filter

`filter(callback)` returns a new list containing only elements for which the callback produces a truthy value.

```veyra
add numbers = [1, 2, 3, 4, 5, 6]

add evens = numbers.filter(func(x) => x % 2 == 0)

say evens
```

Output:

```text
[2, 4, 6]
```

The callback receives one element at a time.

### Reduce

`reduce(callback, initial)` combines all list elements into a single value.

The callback receives two arguments:

- The current accumulator.
- The current list element.

```veyra
add numbers = [5, 10, 15]

add total = numbers.reduce(func(acc, x) => acc + x, 0)

say total
```

Output:

```text
30
```

The second argument to `reduce()` is the initial accumulator value.

You can also multiply all elements:

```veyra
add numbers = [2, 3, 4]

add product = numbers.reduce(func(acc, x) => acc * x, 1)

say product
```

Output:

```text
24
```

### Any

`any(callback)` returns `true` if at least one element satisfies the condition.

```veyra
add numbers = [1, 3, 5, 8]

say numbers.any(func(x) => x % 2 == 0)
say numbers.any(func(x) => x > 10)
```

Output:

```text
true
false
```

Evaluation stops once a truthy result is found.

### All

`all(callback)` returns `true` if every element satisfies the condition.

```veyra
add numbers = [2, 4, 6, 8]

say numbers.all(func(x) => x % 2 == 0)
say numbers.all(func(x) => x > 5)
```

Output:

```text
true
false
```

Evaluation stops once a falsy result is found.

### Empty Lists

Functional methods also work with empty lists.

```veyra
add empty = []

say empty.map(func(x) => x * 2)
say empty.filter(func(x) => true)

say empty.reduce(func(acc, x) => acc + x, 100)

say empty.any(func(x) => true)
say empty.all(func(x) => false)
```

Output:

```text
[]
[]
100
false
true
```

For an empty list, `reduce()` returns its initial value.

`any()` returns `false`, while `all()` returns `true`.

### Functional Method Summary

| Method                      | Description                       | Changes Original |
| --------------------------- | --------------------------------- | ---------------- |
| `map(callback)`             | Transform every element           | No               |
| `filter(callback)`          | Select matching elements          | No               |
| `reduce(callback, initial)` | Combine elements into one value   | No               |
| `any(callback)`             | Check whether any element matches | No               |
| `all(callback)`             | Check whether all elements match  | No               |

## 8. Method Summary

| Method                      | Description                  | Changes Original |
| --------------------------- | ---------------------------- | ---------------- |
| `append(value)`             | Add an element               | Yes              |
| `pop(index)`                | Remove and return an element | Yes              |
| `insert(index, value)`      | Insert an element            | Yes              |
| `remove(value)`             | Remove an element by value   | Yes              |
| `clear()`                   | Remove all elements          | Yes              |
| `sort()`                    | Return a sorted list         | No               |
| `reverse()`                 | Return a reversed list       | No               |
| `index(value)`              | Find an element's index      | No               |
| `len()`                     | Return the list length       | No               |
| `map(callback)`             | Transform elements           | No               |
| `filter(callback)`          | Filter elements              | No               |
| `reduce(callback, initial)` | Reduce elements to one value | No               |
| `any(callback)`             | Test whether any match       | No               |
| `all(callback)`             | Test whether all match       | No               |

`pop()` accepts an optional index. Without an argument, it removes the last element.

## 9. Combining Lists

The `+` operator concatenates lists.

```veyra
add first = [1, 2]
add second = [3, 4]

add combined = first + second

say combined
```

Output:

```text
[1, 2, 3, 4]
```

The original lists are not modified.

## 10. List Destructuring

Destructuring assigns elements to multiple variables.

```veyra
[first, second, third] = [10, 20, 30]

say first
say second
say third
```

Output:

```text
10
20
30
```

### Rest Elements

Use `...rest` to capture remaining elements.

```veyra
[first, ...rest] = [10, 20, 30, 40]

say first
say rest
```

Output:

```text
10
[20, 30, 40]
```

Rest elements are useful when processing lists of varying lengths.

## 11. Iterating Over Lists

The simplest way to iterate over a list is `repeat item in collection`.

```veyra
add numbers = [10, 20, 30]

repeat number in numbers
    say number
end
```

Output:

```text
10
20
30
```

### Index-Based Iteration

You can also use a `while` loop.

```veyra
add numbers = [10, 20, 30]
add index = 0

while index < len(numbers)
    say numbers[index]
    index = index + 1
end
```

### Calculating a Sum

```veyra
add numbers = [5, 10, 15]
add total = 0

repeat number in numbers
    total = total + number
end

say total
```

Output:

```text
30
```

You can also calculate the same result using `reduce()`:

```veyra
add numbers = [5, 10, 15]

add total = numbers.reduce(func(acc, x) => acc + x, 0)

say total
```

Output:

```text
30
```

## 12. Nested Lists

Lists can contain other lists.

```veyra
add grid = [
    [1, 2, 3],
    [4, 5, 6],
    [7, 8, 9]
]

say grid[0][0]
say grid[1][2]
say grid[2][1]
```

Output:

```text
1
6
8
```

Nested elements can also be updated:

```veyra
grid[1][1] = 99

say grid[1]
```

Output:

```text
[4, 99, 6]
```

## 13. Value-Based Lists

Veyra lists use value-based semantics.

```veyra
add original = [1, 2, 3]
add copy = original

copy.append(4)

say original
say copy
```

Output:

```text
[1, 2, 3]
[1, 2, 3, 4]
```

Updating `copy` does not modify `original`.

## 14. Common Mistakes

### Starting at Index 1

List indexing begins at zero.

```veyra
add colors = ["red", "green", "blue"]

say colors[0]
```

Output:

```text
red
```

### Accessing an Invalid Index

```veyra
add numbers = [10, 20, 30]

say numbers[3]
```

This causes a runtime error because the index is outside the list.

### Confusing Length with the Last Index

For a list containing three elements:

```veyra
add numbers = [10, 20, 30]

say len(numbers)
```

The result is `3`, but the last positive index is `2`.

### Expecting `sort()` to Modify the Original

```veyra
add numbers = [3, 1, 2]

numbers.sort()

say numbers
```

The original list remains unchanged.

Assign the result if you want to keep the sorted version:

```veyra
numbers = numbers.sort()
```

### Using a Zero Slice Step

A slice step cannot be zero.

```veyra
add numbers = [1, 2, 3]

say numbers[::0]
```

This causes a runtime error.

### Using the Wrong Callback Argument Count

`map()`, `filter()`, `any()`, and `all()` pass one argument to their callbacks.

`reduce()` passes two arguments.

```veyra
add numbers = [1, 2, 3]

add doubled = numbers.map(func(x) => x * 2)

add total = numbers.reduce(func(acc, x) => acc + x, 0)
```

Make sure your callback accepts the appropriate arguments.

## 15. Practice Exercises

### Exercise 1: Favorite Foods

Create a list of four foods and print the first and last elements.

### Exercise 2: Update an Element

Replace the second element of a list with another value.

### Exercise 3: Slicing

Create a list of six numbers and extract:

- The first three elements.
- The last two elements.
- Every second element.
- The elements in reverse order.

### Exercise 4: List Methods

Create a list and practice using `append()`, `insert()`, `remove()`, and `clear()`.

### Exercise 5: Sorting

Create a list of numbers in random order.

Use `sort()` to produce a sorted copy without modifying the original.

### Exercise 6: Calculate a Total

Use `repeat item in collection` to calculate the sum of a list.

Then calculate the same result using `reduce()`.

### Exercise 7: Nested Assignment

Create a nested list and update one of its inner elements.

### Exercise 8: Destructuring

Destructure a list into two variables and a `...rest` variable.

### Exercise 9: Map

Create a list of numbers and use `map()` to multiply every element by three.

### Exercise 10: Filter

Create a list of numbers and use `filter()` to select values greater than ten.

### Exercise 11: Any and All

Create a list of numbers and use `any()` and `all()` to check different conditions.

## What's Next?

Continue learning Veyra:

- [Variables and Types](variables.md)
- [Control Flow](control-flow.md)
- [Functions](functions.md)
- [Operators](operators.md)
- [Values](values.md)
