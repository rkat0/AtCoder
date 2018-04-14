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

fn read_mat<T: std::str::FromStr>(n: usize) -> Vec<Vec<T>> {
    (0..n).map(|_| read_vec()).collect()
}

fn main() {
    let _ = read::<usize>();
    let a = read_vec::<i64>();
    let x = a.iter().cloned().max().unwrap();
    let mut y = -1;
    let mut s = (y*2-x).abs();
    for b in a {
        if b == x {
            continue;
        }
        let st = (b*2-x).abs();
        if st < s {
            s = st;
            y = b;
        }
    }
    println!("{} {}",x,y);
}
