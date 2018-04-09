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
    let n = read::<usize>();
    let mut dp: Vec<usize> = vec![n+1;n+1];
    for i in 0..n {
        let c = read::<usize>();
        let j = bsearch_r(&dp,i+1,c);
        dp[j] = c;
    }
    let ans = n - bsearch_r(&dp,n+1,n+1);
    println!("{}",ans);
}

fn bsearch_r(v: &Vec<usize>, n: usize, x: usize) -> usize {
    let mut l = 0;
    let mut r = n;
    while l < r {
        let m = (l + r) / 2;
        if v[m] >= x {
            r = m;
        } else {
            l = m + 1;
        }
    }
    r
}
