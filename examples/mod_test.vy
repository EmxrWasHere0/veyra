# Simple import
mod system
say system.os
say system.arch
say system.version

# Function call
mod io
result = io.shell("echo hello")
say result[0]
say result[1]
say result

# Star import
mod base *
say os
say arch
say hostname()
say pid()

# Selective import
mod system [get_env, args]
home = get_env("HOME")
say home

# File operations
io.write("_test.txt", "hello veyra")
content = io.read("_test.txt")
say content
io.remove("_test.txt")
say io.exists("_test.txt")

# Directory operations
say io.cwd()
files = io.listdir(".")
say len(files)