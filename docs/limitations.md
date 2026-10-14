## Limitations

Veyra is intentionally small. The following are **not** supported (yet):

- Dictionary / map type
- List mutation helpers (`push`, `pop`, `slice`, `map`, `filter`)
- Extended standard library (`math`, `str`, `time`, `json`, `random`, etc.)
- String interpolation (`f"hello {name}"`)
- Pattern matching
- Real closures (functions capture no outer state — this is a deliberate design choice)
- User-defined modules in `.veymod` files _(planned)_

Contributions and ideas are welcome.
