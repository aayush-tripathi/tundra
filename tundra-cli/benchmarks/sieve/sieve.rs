// File: sieve/sieve.rs
use std::env;

fn count_primes(n: usize) -> usize {
    let mut is_composite = vec![false; n + 1];
    let mut p = 2;
    while p * p <= n {
        if !is_composite[p] {
            let mut m = p * p;
            while m <= n {
                is_composite[m] = true;
                m += p;
            }
        }
        p += 1;
    }
    (2..=n).filter(|&i| !is_composite[i]).count()
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let n: usize = if args.len() > 1 {
        args[1].parse().unwrap_or(200000)
    } else {
        200000
    };
    println!("{}", count_primes(n));
}
