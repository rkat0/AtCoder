use io::*;
use std::collections::BinaryHeap;

fn main() {
    let input = read_string();
    let mut sc = Scanner::new(&input);
    let n: usize = sc.next();
    let xs: Vec<i64> = sc.next_vec(n);
    let mut ys: Vec<i64> = sc.next_vec(n);
    let zs: Vec<i64> = sc.next_vec(n);
    let mut sumx = xs.iter().sum();
    let mut sumz = zs.iter().sum();
    let mut hx = BinaryHeap::new();
    let mut a = vec![sumx];
    for x in xs {
        hx.push(-x);
    }
    for y in ys.clone() {
        hx.push(-y);
        let out = hx.pop().unwrap();
        sumx += y + out;
        a.push(sumx);
    }
    let mut hz = BinaryHeap::new();
    let mut b = vec![sumz];
    for z in zs {
        hz.push(z);
    }
    ys.reverse();
    for y in ys {
        hz.push(y);
        let out = hz.pop().unwrap();
        sumz += y - out;
        b.push(sumz);
    }
    b.reverse();
    let ans: i64 = a.iter().zip(b.iter()).map(|x: (&i64,&i64)| *x.0 - *x.1).max().unwrap();
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
