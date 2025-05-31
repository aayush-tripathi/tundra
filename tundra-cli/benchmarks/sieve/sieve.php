<?php
// File: sieve/sieve.php
// Sieve of Eratosthenes in PHP

function sieve($n) {
    $is_composite = array_fill(0, $n + 1, false);
    $p = 2;
    while ($p * $p <= $n) {
        if (!$is_composite[$p]) {
            for ($m = $p * $p; $m <= $n; $m += $p) {
                $is_composite[$m] = true;
            }
        }
        $p++;
    }
    $count = 0;
    for ($i = 2; $i <= $n; $i++) {
        if (!$is_composite[$i]) {
            $count++;
        }
    }
    echo $count . "\n";
}

$n=200000;
sieve($n);
