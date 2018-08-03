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
    let n = read::<usize>();
    let mut av = read_vec::<usize>();
    let mut bv = read_vec::<usize>();
    let mut cv = read_vec::<usize>();
    av.sort();
    bv.sort();
    cv.sort();
    let mut dv = vec![0;n];
    let mut c = 0;
    for i in 0..n {
        while c < n && bv[i] >= cv[c] {
            c += 1;
        }
        dv[i] = n - c;
    }
    for i in 0..n-1 {
        dv[n - i - 2] += dv[n - i - 1];
    }
    c = 0;
    let mut ans = 0;
    for i in 0..n {
        while c < n && av[i] >= bv[c] {
            c += 1;
        }
        if c == n {
            break;
        }
        ans += dv[c];
    }
    println!("{}",ans);
}
