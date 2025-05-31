public class fib {
  public static void main(String[] args) {
    int n = 80;
    long a = 0, b = 1;
    for (int i = 0; i < n; i++) {
      long tmp = a + b;
      a = b;
      b = tmp;
    }
    System.out.println(a);
  }
}