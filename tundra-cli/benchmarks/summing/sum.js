// File: sum_squares/sum_squares.js
// Compute sum of squares from 1..N (N = 2_000_000)

function sum_squares(n) {
  let total = 0;
  let i = 1;
  while (i <= n) {
    const ii = i * i;
    total += ii;
    i += 1;
  }
  return total;
}

function main() {
  const n = 3000000;
  const result = sum_squares(n);
  console.log(result);
}

main();
