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
    let v = read_vec::<usize>();
    let (n,k) = (v[0],v[1]);
    let mut dp = vec![[0,i64::min_value()];n+1];
    for i in 0..n {
        let x = read::<i64>();
        dp[i+1][0] = cmp::max(dp[i][0],dp[i][1]) + x;
        if i+1 >= k {
            dp[i+1][1] = cmp::max(dp[i][1],cmp::max(dp[i+1-k][0],dp[i+1-k][1]));
        }
    }
    println!("{}",cmp::max(dp[n][0],dp[n][1]));
}
