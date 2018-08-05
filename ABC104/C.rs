use io::*;
use std::cmp;

fn main() {
    let input = read_string();
    let mut sc = Scanner::new(&input);
    let d: usize = sc.next();
    let g: usize = sc.next();
    let mut v: Vec<usize> = Vec::new();
    let mut comp: Vec<usize> = Vec::new();
    let mut total = 0;
    for i in 1..d+1 {
        let p: usize = sc.next();
        let c: usize = sc.next();
        v.push(p);
        total += p;
        comp.push(100 * i * p + c);
    }
    let mut ans = total;
    for i in 0..1<<d {
        let mut score = g as i64;
        let mut rest: Vec<usize> = Vec::new();
        let mut num = 0;
        for j in 0..d {
            if (i >> j) % 2 == 1 {
                score -= comp[j] as i64;
                num += v[j];
            } else {
                rest.push(j+1);
            }
        }
        for j in rest.into_iter().rev() {
            if score <= 0 {
                break;
            }
            let t = cmp::min((score - 1) as usize / (100 * j) + 1, v[j-1] - 1);
            score -= (100 * j * t) as i64;
            num += t;
        }
        if score <= 0 {
            ans = cmp::min(ans,num);
        }
    }
    println!("{}", ans);
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
