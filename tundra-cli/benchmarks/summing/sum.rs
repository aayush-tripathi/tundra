// File: sum_squares/sum_squares.rs
fn sum_squares(n: i32) -> i64 {
    let mut total: i64 = 0;
    let mut i: i32 = 1;
    while i <= n {
        let ii = (i as i64) * (i as i64);
        total += ii;
        i += 1;
    }
    total
}

fn main() {
    let n: i32 = 3_000_000;
    let result = sum_squares(n);
    println!("{}", result);
}
