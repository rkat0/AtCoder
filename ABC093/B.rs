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
    let v = read_vec::<i64>();
    let (a,b,k) = (v[0],v[1],v[2]);
    for i in a..cmp::min(a+k,b+1) {
        println!("{}",i);
    }
    let m = cmp::max(a+k,b-k+1);
    if m <= b {
        for i in m..b+1 {
            println!("{}",i);
        }
    }
}
