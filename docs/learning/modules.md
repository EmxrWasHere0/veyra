# Modules

Modules allow Veyra programs to organize reusable code and access additional functionality.

Veyra supports built-in modules implemented in Rust and user-defined modules written in `.veymod` files.

## What You Will Learn

By the end of this lesson, you will know how to:

- Import built-in modules.
- Import all or selected module members.
- Create user-defined `.veymod` modules.
- Export functions and constants.
- Understand module isolation.
- Configure module search paths.
- Use the `VEYRA_PATH` environment variable.
- Handle module-related errors.

## 1. Built-in Modules

Veyra includes ten built-in modules.

| Module      | Description                                                              |
| ----------- | ------------------------------------------------------------------------ |
| `system`    | Operating system information, environment variables, and process control |
| `io`        | File operations, directories, and external processes                     |
| `base`      | Hostname, username, home directory, and other system information         |
| `json`      | JSON parsing and serialization                                           |
| `http`      | HTTP requests                                                            |
| `websocket` | WebSocket connections                                                    |
| `math`      | Mathematical operations and constants                                    |
| `time`      | Date, time, and timing utilities                                         |
| `fs`        | Filesystem utilities                                                     |
| `random`    | Pseudorandom number generation                                           |

Built-in modules are implemented in Rust and compiled into the Veyra interpreter.

They do not require separate `.veymod` files.

## 2. Importing Modules

Use `mod` to import a module.

```veyra
mod system

say system.os
say system.arch
```

After importing a module, access its members using the module name followed by a dot.

### Module Constants and Functions

Module constants are accessed without parentheses:

```veyra
mod system

say system.os
say system.version
```

Functions require parentheses:

```veyra
mod system

say system.args()
say system.get_env("HOME")
```

### Importing All Members

Use `*` to import all exported members directly.

```veyra
mod base *

say hostname()
say pid()
```

This makes exported names available without the `base.` prefix.

### Importing Selected Members

Use square brackets to import specific names.

```veyra
mod system [get_env, args]

say get_env("HOME")
say args()
```

Selective imports can make dependencies more explicit.

## 3. Using Built-in Modules

### Math

```veyra
mod math

say math.sqrt(144)
```

### JSON

```veyra
mod json

add data = json.parse("{\"name\":\"Veyra\"}")

say data["name"]
```

JSON objects are represented as Veyra `jsondata` values.

### Time

```veyra
mod time

say time.now()
```

### Random

```veyra
mod random
```

The `random` module provides pseudorandom operations.

Its output should not be used for cryptographic or security-sensitive purposes.

### Filesystem

```veyra
mod fs
```

The `fs` module provides filesystem-related utilities.

Some operations, such as recursive directory removal, can permanently delete data.

### Networking

Veyra includes `http` and `websocket` modules for network communication.

```veyra
mod http
mod websocket
```

Network operations can fail due to connectivity, timeouts, invalid responses, or remote server behavior.

For detailed APIs, see [Built-ins](built-ins.md).

## 4. User-Defined Modules

Veyra supports user-defined modules written in `.veymod` files.

These modules are interpreted at runtime.

### Creating a Module

Create a file named `calculator.veymod`:

```veyra
set VERSION = "1.0"

add add_numbers(a, b)
    return a + b
end

add multiply(a, b)
    return a * b
end
```

### Importing the Module

Create a Veyra program in the same directory:

```veyra
mod calculator

say calculator.VERSION
say calculator.add_numbers(10, 20)
say calculator.multiply(4, 5)
```

Output:

```text
1.0
30
20
```

The interpreter searches supported module directories for `calculator.veymod`.

## 5. Module Exports

User-defined modules expose:

- Constants declared with `set`.
- Functions declared using `add name(...)`.

Mutable variables declared with `add name = value` are not automatically exported.

### Private Members

Names beginning with `_` are private.

```veyra
set VERSION = "1.0"

add _internal_helper(value)
    return value * 2
end

add calculate(value)
    return _internal_helper(value) + 1
end
```

The module can use `_internal_helper()` internally, but other modules cannot import it as a public member.

This allows modules to keep implementation details private.

## 6. Module Isolation

User-defined modules execute in isolated module environments.

Variables created inside a module do not automatically become variables in the importing program.

For example, `settings.veymod`:

```veyra
add internal_value = 100

set APP_NAME = "Veyra"
```

