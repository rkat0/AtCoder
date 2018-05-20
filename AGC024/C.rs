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
    let mut av: Vec<i64> = Vec::with_capacity(n);
    for i in 0..n {
        let a = read::<i64>();
        av.push(a);
    }
    let mut ans = 0;
    let mut x = 0;
    let mut suc = true;
    for i in 0..n {
        let a = av.pop().unwrap();
        if a < x - 1 || a as usize > n - i - 1 {
            suc = false;
            break;
        }
        if a == x - 1 {
            x -= 1;
        } else {
            ans += a;
            x = a;
        }
    }
    if suc {
        println!("{}",ans);
    } else {
        println!("-1");
    }
}
