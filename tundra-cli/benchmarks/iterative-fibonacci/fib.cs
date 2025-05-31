using System;
class Fib01 {
  static void Main() {
    int n = 80;
    long a = 0, b = 1;
    for (int i=0; i<n; i++) {
      var tmp = a + b;
      a = b;
      b = tmp;
    }
    Console.WriteLine(a);
  }
}