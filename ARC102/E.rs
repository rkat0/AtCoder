use io::*;
use std::cmp;

fn main() {
    let input = read_string();
    let mut sc = Scanner::new(&input);
    let k: usize = sc.next();
    let n: usize = sc.next();
    let M = 998244353;
    let mut stair: Vec<usize> = (1..n+k).scan(1, |ac,x| {*ac = *ac * x % M; Some(*ac)}).collect();
    stair.insert(0,1);
    let stair_inv: Vec<usize> = stair.clone().iter().map(|&x| inv_m(x, M)).collect();
    for i in 2..2 * k + 1 {
        let mut ans = 0;
        let d = (i-1) / 2 - if i > k {i-k-1} else {0};
        if i % 2 == 1 {
            for j in 0..cmp::min(n,d)+1 {
                if k + j < 2 * d + 1 {
                    continue;
                }
                let h = stair[k+n-2*d-1] * stair_inv[k+j-2*d-1] % M * stair_inv[n-j] % M;
                ans += stair[d] * stair_inv[d-j] % M * stair_inv[j] % M * pow_m(2, j, M) % M * h % M;
                ans %= M;
            }
        } else {
            for j in 0..cmp::min(n,d)+1 {
                if k + j < 2 * d + 2 {
                    continue;
                }
                let h = stair[k+n-2*d-2] * stair_inv[k+j-2*d-2] % M * stair_inv[n-j] % M;
                ans += stair[d] * stair_inv[d-j] % M * stair_inv[j] % M * pow_m(2, j, M) % M * h % M;
                ans %= M;
            }
            for j in 0..cmp::min(n-1,d)+1 {
                if k + j < 2 * d + 2 {
                    continue;
                }
                let h = stair[k+n-2*d-3] * stair_inv[k+j-2*d-2] % M * stair_inv[n-j-1] % M;
                ans += stair[d] * stair_inv[d-j] % M * stair_inv[j] % M * pow_m(2, j, M) % M * h % M;
                ans %= M;
            }
        }
        println!("{}",ans);
    }
}

fn inv_m(n: usize, m: usize) -> usize {
    pow_m(n,m-2,m)
}

fn pow_m(n: usize, mut p: usize, m: usize) -> usize {
    let mut ret = 1;
    let mut r = n;
    while p > 0 {
        if p % 2 == 1 {
            ret = ret * r % m;
            p -= 1;
        } else {
            r = r * r % m;
            p /= 2;
        }
    }
    ret
}

pub mod io {
    use std;
    use std::str::FromStr;

    pub struct Scanner<'a> {
        iter: std::str::SplitWhitespace<'a>,
    }

    impl<'a> Scanner<'a> {
        pub fn new(s: &'a str) -> Scanner<'a> {
            Scanner {
                iter: s.split_whitespace(),
            }
        }

        pub fn next<T: FromStr>(&mut self) -> T {
            let s = self.iter.next().unwrap();
            if let Ok(v) = s.parse::<T>() {
                v
            } else {
                panic!("Parse error")
            }
        }

        pub fn next_vec_len<T: FromStr>(&mut self) -> Vec<T> {
            let n: usize = self.next();
            self.next_vec(n)
        }

        pub fn next_vec<T: FromStr>(&mut self, n: usize) -> Vec<T> {
            (0..n).map(|_| self.next()).collect()
        }

        pub fn next_mat<T: FromStr>(&mut self, w: usize, h: usize) -> Vec<Vec<T>> {
            (0..h).map(|_| self.next_vec(w)).collect()
        }

        pub fn next_vec_char(&mut self) -> Vec<char> {
            self.next::<String>().chars().collect()
        }

        pub fn next_mat_char(&mut self, n: usize) -> Vec<Vec<char>> {
            (0..n).map(|_| self.next_vec_char()).collect()
        }
    }

    pub fn read_string() -> String {
        use std::io::Read;

        let mut s = String::new();
        std::io::stdin().read_to_string(&mut s).unwrap();
        s
    }

    pub fn read_line() -> String {
        let mut s = String::new();
        std::io::stdin().read_line(&mut s).unwrap();
        s.trim_right().to_owned()
    }
}
