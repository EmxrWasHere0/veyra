## Features

### Language

- **Dynamic typing** with 7 value types: `number`, `text`, `bool`, `list`, `jsondata`, `function`, and `null`.
- **Readable, line-based syntax** with blocks terminated by `end`.
- **Variables and constants** declared using `add` and `set`.
- **Channels** — Runtime value overrides using the `?` operator without modifying the original constant.
- **Conditional assignment** with `?=`.
- **Compound assignment operators** — `+=`, `-=`, `*=`, `/=`, and `%=` for updating mutable variables or constant channels.
- **Constant-aware compound assignment** — Compound operations on `set` constants update their channel values while preserving the original constants.
- **Null coalescing** with `??`.
- **Short-circuit evaluation** — `and`, `or`, and `??` skip unnecessary right-hand expressions.
- **Syntax validation of skipped expressions** — Short-circuiting prevents evaluation without accepting invalid expression syntax.
- **Membership operators** — `in` and `not in`.
- **Multiple return values** and list destructuring, including rest elements.
- **String interpolation** — Embed variables and expressions using `f"Hello, {name}!"`.

### Functions

- **First-class functions** with lambdas, recursion, and nested definitions.
- **Default parameters**, including expressions that reference earlier parameters.
- **Variadic parameters** (`...args`).
- **Closed-scope functions** that do not implicitly capture surrounding local variables.
- **Implicit and explicit returns** using the last expression or `return`.
- **Callback support** for functional list operations and other higher-order functions.

### Control Flow

- **Conditional statements** using `if`, `else if`, and `else`.
- **Loops** using `repeat`, `while`, and `repeat item in collection`.
- **Labeled loop control** using `@label repeat`, `stop @label`, and `fwd @label`.
- **Error handling** using `try`, `catch`, and `throw`.
- **Inline testing** using `test` blocks and `assert`.

### Collections and Text

- **Lists** with nested indexing, negative indices, indexed assignment, and slicing.
- **List methods** — `append`, `pop`, `len`, `index`, `insert`, `remove`, `clear`, `sort`, and `reverse`.
- **Functional list methods** — `map`, `filter`, `reduce`, `any`, and `all`, using callbacks.
- **JSONData objects** with key-based access, nested assignment, and object methods.
- **JSONData methods** — `keys`, `values`, `get`, `has`, `remove`, `len`, `merge`, and `items`.
- **JSONData merging** — Combine objects using `merge()` or the `+` operator, with right-hand values taking precedence.
- **String operations** — Unicode-aware indexing, slicing, searching, splitting, joining, and replacement.
- **Multiline strings** using triple double quotes (`"""`) or triple single quotes (`'''`).
- **Interpolated strings** using `f"..."`, `f'...'`, `f"""..."""`, or `f'''...'''`.
- **Escaped interpolation braces** — Use `{{` and `}}` to include literal braces in interpolated strings.

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
- **Improved multiline REPL input** — Reliable handling of nested blocks, labeled loops, default-parameter functions, lambdas, multiline strings, lists, and JSONData.
- **REPL delimiter validation** — Mismatched closing delimiters are reported instead of leaving the REPL waiting for additional input.
- **Column-based runtime errors** — Error messages include source filenames, line numbers, column numbers, and source context.
- **Precise token highlighting** — Error markers identify problematic identifiers, operators, indexes, and other expression tokens when available.
- **Function and lambda tracebacks** — Show the call chain and preserve source locations when errors occur.
- **Automated regression tests** using Cargo.
- **CLI commands** for running scripts, executing tests, and displaying version information.
- **VSCode extension** with syntax highlighting, snippets, automatic indentation, code folding, and REPL integration.
- **VSCode interpolation highlighting** for embedded expressions and functional collection methods.

### Implementation

- **Written in Rust** with a dynamically typed interpreter.
- **Built-in modules compiled into the interpreter**.
- **Third-party Rust crates** used where appropriate for networking, JSON processing, and other functionality.
