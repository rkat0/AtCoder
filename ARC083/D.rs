use io::*;

fn main() {
    let input = read_string();
    let mut sc = Scanner::new(&input);
    let n: usize = sc.next();
    let d = sc.next_mat::<usize>(n, n);
    let mut total: usize = d.iter().map(|x| x.iter().sum::<usize>()).sum::<usize>() / 2;
    for i in 0..n {
        for j in i+1..n {
            for k in 0..n {
                if k == i || k == j {
                    continue;
                }
                if d[i][j] > d[i][k] + d[k][j] {
                    println!("-1");
                    return;
                } else if d[i][j] == d[i][k] + d[k][j] {
                    total -= d[i][j];
                    break;
                }
            }
        }
    }
    println!("{}",total);
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
