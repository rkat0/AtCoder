use io::*;

fn main() {
    let input = read_string();
    let mut sc = Scanner::new(&input);
    let mut n: i64 = sc.next();
    let mut na = n.abs();
    let mut d: usize;
    let mut s: i64;
    if n > 0 {
        d = 1;
        s = 1;
        na -= s;
        while na > 0 {
            s *= 4;
            d += 2;
            na -= s;
        }
    } else if n < 0 {
        d = 2;
        s = 2;
        na -= s;
        while na > 0 {
            s *= 4;
            d += 2;
            na -= s;
        }
    } else {
        println!("0");
        return;
    }
    for i in 0..d {
        let p = 2i64.pow((d - i - 1) as u32);
        if (d-i-1) % 2 == 0 && n > (p-1)/3 {
            n -= p;
            print!("1");
        } else if (d-i-1) % 2 == 1 && n < -(p-2)/3 {
            n += p;
            print!("1");
        } else {
            print!("0");
        }
    }
    print!("\n");
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
