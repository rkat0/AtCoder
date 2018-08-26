use io::*;
use std::collections::HashSet;

fn main() {
    let input = read_string();
    let mut sc = Scanner::new(&input);
    let n: usize = sc.next();
    let mut e: Vec<Vec<usize>> = vec![Vec::new();n];
    for i in 0..n-1 {
        let a: usize = sc.next();
        let b: usize = sc.next();
        e[a-1].push(b-1);
        e[b-1].push(a-1);
    }
    let mut set: HashSet<usize> = HashSet::new();
    let mut qa: Vec<usize> = Vec::new();
    let mut qb: Vec<usize> = Vec::new();
    qa.push(0);
    qb.push(n-1);
    set.insert(0);
    set.insert(n-1);
    let mut an = 0;
    let mut bn = 0;
    loop {
        if qa.is_empty() && qb.is_empty() {
            break;
        }
        if !qa.is_empty() {
            let mut qt: Vec<usize> = Vec::new();
            for v in qa {
                for w in e[v].clone() {
                    if set.contains(&w) {
                        continue;
                    } else {
                        qt.push(w);
                        set.insert(w);
                        an += 1;
                    }
                }
            }
            qa = qt;
        }
        if !qb.is_empty() {
            let mut qt: Vec<usize> = Vec::new();
            for v in qb {
                for w in e[v].clone() {
                    if set.contains(&w) {
                        continue;
                    } else {
                        qt.push(w);
                        set.insert(w);
                        bn += 1;
                    }
                }
            }
            qb = qt;
        }
    }
    if an > bn {
        println!("Fennec");
    } else {
        println!("Snuke");
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
