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
    let n = read::<usize>();
    let max = usize::max_value();
    let mut v: Vec<usize> = vec![max;n+1];
    v[0] = 0;
    for i in 0..n {
        if v[i] + 1 < v[i+1] {
            v[i+1] = v[i] + 1;
        }
        let mut j = 6;
        while i + j <= n {
            v[i+j] = cmp::min(v[i+j],v[i]+1);
            j *= 6;
        }
        j = 9;
        while i + j <= n {
            v[i+j] = cmp::min(v[i+j],v[i]+1);
            j *= 9;
        }
    }
    println!("{}",v[n]);
}
