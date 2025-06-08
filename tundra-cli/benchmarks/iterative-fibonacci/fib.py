n = 4000000
a, b = 0, 1
for _ in range(n):
    temp=a
    a=(a+b)%(10**9+7)
    b=temp
print(a)