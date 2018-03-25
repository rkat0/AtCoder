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

fn main() {
    let n = read::<usize>();
    let v = read_vec::<usize>();
    let d = v[0];
    let mut ans = n + v[1];
    for i in 0..n {
        let a = read::<usize>();
        ans += (d-1)/a;
    }
    println!("{}",ans);
}
