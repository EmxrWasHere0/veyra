# Running Veyra Programs

Veyra is an interpreted programming language written in Rust.

The interpreter executes Veyra source files directly, without requiring each program to be compiled into a standalone executable.

This guide explains how to run scripts, use the interactive REPL, execute tests, and work with Veyra's command-line interface.

Veyra 0.2.8 improves REPL stability, introduces compound assignment operators, adds short-circuit evaluation, and provides more precise runtime error locations.

## Requirements

To run Veyra programs, you need the Veyra interpreter.

If you want to build the interpreter from source, you also need Rust and Cargo.

For installation instructions, see [Installation](installation.md).

## 1. Running a Veyra File

Create a file named `hello.vey`:

```veyra
say "Hello, World!"

add greet(name)
    say f"Hello, {name}!"
end

greet("Veyra")
```

Run the file:

```bash
veyra hello.vey
```

Output:

```text
Hello, World!
Hello, Veyra!
```

Veyra executes the statements in the file in order.

## 2. Running from Source

If you are developing Veyra, you can run programs using Cargo.

Clone the repository:

```bash
git clone https://github.com/EmxrWasHere0/veyra.git
cd veyra
```

Run a program:

```bash
cargo run -- hello.vey
```

Or run a repository example:

```bash
cargo run -- examples/hello.vey
```

The `--` separates Cargo arguments from arguments passed to Veyra.

### Release Mode

For optimized execution:

```bash
cargo run --release -- hello.vey
```

Release builds generally run faster than development builds.

## 3. Building the Interpreter

Build an optimized Veyra executable:

```bash
cargo build --release
```

The executable is placed in:

| Platform | Executable                 |
| -------- | -------------------------- |
| Linux    | `target/release/veyra`     |
| macOS    | `target/release/veyra`     |
| Windows  | `target/release/veyra.exe` |

Run the executable directly on Linux or macOS:

```bash
./target/release/veyra hello.vey
```

On Windows PowerShell:

```powershell
.\target\release\veyra.exe hello.vey
```

## 4. Interactive REPL

Veyra introduced its interactive Read-Eval-Print Loop (REPL) in version 0.2.6.

**Veyra 0.2.8 improves multiline input detection, nested block handling, and expression completion.**

Start the REPL by running Veyra without arguments:

```bash
veyra
```

The interpreter displays an interactive prompt:

```text
>>>
```

You can enter statements directly:

```text
>>> add x = 10
>>> say x * 2
20
>>> say f"Double: {x * 2}"
Double: 20
```

### Persistent State

Variables, functions, and imports remain available throughout the REPL session.

```text
>>> add name = "Veyra"
>>> say name
Veyra
>>> name = "Veyra 0.2.8"
>>> say name
Veyra 0.2.8
```

### Multiline Blocks

The REPL supports multiline functions, loops, and other blocks.

```text
>>> add square(n)
...     return n * n
... end
>>> say square(5)
25
```

The `...` prompt indicates that Veyra is waiting for more input.

The `>>>` prompt returns when the input is complete.

### Nested Blocks

**Improved in Veyra 0.2.8**

The REPL handles nested control-flow blocks more reliably.

```text
>>> repeat 2
...     if true
...         say "Nested block"
...     end
... end
Nested block
Nested block
```

The REPL waits until all required `end` statements have been entered.

### Labeled Loops

The REPL supports labeled `repeat` loops.

```text
>>> @outer repeat 2
...     say "Hello"
... end
Hello
Hello
```

Loop labels can be used with `stop` and `fwd`.

For more information, see [Control Flow](control-flow.md).

### Functions with Default Parameters

The REPL recognizes function definitions containing default parameter values.

```text
>>> add greet(name = "World")
...     say f"Hello, {name}!"
... end
>>> greet()
Hello, World!
```

The `=` character inside a parameter list does not cause the REPL to treat the definition as a simple assignment.

### Lambda Expressions

Single-expression lambdas are recognized without requiring an `end` statement.

```text
>>> add double = func(x) => x * 2
>>> say double(5)
10
```

The REPL distinguishes lambda expressions from multiline function definitions.

### Multiline Strings

The REPL supports triple-quoted multiline strings.

```text
>>> add message = """
... Hello
... Veyra
... """
>>> say message

Hello
Veyra
```

The REPL continues accepting input until the closing triple quotes are entered.

Both `"""` and `'''` are supported.

