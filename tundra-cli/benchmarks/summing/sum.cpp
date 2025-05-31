// File: sum_squares/sum_squares.cpp
#include <iostream>

int main() {
    const int n = 3000000;
    long long total = 0LL;
    int i = 1;
    while (i <= n) {
        long long ii = static_cast<long long>(i) * static_cast<long long>(i);
        total += ii;
        i++;
    }
    std::cout << total << "\n";
    return 0;
}
