# Channels

Here at Veyra, you can write to a constant. But will the constant change? Nope! It will create a parallel **channel** to write the new data on:

```text
set x = "I will always be constant!"
x = "Hi, this is x's channel!"

say x     # "I will always be constant!"
say 0?x   # "Hi, this is x's channel!"
```

See that `?` there? It's the operator to call the channel! It works like this:

```text
default ? channel_name
```

This will return the value of channel, and if there's no any channel, it will simply return the default value.

Channels also **follow arguments** when passed to a function:

```text
set port = 80
port = 8080

add show(c)
    say c           # 80    (the constant value)
    say 0 ? c       # 8080  (channel tag "port" carried via argument)
end

show(port)
```
