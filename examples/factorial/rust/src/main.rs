fn factorial_rec(n: i64) -> i64 {
    if n < 2 {
        1
    } else {
        n * factorial_rec(n - 1)
    }
}

pub fn main() {
	for i in 0..1_000_000 {
        let n = factorial_rec(i % 20);
		println!("{}", n);
	}
}