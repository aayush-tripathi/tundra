// File: sum_squares/sum_squares.cs
// Compute sum of squares from 1..N (N = 2_000_000)

using System;

class SumSquares {
    static long SumSquaresMethod(int n) {
        long total = 0L;
        int i = 1;
        while (i <= n) {
            long ii = (long)i * (long)i;
            total += ii;
            i++;
        }
        return total;
    }

    static void Main(string[] args) {
        int n = 3000000;
        long result = SumSquaresMethod(n);
        Console.WriteLine(result);
    }
}
