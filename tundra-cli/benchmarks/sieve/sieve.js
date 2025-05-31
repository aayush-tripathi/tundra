// File: sieve/sieve.js
// Sieve of Eratosthenes in JavaScript (Node.js)

function sieve(n) {
  const isComposite = new Array(n + 1).fill(false);
  let p = 2;
  while (p * p <= n) {
    if (!isComposite[p]) {
      for (let m = p * p; m <= n; m += p) {
        isComposite[m] = true;
      }
    }
    p++;
  }
  let count = 0;
  for (let i = 2; i <= n; i++) {
    if (!isComposite[i]) count++;
  }
  console.log(count);
}

const n = parseInt(process.argv[2]) || 200000;
sieve(n);
