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

Table of contents are given in the Naviagtion tab.

```{toctree}
:maxdepth: 3
:caption: Documentation

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
