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
    let v = read_vec::<i64>();
    let (n,m) = (v[0],v[1]);
    let (a,b,c) = if n * 2 > m || n * 4 < m {
        (-1,-1,-1)
    } else {
        let r = m - 2 * n;
        if r <= n {
            (n-r,r,0)
        } else {
            (0,2*n-r,r-n)
        }
    };
    println!("{} {} {}",a,b,c);
}
