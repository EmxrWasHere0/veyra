# I/O

Veyra has a wide support for input and output (I/O) operations. A basic example can be the `get` command:

```text
get "Your name: "        # prompt + read line
say "Hi, " + _input      # input is always stored in _input (as text)
x = _input               # you can make the input a variable's value.
                         # your input is stored in `_input` until another `get` call happens.
```

You can convert the input from `text` to `number` easily:

```text
get "Your age: "
age = num(_input)
say type(age)        # "number"
```
