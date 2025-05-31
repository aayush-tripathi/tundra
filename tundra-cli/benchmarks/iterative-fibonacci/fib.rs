fn main() {
    let n = 80;
    let mut a = 0;
    let mut b = 1;
    for _ in 0..n {
        let tmp = a + b;
        a = b;
        b = tmp;
    }
    println!("{}", a);
}