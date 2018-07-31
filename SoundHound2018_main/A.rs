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

fn read_vec_char() -> Vec<char> {
    read::<String>().chars().collect()
}

fn read_mat<T: std::str::FromStr>(n: usize) -> Vec<Vec<T>> {
    (0..n).map(|_| read_vec()).collect()
}

fn read_mat_char(n: usize) -> Vec<Vec<char>> {
    (0..n).map(|_| read_vec_char()).collect()
}

fn count(n: usize) -> usize {
    let mut m = n / 170;
    let mut k = 0;
    while m > 0 {
        k += 1;
        m /= 2;
    }
    let p = 2usize.pow(k);
    30 * (p - 1) + std::cmp::max(0, n as i64 - 140 * p as i64) as usize
}

fn main() {
    let v = read_vec::<usize>();
    let ans = count(v[1]) - count(v[0]);
    println!("{}",ans);
}
