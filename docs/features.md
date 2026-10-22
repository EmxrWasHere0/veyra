## Features

### Language

- **Dynamic typing** with 7 value types: `number`, `text`, `bool`, `list`, `jsondata`, `function`, and `null`.
- **Readable, line-based syntax** with blocks terminated by `end`.
- **Variables and constants** declared using `add` and `set`.
- **Channels** — Runtime value overrides using the `?` operator without modifying the original constant.
- **Conditional assignment** with `?=`.
- **Null coalescing** with `??`.
- **Membership operators** — `in` and `not in`.
- **Multiple return values** and list destructuring, including rest elements.

### Functions

- **First-class functions** with lambdas, recursion, and nested definitions.
- **Default parameters**, including expressions that reference earlier parameters.
- **Variadic parameters** (`...args`).
- **Closed-scope functions** that do not implicitly capture surrounding local variables.
- **Implicit and explicit returns** using the last expression or `return`.

### Control Flow

- **Conditional statements** using `if`, `else if`, and `else`.
- **Loops** using `repeat`, `while`, and `repeat item in collection`.
- **Labeled loop control** using `@label repeat`, `stop @label`, and `fwd @label`.
- **Error handling** using `try`, `catch`, and `throw`.
- **Inline testing** using `test` blocks and `assert`.

### Collections and Text

- **Lists** with nested indexing, negative indices, indexed assignment, and slicing.
- **List methods** — `append`, `pop`, `len`, `index`, `insert`, `remove`, `clear`, `sort`, and `reverse`.
- **JSON-like objects (`jsondata`)** with key-based access, nested assignment, and object methods.
- **Object methods** — `keys`, `values`, `get`, `has`, `remove`, and `len`.
- **String operations** — Unicode-aware indexing, slicing, searching, splitting, joining, and replacement.
- **Multiline strings** using triple double quotes (`"""`) or triple single quotes (`'''`).

### Modules

- **Built-in modules** implemented in Rust:
  - `system`, `io`, `base`
  - `json`, `http`, `websocket`
  - `math`, `time`, `fs`, `random`
- **User-defined modules** using `.veymod` files.
- **Flexible imports** — Import an entire module, all exported members, or selected members.
- **Module search paths** — Project, user, and system module directories, with additional paths through `VEYRA_PATH`.

### Developer Experience

- **Interactive REPL** with persistent variables, functions, and imports.
- **Multiline REPL input** for functions, loops, and other blocks.
- **Detailed runtime errors** with source filenames, line numbers, and source context.
- **Function tracebacks** showing the call chain when an error occurs.
- **Automated regression tests** using Cargo.
- **CLI commands** for running scripts, executing tests, and displaying version information.
- **VSCode extension** with syntax highlighting, snippets, automatic indentation, code folding, and REPL integration.

### Implementation

- **Written in Rust** with a dynamically typed interpreter.
- **Built-in modules compiled into the interpreter**.
- **Third-party Rust crates** used where appropriate for networking, JSON processing, and other functionality.
