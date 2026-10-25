# Input and Output

Input and output allow Veyra programs to communicate with users, work with files, and interact with the operating system.

Veyra provides `say` for displaying output, `get` for reading user input, and built-in modules such as `io` and `fs` for filesystem operations.

## What You Will Learn

By the end of this lesson, you will know how to:

- Display values using `say`.
- Format output with string interpolation.
- Read user input using `get`.
- Display prompts while requesting input.
- Convert input into numbers.
- Read, write, and append to files.
- Manage files and directories.
- Execute external programs.
- Understand the roles of the `io` and `fs` modules.

## 1. Displaying Output

Use `say` to print values to the terminal.

```veyra
say "Hello, World!"
say 42
say true
say null
```

Output:

```text
Hello, World!
42
true
null
```

### Printing Variables

```veyra
add name = "Veyra"
add version = "0.2.7"

say name
say version
```

Output:

```text
Veyra
0.2.7
```

### Combining Text and Values

The `+` operator concatenates two text values:

```veyra
add name = "Alex"

say "Player: " + name
```

Output:

```text
Player: Alex
```

To include numbers or other values in text, use string interpolation.

Veyra 0.2.7 introduces interpolated strings using the `f` prefix:

```veyra
add name = "Alex"
add score = 100

say f"Player: {name}"
say f"Score: {score}"
say f"Next score: {score + 10}"
```

Output:

```text
Player: Alex
Score: 100
Next score: 110
```

Expressions inside `{}` are evaluated using Veyra's normal expression rules.

You can also use multiline interpolation:

```veyra
add report = f"""
Player: {name}
Score: {score}
Passed: {score >= 50}
"""

say report
```

Use `{{` and `}}` to include literal braces in interpolated strings.

## 2. Reading User Input

Use `get` to read a line of text from standard input.

The general syntax is:

```text
get variable "Optional prompt"
```

### Input Without a Prompt

```veyra
get name

say f"Hello, {name}!"
```

Veyra waits for the user to enter a value and stores the result in `name`.

### Input With a Prompt

```veyra
get name "What is your name? "

say f"Hello, {name}!"
```

Example interaction:

```text
What is your name? Alex
Hello, Alex!
```

The prompt is displayed before reading input.

### Dynamic Prompts

The prompt can also be an expression.

```veyra
add field = "username"

get value f"Enter {field}: "

say value
```

This is useful when prompts depend on program data.

## 3. Converting Input

User input is read as text.

Use `num()` to convert numeric input.

```veyra
get age "Enter your age: "

add next_age = num(age) + 1

say f"Next year, you will be {next_age}"
```

Example:

```text
Enter your age: 20
Next year, you will be 21
```

If the input cannot be converted into a number, `num()` raises a runtime error.

### Reading Multiple Values

Each input can be stored in its own variable.

```veyra
get first "First number: "
get second "Second number: "

add total = num(first) + num(second)

say f"Sum: {total}"
```

Example:

```text
First number: 10
Second number: 20
Sum: 30
```

There is no need to copy values from `_input` when using named input variables.

## 4. Importing the IO Module

The `io` module provides file, directory, and process operations.

Import it using:

```veyra
mod io
```

Example:

```veyra
mod io

say io.cwd()
```

This displays the current working directory.

Relative paths are generally resolved from the process's current working directory, which may differ from the script's directory.

## 5. Writing Files

Use `io.write(path, content)` to write text to a file.

```veyra
mod io

add name = "Alex"

io.write("hello.txt", f"Hello, {name}!")

say "File written."
```

Writing to an existing file replaces its previous contents.

## 6. Reading Files

Use `io.read(path)` to read a file.

```veyra
mod io

add content = io.read("hello.txt")

say content
```

Output:

```text
Hello, Alex!
```

The operation may fail if the file does not exist or cannot be read.

## 7. Appending to Files

Use `io.append(path, content)` to add text to the end of a file.

```veyra
mod io

io.write("notes.txt", "First line\n")
io.append("notes.txt", "Second line\n")

say io.read("notes.txt")
```

Output:

```text
First line
Second line
```

Unlike `io.write()`, appending preserves existing content.

## 8. Checking Paths

Use `io.exists(path)` to check whether a path exists.

```veyra
mod io

if io.exists("hello.txt")
    say "File exists."
else
    say "File does not exist."
end
```

