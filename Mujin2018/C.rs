use io::*;

fn main() {
    let input = read_string();
    let mut sc = Scanner::new(&input);
    let n: usize = sc.next();
    let m: usize = sc.next();
    let s = sc.next_mat_char(n);
    let mut p: Vec<Vec<[usize;4]>> = vec![vec![[0,0,0,0];m];n];
    for i in 0..n {
        for j in 1..m {
            if s[i][m-j-1] == '.' && s[i][m-j] == '.' {
                p[i][m-j-1][0] = p[i][m-j][0] + 1;
            }
            if s[i][j] == '.' && s[i][j-1] == '.' {
                p[i][j][1] = p[i][j-1][1] + 1;
            }
        }
    }
    for j in 0..m {
        for i in 1..n {
            if s[n-i-1][j] == '.' && s[n-i][j] == '.' {
                p[n-i-1][j][2] = p[n-i][j][2] + 1;
            }
            if s[i][j] == '.' && s[i-1][j] == '.' {
                p[i][j][3] = p[i-1][j][3] + 1;
            }
        }
    }
    let mut sp: Vec<Vec<[usize;4]>> = vec![vec![[0,0,0,0];m];n];
    for i in 0..n {
        for j in 1..m {
            if s[i][j] == '.' && s[i][j-1] == '.' {
                sp[i][j][3] = sp[i][j-1][3] + p[i][j-1][3];
            }
            if s[i][m-j-1] == '.' && s[i][m-j] == '.' {
                sp[i][m-j-1][2] = sp[i][m-j][2] + p[i][m-j][2];
            }
        }
    }
    for j in 0..m {
        for i in 1..n {
            if s[i][j] == '.' && s[i-1][j] == '.' {
                sp[i][j][0] = sp[i-1][j][0] + p[i-1][j][0];
            }
            if s[n-i-1][j] == '.' && s[n-i][j] == '.' {
                sp[n-i-1][j][1] = sp[n-i][j][1] + p[n-i][j][1];
            }
        }
    }
    let ans: usize = sp.iter().map(|v| v.iter().map(|x| x.iter().sum::<usize>()).sum::<usize>()).sum();
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
