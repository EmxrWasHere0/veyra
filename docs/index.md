This document is just a copy of [README.md](https://github.com/EmxrWasHere0/veyra/blob/main/README.md) with betetr navigation. Educational documents will be out soon.

<div align="center">
<img src="https://raw.githubusercontent.com/EmxrWasHere0/veyra/refs/heads/main/media/veyra-banner.png">
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

```{toctree}
:maxdepth: 3

why-veyra
features
installation
quick-start
language-overview
vscode-extension
project-layout
limitations
future-plans
license
```
