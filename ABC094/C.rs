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
    let n = read::<usize>();
    let v = read_vec::<usize>();
    let mut vs: Vec<usize> = v.iter().cloned().collect();
    vs.sort();
    let m1 = vs[n/2-1];
    let m2 = vs[n/2];
    for x in v {
        if x <= m1 {
            println!("{}",m2);
        } else {
            println!("{}",m1);
        }
    }
}
