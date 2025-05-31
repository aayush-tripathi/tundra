public class MatMul {
    public static int bench_matmul(int n) {
        // 1) Build A and B
        int[][] a = new int[n][n];
        for (int i = 0; i < n; i++) {
            for (int j = 0; j < n; j++) {
                a[i][j] = i * n + j;
            }
        }

        int[][] b = new int[n][n];
        for (int i = 0; i < n; i++) {
            for (int j = 0; j < n; j++) {
                b[i][j] = j * n + i;
            }
        }

        // 2) Allocate C = zero matrix
        int[][] c = new int[n][n];
        for (int i = 0; i < n; i++) {
            for (int j = 0; j < n; j++) {
                c[i][j] = 0;
            }
        }

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
        System.out.println(c[0][0]);
        return c[0][0];
    }

    public static void main(String[] args) {
        int n = 200;
        bench_matmul(n);
    }
}
