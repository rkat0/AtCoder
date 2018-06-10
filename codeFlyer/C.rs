use std::io::*;

fn read<T: std::str::FromStr>() -> T {
    let stdin = stdin();
    let mut buf = String::new();
	stdin.lock().read_line(&mut buf);
	buf.trim().parse().ok().unwrap()
}

fn read_vec<T: std::str::FromStr>() -> Vec<T> {
	read::<String>().trim().split_whitespace()
        .map(|w| w.parse().ok().unwrap()).collect()
}

fn read_vec_char() -> Vec<char> {
    read::<String>().chars().collect()
}

fn read_mat<T: std::str::FromStr>(n: usize) -> Vec<Vec<T>> {
    (0..n).map(|_| read_vec()).collect()
}

fn read_mat_char(n: usize) -> Vec<Vec<char>> {
    (0..n).map(|_| read_vec_char()).collect()
}

fn main() {
    let v = read_vec::<usize>();
    let (n,d) = (v[0],v[1]);
    let x = read_vec::<usize>();
    let mut ans = 0;
    for i in 1..n-1 {
        let nl: usize;
        {
            let mut l = - 1;
            let mut r = i as i64;
            while l + 1 < r {
                let m = (l + r) / 2;
                if x[m as usize] + d >= x[i] {
                    r = m;
                } else {
                    l = m;
                }
            }
            nl = i - r as usize;
        }
        let nr: usize;
        {
            let mut l = i;
            let mut r = n;
            while l + 1 < r {
                let m = (l + r) / 2;
                if x[m] <= x[i] + d {
                    l = m;
                } else {
                    r = m;
                }
            }
            nr = l - i;
        }
        ans += nl * nr;
    }
    for i in 0..n-2 {
        let mut l = i;
        let mut r = n;
        while l + 1 < r {
            let m = (l + r) / 2;
            if x[m] <= x[i] + d {
                l = m;
            } else {
                r = m;
            }
        }
        let nr = l - i;
        if nr > 1 {
            ans -= nr * (nr - 1) / 2;
        }
    }
    println!("{}",ans);
}
