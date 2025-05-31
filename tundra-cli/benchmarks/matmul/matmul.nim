# File: matmul/matmul.nim
# Triple-loop matrix multiplication, n = 200

proc benchMatMul(n: int): int =
  # 1) Build A and B as seq[seq[int]]
  var a: seq[seq[int]] = newSeq[seq[int]](n)
  for i in 0..<n:
    a[i] = newSeq[int](n)
    for j in 0..<n:
      a[i][j] = i * n + j

  var b: seq[seq[int]] = newSeq[seq[int]](n)
  for i in 0..<n:
    b[i] = newSeq[int](n)
    for j in 0..<n:
      b[i][j] = j * n + i

  # 2) Allocate C = zero matrix
  var c: seq[seq[int]] = newSeq[seq[int]](n)
  for i in 0..<n:
    c[i] = newSeq[int](n)
    for j in 0..<n:
      c[i][j] = 0

  # 3) Compute C = A × B
  for i in 0..<n:
    for j in 0..<n:
      var sum = 0
      for k in 0..<n:
        sum += a[i][k] * b[k][j]
      c[i][j] = sum

  # 4) Print C[0][0]
  echo c[0][0]
  return c[0][0]

when isMainModule:
  let n = 200
  discard benchMatMul(n)
