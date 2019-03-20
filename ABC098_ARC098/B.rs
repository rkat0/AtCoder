use std::io::*;
use std::collections::HashSet;
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
    let s = read_vec_char();
    let mut ans = 0;
    let mut s1: HashSet<char> = HashSet::new();
    let mut s2: HashSet<char>;
    for i in 1..n {
        s1.insert(s[i-1]);
        s2 = s[i..n].iter().map(|&x| x).collect();
        let m = s1.intersection(&s2).collect::<HashSet<_>>().len();
        ans = cmp::max(ans,m);
    }
    println!("{}",ans);
}
