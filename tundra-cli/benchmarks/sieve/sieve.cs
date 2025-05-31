// File: sieve/sieve.cs
using System;

class Sieve {
    static int CountPrimes(int n) {
        bool[] isComposite = new bool[n + 1];
        int p = 2;
        while (p * p <= n) {
            if (!isComposite[p]) {
                for (int m = p * p; m <= n; m += p) {
                    isComposite[m] = true;
                }
            }
            p++;
        }
        int count = 0;
        for (int i = 2; i <= n; i++) {
            if (!isComposite[i]) count++;
        }
        return count;
    }

    static void Main(string[] args) {
        int n = 200000;
        if (args.Length > 0) {
            int.TryParse(args[0], out n);
        }
        Console.WriteLine(CountPrimes(n));
    }
}
