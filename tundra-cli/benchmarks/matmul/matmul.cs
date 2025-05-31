// File: matmul/matmul.cs
// Triple‐loop matrix multiplication, n = 200

using System;

class MatrixMul {
    static int BenchMatMul(int n) {
        // 1) Build A and B
        int[,] a = new int[n, n];
        for (int i = 0; i < n; i++) {
            for (int j = 0; j < n; j++) {
                a[i, j] = i * n + j;
            }
        }

        int[,] b = new int[n, n];
        for (int i = 0; i < n; i++) {
            for (int j = 0; j < n; j++) {
                b[i, j] = j * n + i;
            }
        }

        // 2) Allocate C = zero matrix
        int[,] c = new int[n, n];
        for (int i = 0; i < n; i++) {
            for (int j = 0; j < n; j++) {
                c[i, j] = 0;
            }
        }

        // 3) Compute C = A × B
        for (int i = 0; i < n; i++) {
            for (int j = 0; j < n; j++) {
                int sum = 0;
                for (int k = 0; k < n; k++) {
                    sum += a[i, k] * b[k, j];
                }
                c[i, j] = sum;
            }
        }

        // 4) Print C[0][0]
        Console.WriteLine(c[0, 0]);
        return c[0, 0];
    }

    static void Main(string[] args) {
        int n = 200;
        BenchMatMul(n);
    }
}
