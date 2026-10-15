# Modules

Modules extend Veyra's built-in functionality by grouping related operations into reusable components. They allow programs to access system information, work with files and directories, and interact with the environment.

Veyra provides three built-in modules:

| Module   | Description                                                                                           |
| -------- | ----------------------------------------------------------------------------------------------------- |
| `system` | Operating system information, environment variables, command-line arguments, and process exit control |
| `io`     | File operations, directory management, and external command execution                                 |
| `base`   | Basic system information, including hostname, username, process ID, and CPU core count                |

## Importing Modules

Use the `mod` keyword to load a module before accessing its members.

### Importing an Entire Module

```text
mod system
mod io

say system.os
say io.cwd()
```

After importing a module, access its members using dot notation:

```text
module_name.member
```

For functions, include parentheses and provide the required arguments.

For example, `system.os` accesses a constant, while `io.cwd()` calls a function.

### Importing All Members

Use `mod name *` to import all members of a module into the current namespace.

```text
mod base *

say os
say arch
say hostname()
say pid()
```

After this import, you can access the imported members directly without the module prefix.

### Importing Selected Members

Use `mod name [member1, member2]` to import only specific members.

```text
mod system [get_env, args]

home = get_env("HOME")
arguments = args()

say home
say arguments
```

This makes the selected members available directly by name.

**Tip:** Importing selected members can make it easier to see which module functionality a program actually uses.

## The `system` Module

The `system` module provides operating system information, environment-variable access, command-line arguments, and process exit control.

Import it with:

```text
mod system
```

### Constants

| Member           | Description                 | Example              |
| ---------------- | --------------------------- | -------------------- |
| `system.os`      | Operating system identifier | `say system.os`      |
| `system.arch`    | CPU architecture identifier | `say system.arch`    |
| `system.family`  | Operating system family     | `say system.family`  |
| `system.version` | Veyra package version       | `say system.version` |

These values are provided by the runtime or build configuration.

### `system.args()`

Returns a list of command-line arguments passed to the Veyra program, excluding the interpreter executable and script filename.

```text
mod system

arguments = system.args()
say arguments
```

Use this function when a script needs to accept arguments from the command line.

### `system.env()`

Returns a list containing environment-variable entries. Each entry is a two-element list containing a variable name and its value.

```text
mod system

variables = system.env()
say variables
```

Because the result may contain many entries, consider processing it rather than printing the entire list in larger applications.

### `system.get_env(name)`

Returns the value of an environment variable as text. If the variable does not exist or cannot be read as a valid Unicode string, the function returns `null`.

```text
mod system

home = system.get_env("HOME")
say home
```

Environment-variable names depend on the operating system and environment. For example, Windows commonly uses `USERPROFILE` for a user's home directory.

You can use `??` to provide a fallback:

```text
mod system

home = system.get_env("HOME") ?? "unknown"
say home
```

### `system.set_env(name, value)`

Sets an environment variable for the running process.

```text
mod system

system.set_env("VEYRA_MODE", "development")

say system.get_env("VEYRA_MODE")
```

Both arguments must be text. The function returns `null`.

**Note:** Setting an environment variable this way does not permanently change the operating system's environment. It affects the running process and its environment.

### `system.exit([code])`

Terminates the process with an optional exit code.

```text
mod system

system.exit(0)
```

- `system.exit()` exits with code `0`.
- `system.exit(1)` exits with code `1`.

Exit codes are commonly used to indicate whether a program completed successfully or encountered a problem. Since this function terminates the process, code after the call will not continue executing.

## The `io` Module

The `io` module provides functions for reading and writing files, managing directories, and running external programs.

Import it with:

```text
mod io
```

### File Operations

| Function                   | Description                                        |
| -------------------------- | -------------------------------------------------- |
| `io.read(path)`            | Reads a text file and returns its contents         |
| `io.write(path, content)`  | Writes text to a file, replacing existing contents |
| `io.append(path, content)` | Appends text to a file, creating it if needed      |
| `io.exists(path)`          | Returns whether a path exists                      |
| `io.remove(path)`          | Removes a file                                     |
| `io.rename(from, to)`      | Renames or moves a file                            |

#### Reading a File

```text
mod io

content = io.read("notes.txt")
say content
```

`io.read()` returns the file contents as text. It produces an error if the file cannot be read.

#### Writing a File

```text
mod io

io.write("notes.txt", "Hello from Veyra!")
```

This creates `notes.txt` if necessary or replaces its existing contents.

#### Appending to a File

```text
mod io

io.append("notes.txt", "\nAnother line")
```

Unlike `io.write()`, `io.append()` preserves existing contents and adds the supplied text to the end of the file.

#### Checking Whether a File Exists

```text
mod io

if io.exists("notes.txt")
    say "The path exists"
else
    say "The path does not exist"
end
```

`io.exists()` returns a Boolean. It checks whether the path exists; it does not guarantee that the path is a regular file or that you can read it.

#### Removing and Renaming Files

