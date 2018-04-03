use std::io::*;

fn read<T: std::str::FromStr>() -> T {
    let stdin = stdin();
    let mut buf = String::new();
	let _ = stdin.lock().read_line(&mut buf);
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
    let v = read_vec::<usize>();
    let k = v[1];
    let mut rs = read_vec::<usize>();
    rs.sort_by(|a,b| b.cmp(a));
    let mut d = 1f64;
    let mut rate = 0f64;
    for i in 0..k {
        d *= 2f64;
        rate += (rs[i] as f64)/d;
    }
    println!("{}",rate);
}
