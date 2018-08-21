use io::*;
use std::collections::*;
use std::cmp;

fn main() {
    let input = read_string();
    let mut sc = Scanner::new(&input);
    let a: usize = 100 * sc.next::<usize>();
    let b: usize = 100 * sc.next::<usize>();
    let c: usize = sc.next();
    let d: usize = sc.next();
    let e: usize = sc.next();
    let f: usize = sc.next();
    let mut water: BTreeSet<usize> = BTreeSet::new();
    let mut salt: BTreeSet<usize> = BTreeSet::new();
    let mut q: Vec<usize> = Vec::new();
    q.push(a);
    water.insert(a);
    if b <= f {
        q.push(b);
        water.insert(b);
    }
    loop {
        if q.is_empty() {
            break;
        }
        let n = q.pop().unwrap();
        let x = n + a;
        if x <= f && !water.contains(&x) {
            water.insert(x);
            q.push(x);
        }
        let x = n + b;
        if x <= f && !water.contains(&x) {
            water.insert(x);
            q.push(x);
        }
    }
    q.clear();
    q.push(0);
    salt.insert(0);
    loop {
        if q.is_empty() {
            break;
        }
        let n = q.pop().unwrap();
        let x = n + c;
        if x <= f && !salt.contains(&x) {
            salt.insert(x);
            q.push(x);
        }
        let x = n + d;
        if x <= f && !salt.contains(&x) {
            salt.insert(x);
            q.push(x);
        }
    }
    let mut max: f64 = 0.0;
    let mut mw = a;
    let mut ms = 0;
    let sv: Vec<usize> = salt.into_iter().collect();
    for w in water {
        let bound = cmp::min(w * e / 100, f - w);
        let s = bsearch_max(&sv,bound);
        let con = 100.0 * s as f64 / (s+w) as f64;
        if max < con {
            max = con;
            mw = w;
            ms = s;
        }
    }
    println!("{} {}",mw+ms,ms);
}

fn bsearch_max(v: &Vec<usize>, b: usize) -> usize {
    let mut l = 0;
    let mut r = v.len();
    while l + 1 < r {
        let m = (l + r) / 2;
        if v[m] <= b {
            l = m;
        } else {
            r = m;
        }
    }
    v[l]
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