```text
mod io

io.rename("old.txt", "new.txt")
io.remove("new.txt")
```

`io.rename()` renames or moves a path according to the host operating system's filesystem rules. `io.remove()` removes a file.

Be careful when using these functions: they modify the filesystem, and removed files may not be recoverable.

### Directory Operations

| Function           | Description                                               |
| ------------------ | --------------------------------------------------------- |
| `io.mkdir(path)`   | Creates a directory, including missing parent directories |
| `io.rmdir(path)`   | Removes a directory and its contents recursively          |
| `io.listdir(path)` | Returns a list of entry names in a directory              |
| `io.cwd()`         | Returns the current working directory                     |
| `io.chdir(path)`   | Changes the current working directory                     |

#### Listing a Directory

```text
mod io

entries = io.listdir(".")
say entries
```

The `"."` path refers to the current working directory. `io.listdir()` returns a list of entry names rather than full paths.

#### Getting the Current Directory

```text
mod io

directory = io.cwd()
say directory
```

#### Creating and Changing Directories

```text
mod io

io.mkdir("my_folder")
io.chdir("my_folder")

say io.cwd()
```

`io.mkdir()` creates the requested directory and any missing parent directories. `io.chdir()` changes the working directory of the running process.

#### Removing a Directory

```text
mod io

io.rmdir("my_folder")
```

**Warning:** `io.rmdir()` removes the specified directory recursively, including its contents. Use it only when you intend to delete everything inside that directory.

### Running External Programs

Veyra provides two functions for running external commands: `io.run()` and `io.shell()`.

Both return a three-element list:

```text
[exit_code, stdout, stderr]
```

- `exit_code` is the process exit code, or `-1` if the operating system does not provide one.
- `stdout` contains standard output as text.
- `stderr` contains standard error as text.

#### `io.run(program, [args])`

Runs an executable directly, optionally passing a list of arguments.

```text
mod io

result = io.run("echo", ["Hello from Veyra"])

say result[0]  # Exit code
say result[1]  # Standard output
say result[2]  # Standard error
```

The example uses `echo`, which may behave differently depending on the operating system. Replace it with an executable available in your environment when necessary.

Passing the executable and arguments separately avoids using a shell to interpret the command string.

#### `io.shell(command)`

Runs a command string through the system shell. Veyra uses `cmd /C` on Windows and `sh -c` on other supported platforms.

```text
mod io

result = io.shell("echo Hello from Veyra")

say result[0]
say result[1]
say result[2]
```

Use `io.shell()` only when shell features are necessary.

**Security warning:** Avoid placing untrusted input into a shell command. Shell metacharacters can change how a command is interpreted. Prefer `io.run()` with a separate argument list when possible, and be cautious with any operation that executes external programs.

## The `base` Module

The `base` module provides basic information about the current system and process.

Import it with:

```text
mod base
```

### Constants

| Member        | Description                 | Example           |
| ------------- | --------------------------- | ----------------- |
| `base.os`     | Operating system identifier | `say base.os`     |
| `base.arch`   | CPU architecture identifier | `say base.arch`   |
| `base.family` | Operating system family     | `say base.family` |

These constants provide platform information from the environment in which Veyra was built or is running.

### Functions

| Function          | Description                                                     |
| ----------------- | --------------------------------------------------------------- |
| `base.hostname()` | Returns the computer's hostname                                 |
| `base.username()` | Returns the current username, when available                    |
| `base.home_dir()` | Returns the user's home directory                               |
| `base.exe_path()` | Returns the path to the Veyra executable                        |
| `base.pid()`      | Returns the current process ID                                  |
| `base.cores()`    | Returns the available parallelism count reported by the runtime |

All six functions take no arguments.

#### Example

```text
mod base

say base.os
say base.arch
say base.hostname()
say base.username()
say base.home_dir()
say base.exe_path()
say base.pid()
say base.cores()
```

The output depends on the computer and environment running the program.

For example, `base.pid()` returns a process identifier, while `base.cores()` reports the parallelism count available to the runtime. This may differ from the total number of physical CPU cores.

## Error Handling

Module operations may fail because of invalid arguments, missing files, insufficient permissions, or unavailable external programs.

Use `try/catch` to handle runtime errors.

```text
mod io

try
    content = io.read("missing_file.txt")
    say content
catch err
    say "Could not read the file: " + err
end
```

The catch variable `err` receives the error message.

Not every unsuccessful operation raises an error. For example, `io.exists()` returns `false` when a path does not exist, and `system.get_env()` returns `null` when an environment variable cannot be retrieved.

## Summary

| Module   | Main capabilities                                                         |
| -------- | ------------------------------------------------------------------------- |
| `system` | OS information, environment variables, arguments, process exit            |
| `io`     | File and directory management, external command execution                 |
| `base`   | Hostname, username, home directory, executable path, PID, CPU parallelism |

Veyra's built-in modules allow programs to interact with the host environment without implementing these operations from scratch. Since several functions can modify files, change process state, or execute external programs, use them carefully and handle errors where appropriate.
