use io::*;

fn main() {
    let input = read_string();
    let mut sc = Scanner::new(&input);
    let k: usize = sc.next();
    if k < 2 {
        println!("2");
        println!("0 {}", k * 2);
    } else if k < 50 {
        println!("{}", k);
        print!("{}", k);
        for i in 1..k {
            print!(" {}", k);
        }
        print!("\n");
    } else {
        let mut v = vec![k / 50 + 49; 50];
        for i in 0..k % 50 {
            v[i] += 50 - k % 50 + 1;
        }
        for i in k % 50..50 {
            v[i] -= k % 50;
        }
        println!("50");
        print!("{}", v[0]);
        for i in 1..50 {
            print!(" {}", v[i]);
        }
        print!("\n");
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
