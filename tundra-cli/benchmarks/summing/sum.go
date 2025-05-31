// File: sum_squares/sum_squares.go
package main

import "fmt"

func sumSquares(n int) int64 {
    var total int64 = 0
    i := 1
    for i <= n {
        ii := int64(i) * int64(i)
        total += ii
        i++
    }
    return total
}

func main() {
    n := 3000000
    result := sumSquares(n)
    fmt.Println(result)
}
