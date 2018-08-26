use io::*;
use std::collections::BTreeSet;

fn main() {
    let input = read_string();
    let mut sc = Scanner::new(&input);
    let n: usize = sc.next();
    let v: Vec<usize> = sc.next_vec(n);
    let mut l = 0;
    let mut r = 1000000001;
    while l + 1 < r {
        let m = (l + r) / 2;
        let mut s: Vec<i64> = v.iter().map(|&x| if x < m {-1} else {1}).collect();
        s.insert(0,0);
        for i in 1..n {
            s[i] += s[i-1];
        }
        let com = compress(&s);
        let mut bit = BIT::new(n+1);
        let mut count = 0;
        for x in com {
            count += bit.sum(x) as usize;
            bit.add(x, 1);
        }
        if count >= (n*(n+1)+2)/4 {
            l = m;
        } else {
            r = m;
        }
    }
    println!("{}",l);
}

fn compress(v: &Vec<i64>) -> Vec<usize> {
    let mut s: BTreeSet<i64> = BTreeSet::new();
    for x in v {
        s.insert(*x);
    }
    let w: Vec<i64> = s.into_iter().collect();
    let mut ret: Vec<usize> = vec![0;v.len()];
    for i in 0..v.len() {
        let mut l = 0;
        let mut r = w.len();
        while l + 1 < r {
            let m = (l + r) / 2;
            if w[m] > v[i] {
                r = m;
            } else {
                l = m;
            }
        }
        ret[i] = l;
    }
    ret
}

struct BIT {
    v: Vec<i64>,
    n: usize
}

impl BIT {
    fn new(n: usize) -> BIT {
        BIT{v: vec![0;n+1], n: n}
    }

    fn add(&mut self, i: usize, x: i64) {
        let mut id = i as i64 + 1;
        while id <= self.n as i64 {
            self.v[id as usize] += x;
            id += id & -id;
        }
    }

    fn sum(&self, i: usize) -> i64 {
        let mut id = i as i64 + 1;
        let mut ret = 0;
        while id > 0 {
            ret += self.v[id as usize];
            id &= id - 1;
        }
        ret
    }
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
