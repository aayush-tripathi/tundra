// Recursive Fibonacci in Go

package main

import (
    "fmt"
)

func fib(n int) int {
    if n < 2 {
        return n
    }
    return fib(n-1) + fib(n-2)
}

func main() {
    n := 30
    fmt.Println(fib(n))
}