Main program:

```veyra
mod settings

say settings.APP_NAME
```

The constant is exported, but `internal_value` is not.

### Functions and Module Context

Functions defined inside modules retain their module context when called.

This is important for callbacks and recursive module functions.

For example, a module function can use imported modules that belong to its defining module without requiring the caller to import them separately.

### Closed Scopes

Veyra's closed-scope function behavior also applies to module functions.

A function does not automatically capture arbitrary surrounding local variables.

Pass required values explicitly through parameters.

## 7. Module Search Paths

Veyra searches multiple locations when importing user-defined modules.

The search locations include:

1. The directory containing the running script.
2. The script directory's `modules/` folder.
3. The script directory's `lib/` folder.
4. The current working directory.
5. The working directory's `modules/` folder.
6. The working directory's `lib/` folder.
7. Additional directories configured through `VEYRA_PATH`.
8. User-level module directories.
9. System-level module directories.

The exact user and system module directories depend on the operating system.

When multiple directories contain a module with the same name, search order determines which module is loaded.

## 8. The VEYRA_PATH Environment Variable

Use `VEYRA_PATH` to add custom module search directories.

### Linux and macOS

```bash
export VEYRA_PATH="/home/user/veyra-modules:/opt/veyra/modules"
```

### Windows PowerShell

```powershell
$env:VEYRA_PATH = "C:\veyra\modules;D:\shared\veyra"
```

Veyra uses the operating system's path-list separator:

- Linux/macOS: `:`
- Windows: `;`

The variable can contain multiple directories.

After configuring `VEYRA_PATH`, modules inside those directories can be imported by name.

Example:

```veyra
mod calculator
```

You do not need to specify the full path when importing a module located in a configured search directory.

## 9. Importing User-Defined Module Members

User-defined modules support the same import syntax as built-in modules.

### Regular Import

```veyra
mod calculator

say calculator.add_numbers(2, 3)
```

### Import All

```veyra
mod calculator *

say add_numbers(2, 3)
```

### Selective Import

```veyra
mod calculator [add_numbers]

say add_numbers(2, 3)
```

Private members beginning with `_` are not exported.

## 10. Handling Module Errors

Module imports or function calls may fail.

Common causes include:

- The module file cannot be found.
- A requested member does not exist.
- The requested member is private.
- The module contains invalid Veyra code.
- A module function raises a runtime error.
- A built-in module operation fails.

Use `try/catch` when an operation may fail during execution.

```veyra
mod io

try
    add content = io.read("missing.txt")
    say content
catch err
    say "Operation failed: " + err
end
```

Veyra 0.2.6 also provides source locations and function tracebacks for debugging runtime errors.

## 11. Built-in vs. User-Defined Modules

| Feature                | Built-in Modules          | User-Defined Modules  |
| ---------------------- | ------------------------- | --------------------- |
| Implementation         | Rust                      | Veyra                 |
| File format            | Compiled into interpreter | `.veymod`             |
| Import keyword         | `mod`                     | `mod`                 |
| Selective imports      | Supported                 | Supported             |
| Star imports           | Supported                 | Supported             |
| Runtime interpretation | No                        | Yes                   |
| Module isolation       | Module-specific           | Isolated environments |

Both types of modules use the same import syntax.

## 12. Practice Exercises

### Exercise 1: Import a Built-in Module

Import `math` and calculate the square root of a number.

### Exercise 2: Selective Import

Import `get_env` from `system` and read an environment variable.

### Exercise 3: Create a Module

Create a file named `greetings.veymod`.

Export a function that accepts a name and returns a greeting.

### Exercise 4: Private Functions

Add a private helper function beginning with `_` to your module.

Call it from an exported function.

### Exercise 5: Module Search Paths

Create a `modules/` directory next to a Veyra script.

Place a `.veymod` file inside it and import that module.

### Exercise 6: VEYRA_PATH

Create a custom module directory and add it to `VEYRA_PATH`.

Verify that Veyra can import a module from that directory.

### Exercise 7: Module Errors

Attempt to import a nonexistent module and inspect the resulting error message.

## What's Next?

Continue learning Veyra:

- [Built-in Functions and Modules](built-ins.md)
- [Functions](functions.md)
- [Error Handling](error-handling.md)
- [Input and Output](io.md)
- [Variables and Types](variables.md)
