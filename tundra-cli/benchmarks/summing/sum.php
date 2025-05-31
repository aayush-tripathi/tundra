<?php
// File: sum_squares/sum_squares.php
// Compute sum of squares from 1..N (N = 2_000_000)

function sum_squares(int $n): int {
    $total = 0;
    $i = 1;
    while ($i <= $n) {
        $ii = $i * $i;
        $total += $ii;
        $i += 1;
    }
    return $total;
}

function main() {
    $n = 3000000;
    $result = sum_squares($n);
    echo $result . "\n";
}

main();
