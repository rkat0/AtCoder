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
    let (mut a, mut b, n) = (v[0],v[1],v[2]);
    let s = read_vec_char();
    for c in s {
        if c == 'S' && a > 0 {
            a -= 1;
        } else if c == 'C' && b > 0 {
            b -= 1;
        } else if c == 'E' {
            if a < b {
                b -= 1;
            } else if a > 0 {
                a -= 1;
            }
        }
    }
    println!("{}",a);
    println!("{}",b);
}
