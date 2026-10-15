# Modules

Modules provide additional functionality in Veyra. They let you access features such as system information, environment variables, file operations, and operating system utilities.

Veyra includes three built-in modules:

- `system` — system information, environment variables, command-line arguments, and process exit.
- `io` — file operations, directory management, and process execution.
- `base` — general system utilities such as the hostname, username, and home directory.

## What You Will Learn

By the end of this guide, you will understand:

- How to import modules.
- How to import selected functions.
- How to use the `system` module.
- How to read and write files with `io`.
- How to access basic system information with `base`.
- How to handle errors when using module functions.

## 1. Importing Modules

Use `mod` to import a module.

```veyra
mod system

say system.os
say system.arch
say system.version
```

The module prefix lets you access its members, such as `system.os`.

### Importing All Members

You can import all members from a module using `*`.

```veyra
mod system *

say os
say arch
say version
```

### Importing Selected Members

Use square brackets to import only the members you need.

```veyra
mod system [get_env, set_env]

value = get_env()

say value
```

Use the selected import syntax when you want to make specific module members available without writing the module prefix each time.

> **Note:** Check the function signatures in the sections below before calling a function. For example, `get_env` requires an environment-variable name in this module API.

## 2. The `system` Module

The `system` module provides information about the runtime environment and functions for interacting with environment variables and the running process.

Import it with:

```veyra
mod system
```

### System Constants

| Member           | Description             |
| ---------------- | ----------------------- |
| `system.os`      | Operating system name   |
| `system.arch`    | System architecture     |
| `system.family`  | Operating system family |
| `system.version` | Veyra version           |

Example:

```veyra
mod system

say system.os
say system.arch
say system.family
say system.version
```

The exact output depends on the system running Veyra and the version of the program.

### Environment Variables

Use `get_env(name)` to read an environment variable.

```veyra
mod system

home = system.get_env("HOME")

say home
```

The environment variable may not exist, and its value can differ between operating systems. On Windows, for example, the `USERPROFILE` variable is commonly used for the current user's profile directory.

Use `set_env(name, value)` to set an environment variable for the running process.

```veyra
mod system

system.set_env("VEYRA_MODE", "development")

say system.get_env("VEYRA_MODE")
```

Environment-variable changes should not be assumed to persist after the process exits.

### Command-Line Arguments

The `system.args()` function provides command-line arguments exposed by the runtime.

```veyra
mod system

args = system.args()

say args
```

The exact arguments available depend on how Veyra is launched. If you plan to use command-line arguments in a program, verify the behavior of your installed CLI version.

### Exiting a Program

Use `system.exit(code)` to terminate the running process with an exit code.

```veyra
mod system

system.exit(0)
```

Conventionally, `0` indicates success, while a nonzero code indicates an error or another unsuccessful outcome.

## 3. The `io` Module

The `io` module provides file, directory, and process operations.

Import it with:

```veyra
mod io
```

### Reading a File

Use `io.read(path)` to read a file.

```veyra
mod io

content = io.read("notes.txt")

say content
```

The file must be accessible to the running process. If reading fails, handle the runtime error where appropriate.

### Writing a File

Use `io.write(path, content)` to write content to a file.

```veyra
mod io

io.write("hello.txt", "Hello from Veyra!")

say "File written."
```

Writing may create a file or replace existing content, so use it carefully when working with important files.

### Appending to a File

Use `io.append(path, content)` to append content to a file.

```veyra
mod io

io.append("log.txt", "Application started\n")

say "Log updated."
```

Appending is useful for logs because it adds content rather than intentionally replacing the existing file.

### Checking Whether a File Exists

Use `io.exists(path)` to check whether a path exists.

```veyra
mod io

if io.exists("hello.txt")
    say "The path exists."
else
    say "The path does not exist."
end
```

This checks existence; it does not guarantee that a file is readable or writable.

### Removing and Renaming Paths

Use `io.remove(path)` to remove a file and `io.rename(from, to)` to rename or move a path.

```veyra
mod io

io.rename("hello.txt", "greeting.txt")
```

Be careful when removing files or changing paths. Verify the target before running operations on important data.

### Creating and Removing Directories

Use `io.mkdir(path)` to create a directory.

```veyra
mod io

io.mkdir("my_folder")
```

Use `io.rmdir(path)` to remove a directory recursively.

```veyra
mod io

io.rmdir("my_folder")
```

**Warning:** Recursive directory removal can delete files and subdirectories inside the target. Only use it when you are certain the target is safe to remove.

### Listing Directory Contents

Use `io.listdir(path)` to list entries in a directory.

```veyra
mod io

entries = io.listdir(".")

say entries
```

The path `"."` refers to the current working directory.

### Working with Directories

Use `io.cwd()` to get the current working directory and `io.chdir(path)` to change it.

