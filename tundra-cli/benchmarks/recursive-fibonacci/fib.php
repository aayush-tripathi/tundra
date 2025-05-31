<?php
// Simple Recursive Fibonacci in PHP (No Memoization)

function fib($n) {
    if ($n < 2) {
        return $n;
    }
    return fib($n - 1) + fib($n - 2);
}

echo fib(30);
