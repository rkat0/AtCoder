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
    let s = read_vec_char();
    let t = read_vec_char();
    let n = s.len();
    for i in 0..n {
        let mut f = true;
        for j in 0..n {
            if s[j] != t[(i + j) % n] {
                f = false;
                break;
            }
        }
        if f {
            println!("Yes");
            return;
        }
    }
    println!("No");
}
