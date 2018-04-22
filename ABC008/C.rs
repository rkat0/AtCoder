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
    let mut ans = n as f64;
    let mut v = Vec::new();
    for i in 0..n {
        let c = read::<usize>();
        v.push(c);
    }
    for i in 0..n {
        let divs = v.iter().filter(|&x| v[i] % x == 0).count();
        ans -= ((divs/2) as f64) / divs as f64;
    }
    println!("{}",ans);
}
