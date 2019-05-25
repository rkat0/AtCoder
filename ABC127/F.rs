use io::*;

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

use std::collections::{BTreeSet, BTreeMap};
use std::iter::FromIterator;

fn main() {
    let input = read_string();
    let mut sc = Scanner::new(&input);
    let q: usize = sc.next();
    let mut qs: Vec<(usize, i64, i64)> = Vec::new();
    let mut set: BTreeSet<i64> = BTreeSet::new();
    set.insert(i64::min_value());
    for i in 0..q {
        let t: usize = sc.next();
        if t == 1 {
            let a: i64 = sc.next();
            let b: i64 = sc.next();
            set.insert(a);
            qs.push((t, a, b));
        } else {
            qs.push((t,0,0));
        }
    }
    let map: BTreeMap<i64, usize> = BTreeMap::from_iter(set.iter().cloned().zip(0..));
    let av: Vec<i64> = set.iter().cloned().collect();
    let mut bitn = BIT::new(av.len());
    let mut bitv = BIT::new(av.len());
    let mut b: i64 = 0;
    let mut an = 0;
    for i in 0..q {
        if qs[i].0 == 1 {
            let ai = *map.get(&qs[i].1).unwrap();
            b += qs[i].2;
            bitn.add(ai, 1);
            bitv.add(ai, qs[i].1);
            an += 1;
        } else {
            let mut l = 0;
            let mut r = av.len();
            while l + 1 < r {
                let m = (l + r) / 2;
                if bitn.sum(m) < (an + 1) / 2 {
                    l = m;
                } else {
                    r = m;
                }
            }
            let x = av[r];
            let mut m = bitv.sum(av.len() - 1) - bitv.sum(r) - (bitn.sum(av.len() - 1) - bitn.sum(r) - bitn.sum(r - 1)) * x - bitv.sum(r - 1) + b;
            println!("{} {}", x, m);
        }
    }
}

struct BIT {
    v: Vec<i64>,
    n: usize
}

impl BIT {
    fn new(n: usize) -> BIT {
        BIT{v: vec![0; n + 1], n: n}
    }

    // add x to i th element
    fn add(&mut self, i: usize, x: i64) {
        let mut id = i as i64 + 1;
        while id <= self.n as i64 {
            self.v[id as usize] += x;
            id += id & -id;
        }
    }

    // sum up to i th element
    fn sum(&self, i: usize) -> i64 {
        let mut id = i + 1;
        let mut ret = 0;
        while id > 0 {
            ret += self.v[id];
            id &= id - 1;
        }
        ret
    }
}
