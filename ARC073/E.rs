use io::*;
use std::cmp;
use std::collections::BinaryHeap;

fn main() {
    let input = read_string();
    let mut sc = Scanner::new(&input);
    let n: usize = sc.next();
    let mut v: Vec<Vec<usize>> = sc.next_mat(2, n);
    for i in 0..n {
        if v[i][0] > v[i][1] {
            let t = v[i][0];
            v[i][0] = v[i][1];
            v[i][1] = t;
        }
    }
    let mut mini = 0;
    let mut maxi = 0;
    for i in 1..n {
        if v[mini][0] > v[i][0] {
            mini = i;
        }
        if v[maxi][1] < v[i][1] {
            maxi = i;
        }
    }
    let min1 = v[mini][0];
    let min2 = v.iter().min_by_key(|x| x[1]).unwrap()[1];
    let max1 = v.iter().max_by_key(|x| x[0]).unwrap()[0];
    let max2 = v[maxi][1];
    let mut ans = (max1 - min1) * (max2 - min2);

    if maxi != mini {
        let mut bh: BinaryHeap<(usize,usize)> = BinaryHeap::new();
        for i in 0..n {
            bh.push((v[i][1],v[i][0]));
        }
        loop {
            let (a,b) = bh.pop().unwrap();
            if a > b && b != min1 {
                bh.push((b,a));
            } else {
                bh.push((a,b));
                break;
            }
        }
        let bv = bh.into_vec();
        let r = bv.iter().map(|x| x.0).max().unwrap() - bv.iter().map(|x| x.0).min().unwrap();
        ans = cmp::min(ans, (max2 - min1) * r);
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
