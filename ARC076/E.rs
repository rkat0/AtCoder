use io::*;

fn main() {
    let input = read_string();
    let mut sc = Scanner::new(&input);
    let r: usize = sc.next();
    let c: usize = sc.next();
    let n: usize = sc.next();
    let v: Vec<Vec<usize>> = sc.next_mat(4, n);
    let mut ps: Vec<(usize,usize)> = Vec::new();
    for (i,xs) in v.iter().enumerate() {
        if (xs[0] == 0 || xs[0] == r || xs[1] == 0 || xs[1] == c) && (xs[2] == 0 || xs[2] == r || xs[3] == 0 || xs[3] == c) {
            ps.push((trans(xs[0],xs[1],r,c),i));
            ps.push((trans(xs[2],xs[3],r,c),i));
        }
    }
    ps.sort_by_key(|x| x.0);
    let mut st: Vec<usize> = Vec::new();
    for p in ps {
        if st.is_empty() || *st.last().unwrap() != p.1 {
            st.push(p.1);
        } else {
            st.pop();
        }
    }
    println!("{}", if st.is_empty() {"YES"} else {"NO"});
}

fn trans(x: usize, y: usize, r: usize, c: usize) -> usize {
    if y == 0 || x == r {
        x + y
    } else {
        2 * (r + c) - x - y
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
