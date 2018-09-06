use io::*;
use std::cmp;

fn main() {
    let input = read_string();
    let mut sc = Scanner::new(&input);
    let n: usize = sc.next();
    let a: i64 = sc.next();
    let b: i64 = sc.next();
    let v: Vec<i64> = sc.next_vec(n);
    let c = a - b;
    let mut l = 0;
    let mut r = 10i64.pow(10);
    while l + 1 < r {
        let m = (l + r) / 2;
        let t: i64 = v.iter().map(|x| (cmp::max(0,x - m * b) + c - 1) / c).sum();
        if t > m {
            l = m;
        } else {
            r = m;
        }
    }
    println!("{}",r);
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
