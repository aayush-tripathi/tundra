# File: sieve/sieve.py
# Sieve of Eratosthenes in Python

import sys

def sieve(n):
    is_composite = [False] * (n + 1)
    p = 2
    while p * p <= n:
        if not is_composite[p]:
            for m in range(p * p, n + 1, p):
                is_composite[m] = True
        p += 1
    count = sum(1 for i in range(2, n + 1) if not is_composite[i])
    print(count)

sieve(200000)