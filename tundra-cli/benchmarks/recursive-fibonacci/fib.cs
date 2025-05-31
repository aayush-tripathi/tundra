// Recursive Fibonacci in C# (Mono)

using System;

public class FibRecursive {
    public static long Fib(long n) {
        if (n < 2) return n;
        return Fib(n - 1) + Fib(n - 2);
    }
    public static void Main(string[] args) {
        long n = 30;
        Console.WriteLine(Fib(n));
    }
}
