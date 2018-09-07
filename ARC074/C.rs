use io::*;
use std::cmp::{max,min};

fn main() {
    let input = read_string();
    let mut sc = Scanner::new(&input);
    let h: usize = sc.next();
    let w: usize = sc.next();
    if h * w % 3 == 0 {
        println!("0");
        return;
    }
    let mut ans = min(h,w);
    {
        let k = h / 3;
        let l = w / 2;
        if w % 2 == 0 {
            ans = l;
        } else {
            if h % 3 == 1 {
                ans = min(ans,min(max(k+l+1,2*k+1),max(k+w,2*k)));
            } else {
                ans = min(ans,min(max(k+l+1,2*k+1),max(k+w,2*k)));
            }
        }
    }
    {
        let k = w / 3;
        let l = h / 2;
        if h % 2 == 0 {
            ans = min(ans,l);
        } else {
            if w % 3 == 1 {
                ans = min(ans,min(max(k+l+1,2*k+1),max(k+w,2*k)));
            } else {
                ans = min(ans,min(max(k+l+1,2*k+1),max(k+w,2*k)));
            }
        }
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
