<?php
// File: matmul/matmul.php
// Triple‐loop matrix multiplication, n = 200

function bench_matmul(int $n): int {
    // 1) Build A and B
    $a = array_fill(0, $n, array_fill(0, $n, 0));
    for ($i = 0; $i < $n; $i++) {
        for ($j = 0; $j < $n; $j++) {
            $a[$i][$j] = $i * $n + $j;
        }
    }

    $b = array_fill(0, $n, array_fill(0, $n, 0));
    for ($i = 0; $i < $n; $i++) {
        for ($j = 0; $j < $n; $j++) {
            $b[$i][$j] = $j * $n + $i;
        }
    }

    // 2) Allocate C = zero matrix
    $c = array_fill(0, $n, array_fill(0, $n, 0));

    // 3) Compute C = A × B
    for ($i = 0; $i < $n; $i++) {
        for ($j = 0; $j < $n; $j++) {
            $sum = 0;
            for ($k = 0; $k < $n; $k++) {
                $sum += $a[$i][$k] * $b[$k][$j];
            }
            $c[$i][$j] = $sum;
        }
    }

    // 4) Print C[0][0]
    echo $c[0][0] . "\n";
    return $c[0][0];
}

function main() {
    $n = 200;
    bench_matmul($n);
}

main();
