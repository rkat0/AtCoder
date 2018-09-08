use io::*;

fn main() {
    let input = read_string();
    let mut sc = Scanner::new(&input);
    let h: usize = sc.next();
    let w: usize = sc.next();
    let mut map: Vec<Vec<usize>> = sc.next_mat(w, h);
    let mut ans: Vec<[usize;4]> = Vec::new();
    for i in 0..h {
        for j in 0..w-1 {
            if map[i][j] % 2 == 0 {
                continue;
            }
            ans.push([i+1,j+1,i+1,j+2]);
            map[i][j+1] += 1;
        }
    }
    for i in 0..h-1 {
        if map[i][w-1] % 2 == 0 {
            continue;
        }
        ans.push([i+1,w,i+2,w]);
        map[i+1][w-1] += 1;
    }
    println!("{}",ans.len());
    for x in ans {
        println!("{} {} {} {}", x[0], x[1], x[2], x[3]);
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
