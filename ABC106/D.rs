use io::*;

fn main() {
    let input = read_string();
    let mut sc = Scanner::new(&input);
    let n: usize = sc.next();
    let m: usize = sc.next();
    let q: usize = sc.next();
    let mut tab: Vec<Vec<i64>> = vec![vec![0;n+1];n+1];
    for i in 0..m {
        let l: usize = sc.next();
        let r: usize = sc.next();
        tab[l][n] += 1;
        tab[l][r-1] -= 1;
    }
    for i in 0..n {
        tab[n-i-1][n] += tab[n-i][n];
        tab[n][n-i-1] += tab[n][n-i];
    }
    for i in 0..n {
        let x = n - i - 1;
        for j in 0..n {
            let y = n - j - 1;
            tab[x][y] += tab[x+1][y] + tab[x][y+1] - tab[x+1][y+1];
        }
    }
    for i in 0..q {
        let p: usize = sc.next();
        let q: usize = sc.next();
        println!("{}", tab[p][q]);
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
