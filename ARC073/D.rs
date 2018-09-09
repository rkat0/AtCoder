use io::*;
use std::collections::BinaryHeap;
use std::cmp;

fn main() {
    let input = read_string();
    let mut sc = Scanner::new(&input);
    let n: usize = sc.next();
    let w: usize = sc.next();
    let bs: Vec<Vec<usize>> = sc.next_mat(2, n);
    let mut hs: Vec<BinaryHeap<usize>> = vec![BinaryHeap::new();4];
    let w0 = bs[0][0];
    for b in bs {
        hs[b[0]-w0].push(b[1]);
    }
    let mut its: Vec<Vec<usize>> = hs.iter().map(|h| h.clone().into_sorted_vec()).collect();
    for i in 0..4 {
        its[i].push(0);
        its[i].reverse();
        its[i] = its[i].iter().scan(0, |ac,&x| {*ac += x; Some(*ac)}).collect();
    }
    let mut ans = 0;
    for i in 0..cmp::min(w / w0 + 1, its[0].len()) {
        let wi = w - i * w0;
        for j in 0..cmp::min(wi / (w0+1) + 1, its[1].len()) {
            let wj = wi - j * (w0+1);
            for k in 0..cmp::min(wj / (w0+2) + 1, its[2].len()) {
                let wk = wj - k * (w0+2);
                let l = cmp::min(wk / (w0+3), its[3].len()-1);
                ans = cmp::max(ans,its[0][i]+its[1][j]+its[2][k]+its[3][l]);
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
