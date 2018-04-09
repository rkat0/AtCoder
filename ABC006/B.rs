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
    let mut t = vec![0,0,1];
    let ans: usize;
    if n < 4 {
        ans = t[n-1];
    } else {
        for i in 4..n+1 {
            let next = (t[0]+t[1]+t[2]) % 10007;
            t[0] = t[1];
            t[1] = t[2];
            t[2] = next;
        }
        ans = t[2];
    }
    println!("{}",ans);
}
