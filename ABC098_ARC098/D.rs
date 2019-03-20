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

fn read_vec_char() -> Vec<char> {
    read::<String>().chars().collect()
}

fn read_mat<T: std::str::FromStr>(n: usize) -> Vec<Vec<T>> {
    (0..n).map(|_| read_vec()).collect()
}

fn read_mat_char(n: usize) -> Vec<Vec<char>> {
    (0..n).map(|_| read_vec_char()).collect()
}

fn main() {
    let n = read::<usize>();
    let a = read_vec::<usize>();
    let mut ans = 0;
    let mut t = a[0];
    let mut k = 0;
    for i in 0..n {
        if k < i {
            k = i;
            t = a[i];
        }
        while k < n-1 && t & a[k+1] == 0 {
            t ^= a[k+1];
            k += 1;
        }
        if k == n-1 {
            ans += (k + 1 - i) * (k + 2 - i) / 2;
            break;
        }
        ans += k + 1 - i;
        t ^= a[i];
    }
    println!("{}",ans);
}
