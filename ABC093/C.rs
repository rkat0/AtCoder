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
    let (a,b,c) = (v[0],v[1],v[2]);
    let ea = a % 2;
    let eb = b % 2;
    let ec = c % 2;
    let s = ea + eb + ec;
    let max = cmp::max(cmp::max(a,b),c);
    let ds = 3 * max - a - b - c;
    let ans: usize;
    if s == 0 || s == 3 {
        ans = ds / 2;
    } else if s == 1 {
        if max%2 == 1 {
            ans = ds / 2;
        } else {
            ans = (ds + 3) / 2;
        }
    } else {
        if max%2 == 0 {
            ans = ds / 2;
        } else {
            ans = (ds + 3) / 2;
        }
    }
    println!("{}",ans);
}
