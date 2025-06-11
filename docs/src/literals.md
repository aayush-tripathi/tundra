# Literals

Tundra supports the following literal types:

| Type         | Example       | Notes                              |
|--------------|---------------|------------------------------------|
| Integer      | `123`         | 64-bit signed                      |
| Float        | `3.14`        | IEEE-754 double precision          |
| String       | `"hello\n"`   | Double quotes; `\n, \\", \\t` escapes |
| Char         | `'a'`         | Single quotes                      |
| Boolean      | `true`, `false` | Keywords                          |
| None/null    | `none`        | Represents absence of value        |
| Array        | `[1, 2, 3]`   | List literal; optional newlines inside |

```tundra
var x = 24
print(x)          # integer

var y = 12.21
print(y)          # float

y = "hi"
print(y)          # string 

print('z')        # char literal

print(true)       # boolean

print(none)       # none

var a =[1,2,3]    # array
print(a[0])

Arrays may span multiple lines

var a = [
    1,
    2,
    1111
]