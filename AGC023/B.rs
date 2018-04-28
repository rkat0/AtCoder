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
    let s: Vec<Vec<char>> = (0..n).map(|_| read::<String>().chars().collect()).collect();
    let mut ans = 0;
    'outer: for k in 0..n {
        for i in 0..n-1 {
            for j in i+1..n {
                if s[i][(j + k) % n] as u32 != s[j][(i + k) % n] as u32 {
                    continue 'outer;
                }
            }
        }
        ans += n;
    }
    println!("{}",ans);
}
