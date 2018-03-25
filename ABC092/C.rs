use std::io::*;
use std::cmp;

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
    let mut a = read_vec::<i32>();
    let mut total = 0;
    a.insert(0,0);
    a.push(0);
    for i in 1..n+2 {
        total += (a[i]-a[i-1]).abs();
    }
    for i in 1..n+1 {
        let x = a[i-1]-a[i];
        let y = a[i+1]-a[i];
        if x * y > 0 {
            println!("{}",total - cmp::min(x.abs(),y.abs()) * 2);
        } else {
            println!("{}",total);
        }
    }
}
