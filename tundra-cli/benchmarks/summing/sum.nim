# File: sum_squares/sum_squares.nim
# Compute sum of squares from 1..N (N = 2_000_000)

proc sumSquares(n: int): int64 =
  var total: int64 = 0
  var i: int = 1
  while i <= n:
    let ii = int64(i) * int64(i)
    total += ii
    i += 1
  result = total

when isMainModule:
  let n = 3000000
  let result = sumSquares(n)
  echo result