### Multiline Lists

Lists can span multiple lines.

```text
>>> add numbers = [
...     10,
...     20,
...     30
... ]
>>> say numbers
[10, 20, 30]
```

The REPL waits for the closing `]`.

### Multiline JSONData

JSONData objects can also span multiple lines.

```text
>>> add person = {
...     "name": "Veyra",
...     "active": true
... }
>>> say person["name"]
Veyra
```

The REPL waits for the closing `}`.

### Try/Catch Blocks

The REPL supports multiline error handling.

```text
>>> try
...     throw "Example error"
... catch err
...     say f"Caught: {err}"
... end
```

The `try/catch` structure executes after its closing `end`.

### If/Else Blocks

Conditional blocks are also supported.

```text
>>> add score = 75
>>> if score >= 50
...     say "Passed"
... else
...     say "Failed"
... end
Passed
```

The REPL does not execute the incomplete conditional block when `else` is entered.

### Strings and Special Characters

Characters inside quoted strings do not affect REPL block detection.

```text
>>> add message = "This is # not a comment (end)"
>>> say message
This is # not a comment (end)
```

The REPL correctly distinguishes string contents from comments, delimiters, and block keywords.

### Invalid Delimiters

Veyra 0.2.8 improves handling of mismatched closing delimiters.

For example:

```veyra
say (10 + 20]
```

The REPL reports an error rather than waiting indefinitely for additional input.

### Compound Assignment in the REPL

**New in Veyra 0.2.8**

You can update mutable variables using compound assignment operators.

```text
>>> add counter = 10
>>> counter += 5
>>> say counter
15
>>> counter *= 2
>>> say counter
30
```

The supported operators are:

- `+=`
- `-=`
- `*=`
- `/=`
- `%=`

For constants declared with `set`, compound assignment updates the associated channel instead of modifying the original constant.

See [Operators](operators.md) and [Channels](channels.md).

### Short-Circuit Evaluation in the REPL

**New in Veyra 0.2.8**

The `and`, `or`, and `??` operators support short-circuit evaluation.

```text
>>> say false and (10 / 0)
false
>>> say true or (10 / 0)
true
>>> say 42 ?? (10 / 0)
42
```

The right operand is skipped when its value is unnecessary.

Skipped expressions must still be syntactically valid.

### Functional List Methods in the REPL

Veyra 0.2.7 introduced functional list methods.

You can experiment with them interactively:

```text
>>> add numbers = [1, 2, 3, 4, 5]
>>> say numbers.map(func(x) => x * 2)
[2, 4, 6, 8, 10]
>>> say numbers.filter(func(x) => x % 2 == 0)
[2, 4]
>>> say numbers.reduce(func(acc, x) => acc + x, 0)
15
```

### Leaving the REPL

Type:

```text
exit
```

Or:

```text
quit
```

You can also send an end-of-file signal using your terminal's standard keyboard shortcut.

## 5. Command-Line Options

Veyra provides several command-line options.

| Command                 | Description                     |
| ----------------------- | ------------------------------- |
| `veyra file.vey`        | Execute a Veyra source file     |
| `veyra`                 | Start the interactive REPL      |
| `veyra --help`          | Display usage information       |
| `veyra -h`              | Short form of `--help`          |
| `veyra --version`       | Display the interpreter version |
| `veyra -v`              | Short form of `--version`       |
| `veyra --test file.vey` | Execute Veyra test blocks       |
| `veyra -t file.vey`     | Short form of `--test`          |

### Checking the Version

```bash
veyra --version
```

The version command reports the interpreter's current package version.

For Veyra 0.2.8, the package version is defined in `Cargo.toml`.

The version is embedded at build time.

### Displaying Help

```bash
veyra --help
```

This displays the supported command-line syntax and options.

## 6. Running Veyra Tests

Veyra supports test blocks and assertions.

Example file `tests.vey`:

```veyra
test addition
    assert 2 + 2 == 4
end
```

Run the test file:

```bash
veyra --test tests.vey
```

You can also use:

```bash
veyra -t tests.vey
```

Test mode executes the file's Veyra tests and reports their results.

### Testing Language Features

You can use assertions to test string interpolation, functional list methods, JSONData, and short-circuit evaluation.

Example file `features.vey`:

