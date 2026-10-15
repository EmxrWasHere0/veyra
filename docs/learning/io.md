# Input and Output

Input and output allow a program to communicate with the user and interact with files on the computer.

In Veyra, `say` displays values, `get` reads user input, and the `io` module provides file and directory operations.

## What You Will Learn

By the end of this lesson, you will know how to:

- Display text and values using `say`.
- Read user input using `get`.
- Work with the `_input` variable.
- Convert user input into numbers.
- Import and use the `io` module.
- Read, write, and append to files.
- List directories and check whether paths exist.

## 1. Displaying Output with `say`

The `say` statement prints a value to the terminal.

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

You can display the contents of a variable:

```veyra
name = "Veyra"
version = 1

say name
say version
```

Output:

```text
Veyra
1
```

### Combining Text and Values

Use `+` to concatenate text and other values.

```veyra
name = "Alex"
score = 100

say "Player: " + name
say "Score: " + score
```

Output:

```text
Player: Alex
Score: 100
```

## 2. Reading User Input

Use `get` to display a prompt and wait for the user to enter text.

```veyra
get "What is your name? "
```

When the user submits their input, Veyra stores it in the special `_input` variable.

You can copy that value into another variable:

```veyra
get "What is your name? "
name = _input

say "Hello, " + name + "!"
```

Example interaction:

```text
What is your name? Alex
Hello, Alex!
```

The value of `_input` is text. If you need a number, convert it with `num()`.

### A Simple Calculator

```veyra
get "Enter a number: "
number = num(_input)

say "Double: " + number * 2
```

If the user enters `5`, the program displays:

```text
Enter a number: 5
Double: 10
```

If the input is not a valid number, the conversion produces a runtime error.

### Reading Multiple Inputs

Each new `get` updates `_input`, so save each response before asking the next question.

```veyra
get "First number: "
first = num(_input)

get "Second number: "
second = num(_input)

say "Sum: " + (first + second)
```

Example interaction:

```text
First number: 10
Second number: 20
Sum: 30
```

## 3. Importing the `io` Module

Veyra's `io` module provides functions for working with files, directories, and external processes.

Import the module with:

```veyra
mod io
```

You can then call its functions using the `io.` prefix.

For example:

```veyra
mod io

say io.cwd()
```

The `cwd()` function returns the current working directory.

**Important:** Relative file paths are resolved from the process's working directory, which may be different from the directory containing your `.vey` file.

## 4. Writing a File

Use `io.write(path, content)` to write text to a file.

```veyra
mod io

io.write("hello.txt", "Hello from Veyra!")
say "File written."
```

This writes the specified content to `hello.txt`.

Writing to an existing file replaces its previous contents.

## 5. Reading a File

Use `io.read(path)` to read a file's contents.

```veyra
mod io

content = io.read("hello.txt")
say content
```

If `hello.txt` contains `Hello from Veyra!`, the output is:

```text
Hello from Veyra!
```

The file must exist and be readable. If the operation fails, Veyra reports an error.

## 6. Appending to a File

Use `io.append(path, content)` to add content to the end of a file.

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

The `\n` escape sequence adds a newline.

Unlike `io.write`, `io.append` preserves the existing contents and adds the supplied content at the end.

## 7. Checking Whether a Path Exists

Use `io.exists(path)` to check whether a file or directory exists.

```veyra
mod io

if io.exists("hello.txt")
    say "The path exists."
else
    say "The path does not exist."
end
```

This is useful before attempting operations on a path.

## 8. Renaming and Removing Files

Veyra provides functions to rename and remove files.

### Rename a File

```veyra
mod io

io.write("old.txt", "Example")
io.rename("old.txt", "new.txt")

say io.exists("new.txt")
```

The last line prints `true` if the rename succeeds and the new path exists.

### Remove a File

```veyra
mod io

if io.exists("new.txt")
    io.remove("new.txt")
    say "File removed."
end
```

**Warning:** File operations can modify or delete real data. Use paths carefully, especially when working with existing files.

## 9. Working with Directories

The `io` module includes functions for directory management.

### Create a Directory

```veyra
mod io

io.mkdir("my_folder")
say io.exists("my_folder")
```

### List Directory Contents

```veyra
mod io

items = io.listdir(".")

say items
```

The `listdir(path)` function returns the entries in a directory as a list.

The special path `"."` refers to the current working directory.

### Change the Working Directory

```veyra
mod io

io.chdir("my_folder")
say io.cwd()
```

After a successful `chdir`, subsequent relative paths are resolved from the new working directory.

### Remove a Directory

```veyra
mod io

io.rmdir("my_folder")
```

**Warning:** Veyra's `io.rmdir()` removes a directory recursively. It can delete files and subdirectories inside the target directory, so use it only when you are certain the path is correct.

## 10. Running External Commands

The `io` module can also execute external programs.

### Run a Program

Use `io.run(program, args)` to execute a program with an argument list.

```veyra
mod io

result = io.run("echo", ["Hello from Veyra"])
say result
```

The function returns a list containing:

1. The process exit code.
2. Standard output.
3. Standard error.

The exact behavior of an external command can vary by operating system.

### Run a Shell Command

Use `io.shell(command)` to execute a command through the system shell.

```veyra
mod io

result = io.shell("echo Hello from Veyra")
say result
```

On Windows, commands run through `cmd /C`. On other supported platforms, they run through `sh -c`.

**Security note:** Avoid passing untrusted user input directly into shell commands. A crafted input could cause unintended commands to execute.

## 11. Reference: `io` Functions

| Function                   | Purpose                              |
| -------------------------- | ------------------------------------ |
| `io.read(path)`            | Read a file                          |
| `io.write(path, content)`  | Write or replace file contents       |
| `io.append(path, content)` | Append content to a file             |
| `io.exists(path)`          | Check whether a path exists          |
| `io.remove(path)`          | Remove a file                        |
| `io.rename(from, to)`      | Rename a path                        |
| `io.mkdir(path)`           | Create a directory                   |
| `io.rmdir(path)`           | Remove a directory recursively       |
| `io.listdir(path)`         | List directory entries               |
| `io.cwd()`                 | Get the current working directory    |
| `io.chdir(path)`           | Change the current working directory |
| `io.run(program, args)`    | Run an external program              |
| `io.shell(command)`        | Execute a shell command              |

File and process operations may fail because of missing paths, permissions, or operating-system restrictions. Handle such errors appropriately when building larger programs.

## 12. Practice Exercises

### Exercise 1: Greeting Program

Ask the user for their name and display a personalized greeting.

### Exercise 2: Sum Calculator

Ask for two numbers, convert them using `num()`, and display their sum.

### Exercise 3: Notes File

Write a message to `notes.txt`, append a second message, and display the complete contents.

### Exercise 4: File Checker

Ask the user for a filename and use `io.exists()` to report whether that path exists.

### Exercise 5: Directory Explorer

Use `io.cwd()` and `io.listdir(".")` to display the current working directory and its entries.

### Exercise 6: Safe File Cleanup

Create a test directory with a file inside it. Verify the directory's contents before using `io.rmdir()` to remove it. Do not use an important directory for this exercise.

## What's Next?

Continue with:

- [Variables and Types](variables.md)
- [Operators](operators.md)
- [Control Flow](control-flow.md)
- [Modules](modules.md)
- [Error Handling](error-handling.md)
