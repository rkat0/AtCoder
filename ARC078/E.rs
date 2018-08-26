use io::*;
use std::io::Write;

fn main() {
    let mut l: Vec<usize> = vec![0;10];
    l[0] = 1;
    let mut r: Vec<usize> = vec![9;10];
    for d in 0..10 {
        let mut t = r.clone();
        while l[d] < r[d] {
            t[d] = (l[d] + r[d]) / 2;
            println!("? {}", to_num(&t));
            std::io::stdout().flush().unwrap();
            let res = read_line();
            if res == "Y" {
                r[d] = t[d];
            } else {
                l[d] = t[d] + 1;
            }
        }
    }
    let mut z = 9;
    while r[z] == 0 {
        z -= 1;
    }
    let m = r[z];
    if m == 1 {
        r[z] = 9;
        loop {
            println!("? {}", to_num(&r));
            std::io::stdout().flush().unwrap();
            let res = read_line();
            if res == "N" {
                break;
            }
            r.pop();
            if r.len() == z {
                r.push(m);
                println!("! {}",to_num(&r));
                return;
            }
        }
        r[z] = 1;
        r.push(0);
        println!("! {}",to_num(&r));
    } else {
        r[z] -= 1;
        loop {
            println!("? {}", to_num(&r));
            std::io::stdout().flush().unwrap();
            let res = read_line();
            if res == "Y" {
                break;
            }
            r.pop();
            if r.len() == z {
                r.push(m);
                println!("! {}",to_num(&r));
                return;
            }
        }
        r[z] += 1;
        println!("! {}",to_num(&r));
    }
}

fn to_num(v: &Vec<usize>) -> usize {
    v.iter().fold(0, |ac,x| ac*10 + x)
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
