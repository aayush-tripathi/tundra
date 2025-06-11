# Control Flow

### `if` / `else`
- Tundra supports regular `if`/`else` statements like python.
- The condition must be wrapped around parenthesis 
```tundra
if (condition):
    # true branch
    print("Yes")
else:
    # false branch
    print("No")
```

### `while`
- Tundra supports regular `while` statements like python.
- The condition must be wrapped around parenthesis 
```tundra
var i = 0
while (i < 5):
    print(i)
    i = i + 1
```

### `for over range()`
- Tundra supports regular `for` statements like python.
```tundra
for i in range(3):
    print(i)
```

### `break` and `continue`
- - Tundra supports regular `break` and `continue` statements like python.
```tundra
for i in range(10):
    if (i == 3):
        break
    if (i % 2 == 0):
        continue
    print(i)
```

