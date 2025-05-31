# File: sieve/sieve.nim
import sequtils, strutils, os

proc sieve(n: int): int =
  var isComposite = newSeq[bool](n + 1)
  for i in 0..n:
    isComposite[i] = false

  var p = 2
  while p * p <= n:
    if not isComposite[p]:
      var m = p * p
      while m <= n:
        isComposite[m] = true
        m += p
    p.inc()
  
  # Count primes:
  result = 0
  for i in 2..n:
    if not isComposite[i]:
      result.inc()

when isMainModule:
  var n = 200000
  if paramCount() > 0:
    n = parseInt(paramStr(1))
  echo sieve(n)
