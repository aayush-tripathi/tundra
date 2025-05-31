# File: sum_squares/sum_squares.py
# Compute sum of squares from 1..N (N = 2_000_000)

def sum_squares(n):
    total = 0
    i = 1
    while i <= n:
        ii = i * i
        total += ii
        i += 1
    return total

def main():
    n = 3_000_000
    result = sum_squares(n)
    print(result)

if __name__ == "__main__":
    main()
