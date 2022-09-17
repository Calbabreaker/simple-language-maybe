# Simple Language Maybe

## Specification

### Syntax

Comments can be declared with `#` where `#` will comment out all the text until the end of the line. Code between `#*` and `*#` will be comments as well:

```slm
# This is comment
#*
    This is another comment
    This is another comment
*#
```

Variables are created using `=` or `:=` for constants:

```slm
variable = "hello"
gravity := 9.1
```

The seperator for each statement is a linebreak. Use `~` if multiple statement are need on one line with each side of the `~` needing to be a statement:

```slm
var1 = "yes" ~ var2 = "no"
```

Functions are called with arguments being seperated by spaces or a seperator:

```slm
print "Hello World" variable variable2
# Or:
print"Hello World"variable variable2
```

`()` can be used to execute code where the value of the last statement will be returned (or returned with `return`). This can be multiline:

```slm
print (read_file "hello")
print (
    user = "Hello"
    get_password(user) # or return get_password(user)
)
# Required if doing math like this:
print (1 + 2)
```

Define functions using `func` and set it as a variable:

```slm
say_hello = func arg1 agr2 (

)
```

### Style

Files should end in `.slm`.
