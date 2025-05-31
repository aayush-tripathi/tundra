// File: sieve/sieve.go
package main

import (
    "fmt"
    "os"
    "strconv"
)

func sieve(n int) int {
    isComposite := make([]bool, n+1)
    for p := 2; p*p <= n; p++ {
        if !isComposite[p] {
            for m := p * p; m <= n; m += p {
                isComposite[m] = true
            }
        }
    }
    count := 0
    for i := 2; i <= n; i++ {
        if !isComposite[i] {
            count++
        }
    }
    return count
}

func main() {
    n := 200000
    if len(os.Args) > 1 {
        if v, err := strconv.Atoi(os.Args[1]); err == nil {
            n = v
        }
    }
    fmt.Println(sieve(n))
}
