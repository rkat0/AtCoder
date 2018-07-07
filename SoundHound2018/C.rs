use std::io::*;
use std::cmp::{min,max};

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

fn main() {
    let v = read_vec::<usize>();
    let (n,m,d) = (v[0],v[1],v[2]);
    let mut ans: f64 = 0.0;
    if d == 0 {
        ans = (m-1) as f64 / n as f64;
    } else {
        ans = ((n-d) * 2 * (m-1)) as f64 / (n*n) as f64;
    }
    println!("{}",ans);
}
