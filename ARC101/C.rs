use io::*;
use std::cmp;

fn main() {
    let input = read_string();
    let mut sc = Scanner::new(&input);
    let n: usize = sc.next();
    let mut k: usize = sc.next();
    let xs: Vec<i64> = sc.next_vec(n);
    let mut r: Vec<i64> = vec![0];
    let mut l: Vec<i64> = Vec::new();
    for i in 0..n {
        if xs[i] == 0 {
            k -= 1;
        } else if xs[i] > 0 {
            r.push(xs[i]);
        } else {
            l.push(-xs[i]);
        }
    }
    l.push(0);
    l.reverse();
    if k == 0 {
        println!("0");
        return;
    }
    let mut ans = i64::max_value();
    for i in cmp::max(0,k as i64 - r.len() as i64 + 1) as usize .. cmp::min(l.len(),k) {
        ans = cmp::min(ans, l[i] * 2 + r[k-i]);
    }
    for i in cmp::max(0,k as i64 - l.len() as i64 + 1) as usize .. cmp::min(r.len(),k) {
        ans = cmp::min(ans, r[i] * 2 + l[k-i]);
    }
    println!("{}",ans);
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
