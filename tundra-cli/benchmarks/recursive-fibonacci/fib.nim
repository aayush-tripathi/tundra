# Recursive Fibonacci in Nim

proc fib(n: int): int =
  if n < 2:
    return n
  return fib(n - 1) + fib(n - 2)

when isMainModule:
  let n = 30;
  echo fib(n)