```veyra
test interpolation
    add name = "Alex"

    assert f"Hello, {name}!" == "Hello, Alex!"
end

test functional_lists
    add numbers = [1, 2, 3, 4]

    assert numbers.map(func(x) => x * 2) == [2, 4, 6, 8]
    assert numbers.filter(func(x) => x % 2 == 0) == [2, 4]
    assert numbers.reduce(func(acc, x) => acc + x, 0) == 10
end

test jsondata_merge
    add left = {"name": "Alex", "age": 20}
    add right = {"age": 21}

    add merged = left.merge(right)

    assert merged["age"] == 21
end

test short_circuit
    assert (true or (10 / 0)) == true
    assert (false and (10 / 0)) == false
    assert (42 ?? (10 / 0)) == 42
end

test compound_assignment
    add score = 10

    score += 5
    assert score == 15

    score *= 2
    assert score == 30
end
```

Run:

```bash
veyra --test features.vey
```

### Interpreter Regression Tests

If you are developing the Veyra interpreter itself, run the Rust regression tests:

```bash
cargo test
```

These tests execute real Veyra programs and check their behavior.

This is different from using `veyra --test` to execute test blocks inside a Veyra source file.

## 7. Running Files from Different Directories

Veyra accepts relative and absolute file paths.

Relative path:

```bash
veyra examples/hello.vey
```

Absolute path on Linux or macOS:

```bash
veyra /home/user/projects/hello.vey
```

Absolute path on Windows:

```powershell
veyra "C:\Projects\Veyra\hello.vey"
```

Relative paths depend on the current working directory.

## 8. Command-Line Arguments

Veyra provides access to command-line arguments through the `system` module.

Example file `arguments.vey`:

```veyra
mod system

add arguments = system.args()

say arguments
```

Run it with arguments:

```bash
veyra arguments.vey first second
```

The `system.args()` function returns the script arguments exposed by the runtime.

The interpreter and script paths are excluded from the returned argument list.

### Using Arguments with String Interpolation

You can also display arguments using interpolated strings.

```veyra
mod system

add arguments = system.args()

say f"Arguments: {arguments}"
say f"Argument count: {len(arguments)}"
```

For example, when two arguments are provided:

```text
Arguments: ["first", "second"]
Argument count: 2
```

## 9. Running Programs in VSCode

The Veyra VSCode extension supports syntax highlighting, snippets, and REPL integration.

Version 0.2.7 introduced support for string interpolation, functional list methods, and JSONData enhancements.

Veyra 0.2.8 adds new compound assignment operators to the language.

To start the REPL:

1. Open the VSCode Command Palette.
2. Search for `Veyra: Start REPL`.
3. Select the command.

To run a script, use the integrated terminal:

```bash
veyra hello.vey
```

For installation and usage instructions, see [VSCode Extension](extension.md).

## 10. Runtime Errors

Veyra introduced source locations and function tracebacks in version 0.2.6.

**Veyra 0.2.8 adds column-based error locations.**

For example:

```veyra
say 10 / 0
```

An error may look like:

```text
Runtime Error: example.vey:1:8: Division by zero

  1 | say 10 / 0
    |        ^
```

The error includes:

- The source file.
- The line number.
- The column number.
- The relevant source line.
- A marker identifying the problematic token.

Errors inside functions can also include a traceback showing the function call chain.

For more information, see [Error Handling](error-handling.md).

## 11. Troubleshooting

### Cargo Is Not Recognized

Install Rust and Cargo using [rustup](https://rustup.rs/).

Make sure Cargo is available in your system's `PATH`.

### Veyra Is Not Recognized

If you have built Veyra from source, install the interpreter:

```bash
cargo install --path .
```

Make sure Cargo's binary directory is included in your `PATH`.

You can also run Veyra through Cargo:

```bash
cargo run -- hello.vey
```

### Source File Not Found

Check the filename, file extension, and current working directory.

Use an absolute path when necessary.

### Module Not Found

Check that the module exists in one of Veyra's module search directories.

You can also configure additional directories using `VEYRA_PATH`.

See [Modules](modules.md).

### Runtime Error

Read the source location and traceback.

In Veyra 0.2.8, check the highlighted token or operator to identify the failing expression.

Verify the operation, its arguments, and any required external resources.

## What's Next?

Continue learning Veyra:

- [Installation](installation.md)
- [Quick Start](quickstart.md)
- [Modules](modules.md)
- [Error Handling](error-handling.md)
- [VSCode Extension](extension.md)
