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

fn read_mat<T: std::str::FromStr>(n: usize) -> Vec<Vec<T>> {
    (0..n).map(|_| read_vec()).collect()
}

fn main() {
    let v = read_vec::<usize>();
    let a = read_vec::<usize>();
    let mut c1 = 0;
    let mut c2 = 0;
    for b in a {
        if b < v[2] {
            c1 += 1;
        } else {
            c2 += 1;
        }
    }
    let ans: usize = cmp::min(c1,c2);
    println!("{}",ans);
}
