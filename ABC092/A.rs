use std::io::*;
use std::cmp;

fn read<T: std::str::FromStr>() -> T {
    let stdin = stdin();
    let mut buf = String::new();
	stdin.lock().read_line(&mut buf);
	buf.trim().parse().ok().unwrap()
}

fn main() {
    let a = read::<usize>();
    let b = read::<usize>();
    let c = read::<usize>();
    let d = read::<usize>();
    println!("{}",cmp::min(a,b)+cmp::min(c,d));
}
