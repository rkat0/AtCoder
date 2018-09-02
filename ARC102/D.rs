use io::*;

fn main() {
    let input = read_string();
    let mut sc = Scanner::new(&input);
    let l: usize = sc.next();
    let mut m = l;
    let mut bits: Vec<usize> = Vec::new();
    let mut ones = 0;
    while m > 1 {
        bits.push(m % 2);
        if m % 2 == 1 {
            ones += 1;
        }
        m /= 2;
    }
    let len = bits.len();
    println!("{} {}", len+1, 2 * len + ones);
    let mut max = 2usize.pow(len as u32);
    for i in 0..len {
        println!("{} {} 0", i+1, i+2);
        println!("{} {} {}", i+1, i+2, 2usize.pow(i as u32));
    }
    for i in 0..len {
        if bits[len-i-1] == 1 {
            println!("{} {} {}", len - i, len+1, max);
            max += 2usize.pow((len-i-1) as u32);
        }
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
