// File: matmul/matmul.go
package main

import "fmt"

func benchMatmul(n int) int {
    // 1) Build A and B
    a := make([][]int, n)
    for i := 0; i < n; i++ {
        a[i] = make([]int, n)
        for j := 0; j < n; j++ {
            a[i][j] = i*n + j
        }
    }

    b := make([][]int, n)
    for i := 0; i < n; i++ {
        b[i] = make([]int, n)
        for j := 0; j < n; j++ {
            b[i][j] = j*n + i
        }
    }

    // 2) Allocate C = zero matrix
    c := make([][]int, n)
    for i := 0; i < n; i++ {
        c[i] = make([]int, n)
        for j := 0; j < n; j++ {
            c[i][j] = 0
        }
    }

    // 3) Compute C = A × B
    for i := 0; i < n; i++ {
        for j := 0; j < n; j++ {
            sum := 0
            for k := 0; k < n; k++ {
                sum += a[i][k] * b[k][j]
            }
            c[i][j] = sum
        }
    }

    // 4) Print C[0][0]
    fmt.Println(c[0][0])
    return c[0][0]
}

func main() {
    n := 200
    benchMatmul(n)
}
