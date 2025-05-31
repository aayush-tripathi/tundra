
public class sieve {
    public static int countPrimes(int n) {
        boolean[] isComposite = new boolean[n + 1];
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

    public static void main(String[] args) {
        int n = 200000;
        if (args.length > 0) {
            n = Integer.parseInt(args[0]);
        }
        System.out.println(countPrimes(n));
    }
}

