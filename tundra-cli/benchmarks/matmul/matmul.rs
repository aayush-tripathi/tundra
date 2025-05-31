// File: matmul/matmul.rs
fn bench_matmul(n: usize) -> usize {
    // 1) Build A and B
    let mut a: Vec<Vec<usize>> = vec![vec![0; n]; n];
    for i in 0..n {
        for j in 0..n {
            a[i][j] = i * n + j;
        }
    }

    let mut b: Vec<Vec<usize>> = vec![vec![0; n]; n];
    for i in 0..n {
        for j in 0..n {
            b[i][j] = j * n + i;
        }
    }

    // 2) Allocate C = zero matrix
    let mut c: Vec<Vec<usize>> = vec![vec![0; n]; n];

    // 3) Compute C = A × B
    for i in 0..n {
        for j in 0..n {
            let mut sum = 0;
            for k in 0..n {
                sum += a[i][k] * b[k][j];
            }
            c[i][j] = sum;
        }
    }

    // 4) Print C[0][0]
    println!("{}", c[0][0]);
    c[0][0]
}

fn main() {
    let n = 200;
    bench_matmul(n);
}
