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
    let v = read_vec::<usize>();
    let mut c = 0;
    let mut count = 0;
    let mut ans = 0;
    for m in v {
        if m == c {
            count += 1;
        } else if count <= 1 {
            c = m;
            count = 1;
        } else {
            ans += count / 2;
            c = m;
            count = 1;
        }
    }
    if count > 1 {
        ans += count / 2;
    }
    println!("{}",ans);
}
