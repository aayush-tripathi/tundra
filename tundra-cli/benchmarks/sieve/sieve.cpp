// File: sieve/sieve.cpp
#include <iostream>
#include <vector>
#include <cstdlib>

int main(int argc, char** argv) {
    int n = 200000;
    if (argc > 1) {
        n = std::atoi(argv[1]);
    }
    std::vector<bool> isComposite(n + 1, false);
    for (int p = 2; p * p <= n; ++p) {
        if (!isComposite[p]) {
            for (int m = p * p; m <= n; m += p) {
                isComposite[m] = true;
            }
        }
    }
    int count = 0;
    for (int i = 2; i <= n; ++i) {
        if (!isComposite[i]) count++;
    }
    std::cout << count << "\n";
    return 0;
}
