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
    let v = read_vec::<usize>();
    let (n,k) = (v[0],v[1]);
    let w = read_vec::<usize>();
    let mut m: HashMap<usize,usize> = HashMap::new();
    for a in w {
        if let Some(x) = m.get_mut(&a) {
            *x += 1;
            continue;
        }
        m.insert(a,1);
    }
    let mut val: Vec<_> = m.values().map(|x| *x).collect();
    let mut ans = 0;
    let l = val.len();
    if l > k {
        val.sort();
        ans = val[0..l-k].iter().sum();
    }
    println!("{}",ans);
}