A successful existence check does not guarantee that a later operation will succeed. Permissions or filesystem changes may still cause errors.

## 9. Renaming and Removing Files

### Rename

```veyra
mod io

io.write("old.txt", "Example")
io.rename("old.txt", "new.txt")

say io.exists("new.txt")
```

### Remove

```veyra
mod io

if io.exists("new.txt")
    io.remove("new.txt")
    say "File removed."
end
```

File removal affects the actual filesystem and should be used carefully.

## 10. Working with Directories

### Create a Directory

```veyra
mod io

io.mkdir("my_folder")
```

### List Directory Contents

```veyra
mod io

add items = io.listdir(".")

say items
```

The `"."` path refers to the current working directory.

### Change the Working Directory

```veyra
mod io

io.chdir("my_folder")

say io.cwd()
```

Subsequent relative paths use the updated working directory.

### Remove a Directory

```veyra
mod io

io.rmdir("my_folder")
```

**Warning:** `io.rmdir()` removes directories recursively. Files and subdirectories inside the target may also be deleted.

Never use it on important directories without verifying the path.

## 11. Running External Programs

Veyra can execute external programs using `io.run()`.

The standard form is:

```text
io.run(program, [arguments])
```

Example:

```veyra
mod io

add result = io.run("echo", ["Hello from Veyra"])

say result
```

The returned list contains:

1. Process exit code.
2. Standard output.
3. Standard error.

External command behavior can vary across operating systems.

### Shell Commands

Use `io.shell(command)` to execute a command through the system shell.

```veyra
mod io

add result = io.shell("echo Hello from Veyra")

say result
```

On Windows, Veyra uses `cmd /C`. On supported Unix-like systems, it uses `sh -c`.

**Security warning:** Never insert untrusted user input directly into shell commands. Doing so can allow unintended commands to execute.

## 12. The FS Module

Veyra also includes a dedicated `fs` module.

Import it using:

```veyra
mod fs
```

While `io` provides general input/output and process operations, `fs` focuses on filesystem utilities.

For example, the `fs` module includes recursive directory removal through `fs.rmtree()`.

**Warning:** Recursive removal permanently deletes directory contents and cannot be undone through Veyra.

The `fs` module complements `io`; it does not replace it.

For a complete list of module functions, see [Built-ins](built-ins.md).

## 13. IO Function Reference

| Function                   | Description                       |
| -------------------------- | --------------------------------- |
| `io.read(path)`            | Read file contents                |
| `io.write(path, content)`  | Write file contents               |
| `io.append(path, content)` | Append text to a file             |
| `io.exists(path)`          | Check whether a path exists       |
| `io.remove(path)`          | Remove a file                     |
| `io.rename(from, to)`      | Rename a path                     |
| `io.mkdir(path)`           | Create a directory                |
| `io.rmdir(path)`           | Remove a directory recursively    |
| `io.listdir(path)`         | List directory entries            |
| `io.cwd()`                 | Get the current working directory |
| `io.chdir(path)`           | Change the working directory      |
| `io.run(program, args)`    | Execute an external program       |
| `io.shell(command)`        | Execute a shell command           |

## 14. Handling IO Errors

Filesystem and process operations may fail.

Use `try/catch` to handle errors.

```veyra
mod io

try
    add content = io.read("missing.txt")
    say content
catch err
    say f"Unable to read file: {err}"
end
```

This allows your program to respond to the failure instead of terminating immediately.

## 15. Practice Exercises

### Exercise 1: Greeting Program

Ask the user for their name using `get` and display a personalized greeting using string interpolation.

### Exercise 2: Calculator

Read two numbers and display their sum.

### Exercise 3: Notes File

Write two lines to a file using `io.write()` and `io.append()`, then display the result.

### Exercise 4: File Checker

Ask the user for a filename and check whether it exists.

### Exercise 5: Directory Explorer

Display the current working directory and list its contents.

### Exercise 6: Error Handling

Attempt to read a missing file and handle the error with `try/catch`.

### Exercise 7: Dynamic Output

Ask the user for their name and age, then display both values in a multiline interpolated string.

## What's Next?

Continue learning Veyra:

- [Variables and Types](variables.md)
- [Operators](operators.md)
- [Control Flow](control-flow.md)
- [Modules](modules.md)
- [Error Handling](error-handling.md)
