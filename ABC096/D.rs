use std::io::*;

fn read<T: std::str::FromStr>() -> T {
    let stdin = stdin();
    let mut buf = String::new();
	stdin.lock().read_line(&mut buf);
	buf.trim().parse().ok().unwrap()
}

fn read_vec<T: std::str::FromStr>() -> Vec<T> {
	read::<String>().trim().split_whitespace()
        .map(|w| w.parse().ok().unwrap()).collect()
}

fn read_mat<T: std::str::FromStr>(n: usize) -> Vec<Vec<T>> {
    (0..n).map(|_| read_vec()).collect()
}

fn is_prime_1(n: usize) -> bool {
    if n % 2 == 0 {
        return false;
    }
    let mut k = 3;
    while k * k <= n {
        if n % k == 0 {
            return false;
        }
        k += 2;
    }
    return true;
}

fn main() {
    let n = read::<usize>();
    let mut a = 11;
    let mut i = 0;
    while i < n {
        if is_prime_1(a) {
            println!("{}",a);
            i += 1;
        }
        a += 10;
    }
}
