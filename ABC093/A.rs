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
    let s = read::<String>();
    let mut a = false;
    let mut b = false;
    let mut c = false;
    for x in s.chars() {
        if x =='a' {
            a = true;
        } else if x == 'b' {
            b = true;
        } else if x == 'c' {
            c = true;
        }
    }
    println!("{}",if a && b && c { "Yes" } else { "No" });
}
