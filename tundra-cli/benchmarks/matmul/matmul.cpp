// File: matmul/matmul.cpp
#include <iostream>
#include <vector>

int main() {
    const int n = 200;

    // 1) Build A and B
    std::vector<std::vector<int>> a(n, std::vector<int>(n));
    for (int i = 0; i < n; i++) {
        for (int j = 0; j < n; j++) {
            a[i][j] = i * n + j;
        }
    }

    std::vector<std::vector<int>> b(n, std::vector<int>(n));
    for (int i = 0; i < n; i++) {
        for (int j = 0; j < n; j++) {
            b[i][j] = j * n + i;
        }
    }

    // 2) Allocate C = zero matrix
    std::vector<std::vector<int>> c(n, std::vector<int>(n, 0));

    // 3) Compute C = A × B
    for (int i = 0; i < n; i++) {
        for (int j = 0; j < n; j++) {
            int sum = 0;
            for (int k = 0; k < n; k++) {
                sum += a[i][k] * b[k][j];
            }
            c[i][j] = sum;
        }
    }

    // 4) Print C[0][0]
    std::cout << c[0][0] << "\n";
    return 0;
}
