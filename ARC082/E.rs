use io::*;
use std::collections::HashSet;
use std::cmp;

fn main() {
    let input = read_string();
    let mut sc = Scanner::new(&input);
    let n: i64 = sc.next();
    let ps: Vec<Vec<i64>> = sc.next_mat(2, n as usize);
    let mut lines: HashSet<(i64,i64,i64)> = HashSet::new();
    let M = 998244353;
    let mut ans = count_m(n, M);
    for i in 0..n as usize {
        for j in i+1..n as usize {
            let mut a = ps[j][1] - ps[i][1];
            let mut b = ps[i][0] - ps[j][0];
            let g = gcd(a,b);
            a /= g;
            b /= g;
            let mut c = a * ps[i][0] + b * ps[i][1];
            if a < 0 || (a == 0 && b < 0) {
                a *= -1;
                b *= -1;
                c *= -1;
            }
            if lines.contains(&(a,b,c)) {
                continue;
            }
            let mut cnt = 0;
            for k in 0..n as usize {
                if ps[k][0] * a + ps[k][1] * b == c {
                    cnt += 1;
                }
            }
            lines.insert((a,b,c));
            ans = (ans + M - count_m(cnt, M)) % M;
        }
    }
    println!("{}",ans);
}

fn count_m(n: i64, M: i64) -> i64 {
    if n < 3 {
        0
    } else {
        (pow_m(2,n,M) + M - n * (n+1) / 2 - 1) % M
    }
}

fn pow_m(a: i64, mut n: i64, M: i64) -> i64 {
    let mut r = a % M;
    let mut ret = 1;
    while n > 0 {
        if n % 2 == 1 {
            ret = ret * r % M;
            n -= 1;
        } else {
            r = r * r % M;
            n /= 2;
        }
    }
    ret
}

fn gcd(a: i64, b: i64) -> i64 {
    let aa = a.abs();
    let ba = b.abs();
    let mut x = cmp::min(aa,ba);
    let mut y = cmp::max(aa,ba);
    while x > 0 {
        let t = x;
        x = y % x;
        y = t;
    }
    y
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
