// Recursive Fibonacci in C++
#include <iostream>
#include <cstdlib>

long fib(long n) {
    if (n < 2) return n;
    return fib(n - 1) + fib(n - 2);
}

int main(int argc, char** argv) {
    long n = 30;
    std::cout << fib(n) << std::endl;
    return 0;
}
