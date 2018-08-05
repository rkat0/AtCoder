use io::*;

fn main() {
    let input = read_string();
    let mut sc = Scanner::new(&input);
    let s = sc.next_vec_char();
    let n = s.len();
    let M = 1000000007;
    let mut va: Vec<usize> = vec![0;n+1];
    let mut vb: Vec<usize> = vec![0;n+1];
    let mut vc: Vec<usize> = vec![0;n+1];
    let mut tot: Vec<usize> = vec![1;n+1];
    for i in 0..n {
        if s[i] == 'A' {
            tot[i+1] = tot[i];
            va[i+1] = (va[i] + tot[i]) % M;
            vb[i+1] = vb[i];
            vc[i+1] = vc[i];
        } else if s[i] == 'B' {
            tot[i+1] = tot[i];
            va[i+1] = va[i];
            vb[i+1] = (vb[i] + va[i]) % M;
            vc[i+1] = vc[i];
        } else if s[i] == 'C' {
            tot[i+1] = tot[i];
            va[i+1] = va[i];
            vb[i+1] = vb[i];
            vc[i+1] = (vc[i] + vb[i]) % M;
        } else if s[i] == '?' {
            tot[i+1] = 3 * tot[i] % M;
            va[i+1] = (3 * va[i] + tot[i]) % M;
            vb[i+1] = (3 * vb[i] + va[i]) % M;
            vc[i+1] = (3 * vc[i] + vb[i]) % M;
        }
    }
    let ans = vc[n];
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
