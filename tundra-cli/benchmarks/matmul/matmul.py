# File: matmul/matmul.py
# Triple‐loop matrix multiplication, n = 200

def bench_matmul(n):
    # 1) Build A and B as lists of lists
    a = [[0] * n for _ in range(n)]
    for i in range(n):
        for j in range(n):
            a[i][j] = i * n + j

    b = [[0] * n for _ in range(n)]
    for i in range(n):
        for j in range(n):
            b[i][j] = j * n + i

    # 2) Allocate C = zero matrix
    c = [[0] * n for _ in range(n)]

    # 3) Compute C = A × B
    for i in range(n):
        for j in range(n):
            s = 0
            for k in range(n):
                s += a[i][k] * b[k][j]
            c[i][j] = s

    # 4) Print C[0][0]
    print(c[0][0])
    return c[0][0]

def main():
    n = 200
    bench_matmul(n)

if __name__ == "__main__":
    main()
