public class fib {
  public static long rec(long n) {
        if (n < 2) return n;
        return rec(n - 1) + rec(n - 2);
    }

    public static void main(String[] args) {
        long n =30;
        System.out.println(rec(n));
    }
}