```veyra
mod io

say io.cwd()

io.chdir("..")

say io.cwd()
```

A relative path is interpreted in relation to the current working directory.

## 4. Running External Programs

The `io` module provides functions for running external commands.

### Running a Program

Use `io.run(program, args)` to run a program with an argument list.

```veyra
mod io

result = io.run("echo", ["Hello from Veyra"])

say result
```

The result is a list containing:

1. The exit code.
2. Standard output.
3. Standard error.

An exit code of `-1` may be returned when the operating system does not provide an exit code.

The availability and behavior of a particular executable depend on the operating system and environment.

### Running a Shell Command

Use `io.shell(command)` to execute a command through the system shell.

```veyra
mod io

result = io.shell("echo Hello from Veyra")

say result
```

Like `io.run`, the result contains the exit code, standard output, and standard error.

**Security warning:** Shell commands can perform actions with the permissions of the current user. Avoid passing untrusted input into `io.shell()`. Prefer `io.run()` with a program and separate arguments when possible.

## 5. The `base` Module

The `base` module provides general system utilities.

Import it with:

```veyra
mod base
```

### System Information

| Member        | Description             |
| ------------- | ----------------------- |
| `base.os`     | Operating system name   |
| `base.arch`   | System architecture     |
| `base.family` | Operating system family |

Example:

```veyra
mod base

say base.os
say base.arch
say base.family
```

### Utility Functions

| Function          | Description                                |
| ----------------- | ------------------------------------------ |
| `base.hostname()` | Returns the hostname                       |
| `base.username()` | Returns the current username               |
| `base.home_dir()` | Returns the home directory                 |
| `base.exe_path()` | Returns the executable path                |
| `base.pid()`      | Returns the process ID                     |
| `base.cores()`    | Returns the available processor-core count |

Example:

```veyra
mod base

say "Hostname: " + base.hostname()
say "Username: " + base.username()
say "Home: " + base.home_dir()
say "Executable: " + base.exe_path()
say "PID: " + base.pid()
say "CPU cores: " + base.cores()
```

Returned values depend on the host system and runtime environment.

## 6. Handling Module Errors

Module operations may fail because of missing files, invalid paths, insufficient permissions, or unavailable programs.

Use `try` and `catch` to handle runtime errors.

```veyra
mod io

try
    content = io.read("missing_file.txt")
    say content
catch err
    say "Operation failed: " + err
end
```

The `catch` variable receives the runtime error message.

For more information, see the [Error Handling Guide](error-handling.md).

## 7. Module Reference

### `system`

| Member                            | Purpose                                       |
| --------------------------------- | --------------------------------------------- |
| `os`, `arch`, `family`, `version` | Runtime and system information                |
| `args()`                          | Command-line arguments exposed by the runtime |
| `env()`                           | Environment information                       |
| `get_env(name)`                   | Read an environment variable                  |
| `set_env(name, value)`            | Set an environment variable                   |
| `exit(code)`                      | Exit the process                              |

### `io`

| Function                | Purpose                              |
| ----------------------- | ------------------------------------ |
| `read(path)`            | Read a file                          |
| `write(path, content)`  | Write a file                         |
| `append(path, content)` | Append to a file                     |
| `exists(path)`          | Check whether a path exists          |
| `remove(path)`          | Remove a file                        |
| `rename(from, to)`      | Rename or move a path                |
| `mkdir(path)`           | Create a directory                   |
| `rmdir(path)`           | Recursively remove a directory       |
| `listdir(path)`         | List directory entries               |
| `cwd()`                 | Get the current working directory    |
| `chdir(path)`           | Change the current working directory |
| `run(program, args)`    | Run an external program              |
| `shell(command)`        | Run a shell command                  |

### `base`

| Function     | Purpose                      |
| ------------ | ---------------------------- |
| `hostname()` | Get the hostname             |
| `username()` | Get the current username     |
| `home_dir()` | Get the home directory       |
| `exe_path()` | Get the executable path      |
| `pid()`      | Get the process ID           |
| `cores()`    | Get the processor-core count |

## 8. Practice Exercises

### Exercise 1: System Information

Write a program that displays the operating system, architecture, and Veyra version.

### Exercise 2: File Operations

Write a short message to a file, read it back, and display its contents.

### Exercise 3: Directory Listing

Display the contents of the current directory.

### Exercise 4: Environment Variables

Set an environment variable and read its value back during the same program execution.

### Exercise 5: Error Handling

Attempt to read a file that does not exist and handle the resulting error using `try` and `catch`.

### Exercise 6: System Utilities

Display the hostname, username, home directory, and process ID.

## What's Next?

You now know how to import Veyra's built-in modules and use their main APIs.

Continue learning:

- [Error Handling](error-handling.md)
- [Input and Output](io.md)
- [Functions](functions.md)
- [Operators](operators.md)
