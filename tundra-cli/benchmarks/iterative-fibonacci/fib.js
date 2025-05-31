const n = 80;
let a = 0, b = 1;
for (let i = 0; i < n; i++) {
  [a, b] = [b, a + b];
}
console.log(a);