mod io

# ---- Labeled loops ----
say "=== Labeled loops ==="

count = 0
@outer repeat 3
    @inner repeat 3
        count += 1
        if count == 5
            stop @outer
        end
        say "tur " + count
    end
end
say "stopped at: " + count

say

# Nested breakout with fwd @label
total = 0
@outer repeat 3
    @inner repeat 3
        total += 1
        if total % 2 == 0
            fwd @outer         # skip to next outer iteration
        end
        say "counted: " + total
    end
end

say

# Classic search pattern
found = -1
@search repeat 5
    i = 0
    @inner repeat while i < 5
        if i == 3
            found = i
            stop @search
        end
        i += 1
    end
end
say "found: " + found

# ---- Try / catch ----
say
say "=== Try/catch ==="

try
    x = 10 / 0
catch err
    say "caught: " + err
end

# Catch without variable
try
    y = undefined_variable
catch
    say "caught something"
end

# Nested try/catch
try
    try
        a = 1 / 0
    catch inner
        say "inner caught: " + inner
    end
catch outer
    say "outer caught: " + outer
end

# ---- ?? null-coalescing ----
say
say "=== Null coalescing ?? ==="

maybe_value = null
result = maybe_value ?? "fallback"
say result

got = "actual" ?? "fallback"
say got

# With functions
add find(x)
    if x > 0
        return "positive"
    end
    null
end

say find(5) ?? "not found"
say find(-1) ?? "not found"

# ---- Combined: try + ?? ----
say
say "=== Combined ==="

add safe_divide(a, b)
    try
        return a / b
    catch
        return null
    end
end

say safe_divide(10, 2) ?? "error"
say safe_divide(10, 0) ?? "error"

# ---- try inside loop with stop @label ----
say
say "=== Try inside labeled loop ==="

@main repeat 5
    try
        if true
            stop @main
        end
    catch
        say "caught"
    end
end
say "done"