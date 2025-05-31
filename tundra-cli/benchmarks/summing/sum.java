// File: sum_squares/SumSquares.java
// Compute sum of squares from 1..N (N = 2_000_000)

public class sum {
    public static long sum_squares(int n) {
        long total = 0L;
        int i = 1;
        while (i <= n) {
            long ii = (long)i * (long)i;
            total += ii;
            i++;
        }
        return total;
    }

    public static void main(String[] args) {
        int n = 3_000_000;
        long result = sum_squares(n);
        System.out.println(result);
    }
}
