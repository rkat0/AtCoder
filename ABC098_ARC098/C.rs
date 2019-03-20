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

fn main() {
    let n = read::<usize>();
    let s = read_vec_char();
    let mut accW: Vec<usize> = Vec::with_capacity(n);
    let mut accS: Vec<usize> = Vec::with_capacity(n);
    accW.push(0);
    accS.push(0);
    for i in 1..n {
        let wn = accW[i-1] + if s[i-1] == 'W' { 1 } else { 0 };
        accW.push(wn);
        let sn = accS[i-1] + if s[n-i] == 'E' { 1 } else { 0 };
        accS.push(sn);
    }
    let mut ans = n;
    for i in 0..n {
        ans = std::cmp::min(ans,accW[i]+accS[n-i-1]);
    }
    println!("{}",ans);
}
