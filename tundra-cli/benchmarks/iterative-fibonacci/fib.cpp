#include <iostream>
int main() {
    int n = 80;
    long a = 0, b = 1;
    for (int i = 0; i < n; i++) {
        auto tmp = a + b;
        a = b;
        b = tmp;
    }
    std::cout << a << std::endl;
    return 0;
}