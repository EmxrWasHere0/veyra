<div align="center">
<img src="images/veyra-banner.png">
</div>

A small, dynamically-typed, interpreted programming language with easy-to-understand keywords and a unique **channel** mechanism for runtime value overrides.

```veyra
set port = 8080
port = 3000             # the constant doesn't change — the channel does

add connect(target)
    say target          # 8080  (constant value)
    say 0 ? target      # 3000  (channel value, carried with the argument)
end

connect(port)
```

---

## Table of Contents

- [Why Veyra?](why-veyra.md)
- [Features](features.md)
- [Installation](installation.md)
  - [Option 1: Run from Source (`cargo run`)](installation.md#option-1-run-from-source-cargo-run)
  - [Option 2: Install Globally (`cargo install`)](installation.md#option-2-install-globally-cargo-install)
- [Quick Start](quick-start.md)
- [Language Overview](language-overview.md)
  - [Values](language-overview.md#values)
  - [Variables and Constants](language-overview.md#variables-and-constants)
  - [Channels](language-overview.md#channels)
  - [I/O](language-overview.md#io)
  - [Operators](language-overview.md#operators)
  - [Control Flow](language-overview.md#control-flow)
  - [Lists](language-overview.md#lists)
  - [Functions](language-overview.md#functions)
  - [Built-ins](language-overview.md#built-ins)
  - [Error Handling](language-overview.md#error-handling)
  - [Modules](language-overview.md#modules)
- [VSCode Extension](vscode-extension.md)
- [Project Layout](project-layout.md)
- [Limitations](limitations.md)
- [Future Plans](future-plans.md)
- [License](license.md)
