use std::io::*;
use std::cmp;

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
    let s = read_vec_char();
    let l = s.len();
    let mut m0 = l;
    let mut m1 = l;
    for i in 0..l {
        if s[i] == '1' {
            m1 = cmp::min(m1,cmp::max(i,l-i-1));
        } else {
            m0 = cmp::min(m0,cmp::max(i,l-i-1));
        }
    }
    println!("{}",cmp::max(m0,m1));
}
