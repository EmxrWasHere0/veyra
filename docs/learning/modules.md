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
- Import packages installed through VPM.
- Understand manifest-based module resolution.

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

Output:

```text
12
```

### JSON

```veyra
mod json

add data = json.parse("{\"name\":\"Veyra\"}")

say data["name"]
```

Output:

```text
Veyra
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

Output:

```text
Veyra
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
4. The script directory's `veyra_packages/` folder (VPM packages).
5. The current working directory.
6. The working directory's `modules/` folder.
7. The working directory's `lib/` folder.
8. The working directory's `veyra_packages/` folder (VPM packages).
9. Additional directories configured through `VEYRA_PATH`.
10. User-level module directories.
11. System-level module directories.

When searching `veyra_packages/`, Veyra reads each subdirectory's `veyra.json` manifest to match the requested import name against the package's `module` field. This allows package directory names and import names to differ.

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

## 9. Package Modules (VPM)

Veyra supports reusable packages installed through the Veyra Package Manager (VPM).

Installed packages behave like user-defined `.veymod` modules but are discovered through their `veyra.json` manifests rather than their filenames.

### Installing a Package

Use VPM to install a local package:

```bash
python /path/to/vpm/python/vpm.py add ../example-package
```

This copies the package into the project's `veyra_packages/` directory:

```text
my-project/
├── main.vey
├── veyra.json
└── veyra_packages/
    └── example-package/
        ├── veyra.json
        └── example.veymod
```

### Package Manifests

Every VPM package contains a `veyra.json` manifest:

```json
{
  "name": "example-package",
  "module": "example",
  "version": "0.1.0",
  "entry": "example.veymod",
  "dependencies": {}
}
```

Important fields:

- `name`: The package identifier used by VPM for installation and removal.
- `module`: The name used when importing the package in Veyra code.
- `entry`: The `.veymod` file that serves as the package's entry point.

The `name` and `module` fields do not have to match.

For example, a package named `my-json-utils` could use `jsonutils` as its module name.

### Importing a Package

Use the `module` field value with the standard `mod` keyword:

```veyra
mod example

say example.PACKAGE_VERSION
say example.greet("World")
```

The interpreter searches `veyra_packages/` for a subdirectory whose `veyra.json` manifest contains `"module": "example"`.

It then loads the file specified in the `entry` field.

### Package Import Styles

Packages support the same import styles as regular modules.

Regular import:

```veyra
mod example

say example.greet("World")
```

Import all members:

```veyra
mod example *

say greet("World")
```

Selective import:

```veyra
mod example [greet, PACKAGE_VERSION]

say PACKAGE_VERSION
say greet("World")
```

### Package Module Isolation

Packages follow the same isolation rules as user-defined `.veymod` modules.

- Constants declared with `set` are exported.
- Functions declared with `add name(...)` are exported.
- Names beginning with `_` are private.
- Mutable variables are not automatically exported.
- Module functions retain their module context for callbacks and recursion.

### Package vs. Regular Module Resolution

When resolving `mod example`, Veyra first searches for a regular `example.veymod` file in the standard module directories.

If no matching file is found, Veyra searches `veyra_packages/` for a package whose manifest declares `"module": "example"`.

This means a regular `.veymod` file in the project directory takes precedence over an installed package with the same module name.

### Troubleshooting Package Imports

If `mod example` fails with "Module not found":

1. Verify the package is installed in `veyra_packages/`.
2. Check that the package contains a valid `veyra.json` manifest.
3. Confirm the manifest's `module` field matches the import name.
4. Confirm the manifest's `entry` field points to an existing `.veymod` file.
5. Reinstall dependencies if the package directory is missing:

```bash
python /path/to/vpm/python/vpm.py install
```

See the [VPM documentation](../../vpm/README.md) for complete package management instructions.

## 10. Importing User-Defined Module Members

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

## 11. Handling Module Errors

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
    say f"Operation failed: {err}"
end
```

Veyra provides source locations and function tracebacks for debugging runtime errors.

These diagnostic features were introduced in version 0.2.6.

## 12. Using String Interpolation with Modules

Veyra 0.2.7 supports string interpolation, which can simplify formatted output involving module values.

```veyra
mod system

say f"Operating system: {system.os}"
say f"Architecture: {system.arch}"
```

You can also interpolate the results of module function calls.

```veyra
mod math

add number = 81

say f"Square root of {number}: {math.sqrt(number)}"
```

Output:

```text
Square root of 81: 9
```

String interpolation uses the normal Veyra expression evaluator.

Module access rules remain unchanged:

- Constants are accessed without parentheses.
- Functions are called with parentheses.

## 13. Built-in vs. User-Defined Modules

| Feature                | Built-in Modules          | User-Defined Modules  | VPM Packages          |
| ---------------------- | ------------------------- | --------------------- | --------------------- |
| Implementation         | Rust                      | Veyra                 | Veyra                 |
| File format            | Compiled into interpreter | `.veymod`             | `.veymod`             |
| Discovery              | Name-based                | Filename-based        | Manifest-based        |
| Import keyword         | `mod`                     | `mod`                 | `mod`                 |
| Selective imports      | Supported                 | Supported             | Supported             |
| Star imports           | Supported                 | Supported             | Supported             |
| Runtime interpretation | No                        | Yes                   | Yes                   |
| Module isolation       | Module-specific           | Isolated environments | Isolated environments |
| Installation           | Built-in                  | Manual                | VPM                   |
| Configuration          | None                      | None                  | `veyra.json`          |

All three types use the same import syntax.

Both types of modules use the same import syntax.

## 14. Practice Exercises

### Exercise 1: Import a Built-in Module

Import `math` and calculate the square root of a number.

### Exercise 2: Selective Import

Import `get_env` from `system` and read an environment variable.

### Exercise 3: Create a Module

Create a file named `greetings.veymod`.

Export a function that accepts a name and returns a greeting using string interpolation.

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

### Exercise 8: Module Interpolation

Import `system` or `math`.

Use an interpolated string to display a module constant or function result.

### Exercise 9: Package Import

Install a local package using VPM.

Import the package in a Veyra program and call one of its exported functions.

Verify that the package's `module` field matches the name used in `mod`.

## What's Next?

Continue learning Veyra:

- [Built-in Functions and Modules](built-ins.md)
- [Functions](functions.md)
- [Error Handling](error-handling.md)
- [Input and Output](io.md)
- [Variables and Types](variables.md)
- [Veyra Package Manager](../../vpm/README.md)
