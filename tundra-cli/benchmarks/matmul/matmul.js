// File: matmul/matmul.js
// Triple‐loop matrix multiplication, n = 200

function bench_matmul(n) {
  // 1) Build A and B
  const a = new Array(n);
  for (let i = 0; i < n; i++) {
    a[i] = new Array(n);
    for (let j = 0; j < n; j++) {
      a[i][j] = i * n + j;
    }
  }

  const b = new Array(n);
  for (let i = 0; i < n; i++) {
    b[i] = new Array(n);
    for (let j = 0; j < n; j++) {
      b[i][j] = j * n + i;
    }
  }

  // 2) Allocate C = zero matrix
  const c = new Array(n);
  for (let i = 0; i < n; i++) {
    c[i] = new Array(n).fill(0);
  }

  // 3) Compute C = A × B
  for (let i = 0; i < n; i++) {
    for (let j = 0; j < n; j++) {
      let sum = 0;
      for (let k = 0; k < n; k++) {
        sum += a[i][k] * b[k][j];
      }
      c[i][j] = sum;
    }
  }

  // 4) Print C[0][0]
  console.log(c[0][0]);
  return c[0][0];
}

function main() {
  const n = 200;
  bench_matmul(n);
}

main();
