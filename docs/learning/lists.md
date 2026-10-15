# Lists

Lists allow you to store multiple values in a single variable. They are useful for organizing data, processing collections, and working with groups of related values.

In Veyra, lists can contain values of different types, and individual elements can be accessed or updated using their indexes.

## What You Will Learn

By the end of this lesson, you will know how to:

- Create lists.
- Access elements using indexes.
- Update list elements.
- Find the length of a list.
- Work with lists containing different types.
- Destructure lists using `...rest`.
- Use loops to process list elements.

## 1. Creating a List

Create a list by placing values inside square brackets, separated by commas.

```veyra
numbers = [10, 20, 30, 40]
colors = ["red", "green", "blue"]
empty = []
```

You can print a list using `say`.

```veyra
say numbers
say colors
say empty
```

A list can contain numbers, text, Boolean values, and other supported values.

```veyra
data = [42, "Hello", true, null]

say data
```

Veyra lists are dynamically typed, so you do not need to specify a type for each element.

## 2. Accessing List Elements

Use square brackets with an index to access an element.

**List indexes start at `0`.**

```veyra
colors = ["red", "green", "blue"]

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

The first element is at index `0`, the second is at index `1`, and the third is at index `2`.

### Accessing Elements with Variables

You can use a variable as an index.

```veyra
numbers = [10, 20, 30]
index = 1

say numbers[index]
```

Output:

```text
20
```

The index must refer to a valid element in the list. An invalid index can cause a runtime error.

## 3. Updating List Elements

You can change an existing element by assigning a new value to its index.

```veyra
colors = ["red", "green", "blue"]

colors[1] = "yellow"

say colors
```

The second element changes from `"green"` to `"yellow"`.

You can also update numeric elements:

```veyra
numbers = [10, 20, 30]

numbers[0] = 100

say numbers
```

Output:

```text
[100, 20, 30]
```

Updating an element does not require creating a new list.

## 4. Finding the Length of a List

Use the built-in `len()` function to find the number of elements in a list.

```veyra
numbers = [10, 20, 30, 40]

say len(numbers)
```

Output:

```text
4
```

The length is the number of elements, not the index of the final element.

For a list with four elements, the valid indexes are `0`, `1`, `2`, and `3`.

### Checking Whether a List Is Empty

An empty list has a length of zero.

```veyra
items = []

say len(items)
```

Output:

```text
0
```

You can use the length when deciding whether there are any elements to process.

```veyra
items = ["apple", "banana"]

if len(items) > 0
    say "The list contains items."
else
    say "The list is empty."
end
```

## 5. Lists Can Contain Different Types

Veyra does not require all elements of a list to have the same type.

```veyra
profile = ["Alex", 20, true]

say profile[0]
say profile[1]
say profile[2]
```

The list contains text, a number, and a Boolean value.

You can inspect individual element types with `type()`.

```veyra
values = [42, "Veyra", false]

say type(values[0])
say type(values[1])
say type(values[2])
```

Output:

```text
num
text
bool
```

This flexibility is useful, but make sure your program handles each value according to its type.

## 6. Destructuring Lists

List destructuring allows you to assign list elements to multiple variables in one statement.

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

Each variable receives the corresponding element from the list.

Destructuring is useful when a function or operation provides multiple values in a list.

### Using `...rest`

The `...rest` syntax captures the remaining elements of a list.

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

Here:

- `first` receives the first element.
- `rest` receives the remaining elements as a list.

You can also capture the last part of a list after multiple individual elements.

```veyra
[first, second, ...remaining] = [10, 20, 30, 40, 50]

say first
say second
say remaining
```

Output:

```text
10
20
[30, 40, 50]
```

The rest variable is useful when you want to process some elements separately and keep the remaining elements together.

## 7. Processing Lists with Loops

Lists become especially useful when combined with loops.

For example, you can process a list by accessing its elements with an index.

```veyra
numbers = [10, 20, 30]
index = 0

repeat while index < len(numbers)
    say numbers[index]
    index += 1
end
```

Output:

```text
10
20
30
```

The loop starts at index `0` and continues while the index is less than the list's length.

Incrementing the index moves to the next element.

### Calculating a Sum

You can use a loop to calculate the sum of a list of numbers.

```veyra
numbers = [5, 10, 15]
index = 0
total = 0

repeat while index < len(numbers)
    total += numbers[index]
    index += 1
end

say total
```

Output:

```text
30
```

This example demonstrates how to process each element and accumulate a result.

## 8. Nested Lists

A list can contain other lists.

```veyra
grid = [
    [1, 2, 3],
    [4, 5, 6],
    [7, 8, 9]
]
```

Use multiple indexes to access nested elements.

```veyra
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

The first index selects an inner list. The second index selects an element from that inner list.

Nested lists can represent grids, tables, and other structured data.

## 9. Common Mistakes

### Starting at Index 1

Incorrect:

```veyra
colors = ["red", "green", "blue"]

say colors[1]
```

This prints `"green"`, not `"red"`.

Correct:

```veyra
say colors[0]
```

Remember that list indexes start at zero.

### Using an Invalid Index

For a list with three elements, the valid indexes are `0`, `1`, and `2`.

```veyra
numbers = [10, 20, 30]

say numbers[3]
```

The index is outside the valid range and can cause a runtime error.

### Confusing Length with the Last Index

```veyra
numbers = [10, 20, 30]

say len(numbers)
```

The result is `3`, but the last valid index is `2`.

Use `len(numbers) - 1` when you specifically need the last index of a non-empty list.

### Forgetting to Increment a Loop Index

If you do not update the index in a `repeat while` loop, the loop may never reach its stopping condition.

```veyra
index = 0

repeat while index < 3
    say index
    index += 1
end
```

Increment the index when moving through a list this way.

### Assuming Every Element Has the Same Type

A list may contain different types. Check or convert values when your program depends on a specific type.

## 10. Practice Exercises

### Exercise 1: Favorite Foods

Create a list containing at least four favorite foods. Print the first and third elements.

### Exercise 2: Update an Element

Create a list of three numbers and replace the second element with a different number.

### Exercise 3: Count the Elements

Create a list and use `len()` to display its length.

### Exercise 4: Calculate a Total

Create a list of numbers and use a loop to calculate their sum.

### Exercise 5: Destructure a List

Assign the first two elements of a list to separate variables and use `...rest` to capture the remaining elements.

### Exercise 6: Work with a Grid

Create a nested list representing a three-by-three grid. Print the center element.

### Exercise 7: Find the Largest Number

Create a non-empty list of numbers. Use a loop and conditional statements to find and print the largest value.

## What's Next?

You now know how to create, access, update, and process lists in Veyra.

Continue with:

- [Variables and Types](variables.md)
- [Control Flow](control-flow.md)
- [Functions](functions.md)
- [Operators](operators.md)
- [Input and Output](io.md)
