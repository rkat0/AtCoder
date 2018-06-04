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

const D: usize = 998244353;

fn pow_mod(n: usize, mut m: usize) -> usize {
    let mut ret = 1;
    let mut k = n;
    while m > 0 {
        if m % 2 == 1 {
            ret = ret * k % D;
        }
        m /= 2;
        k = k * k % D;
    }
    ret
}

fn inv_mod(n: usize) -> usize {
    pow_mod(n,D-2)
}

fn main() {
    let v = read_vec::<usize>();
    let (n,a,b,k) = (v[0],v[1],v[2],v[3]);
    let mut ans = 0;
    let mut facts: Vec<usize> = vec![1;n+1];
    let mut factsi: Vec<usize> = vec![1;n+1];
    for i in 2..n+1 {
        facts[i] = facts[i-1] * i % D;
        factsi[i] = inv_mod(facts[i]);
    }
    for i in 0..cmp::min(k/a,n)+1 {
        let j = (k - a * i) / b;
        if (k - a * i) % b != 0 || j > n {
            continue;
        }
        ans = (ans
            + (facts[n] * factsi[i] % D * factsi[n-i] % D)
            * (facts[n] * factsi[j] % D * factsi[n-j] % D)) % D;
    }
    println!("{}",ans);
}
