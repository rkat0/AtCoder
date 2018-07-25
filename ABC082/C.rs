use std::io::*;
use std::collections::HashMap;

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
    let v = read_vec::<usize>();
    let mut map: HashMap<usize,usize> = HashMap::new();
    for x in v {
        if let Some(c) = map.get_mut(&x) {
            *c += 1;
            continue;
        }
        map.insert(x,1);
    }
    let mut ans = 0;
    for (x,c) in map {
        if x > c {
            ans += c;
        } else {
            ans += c - x;
        }
    }
    println!("{}",ans);
}
