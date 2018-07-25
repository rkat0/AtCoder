use std::io::*;
use std::collections::BTreeMap;

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
    let s = read_vec_char();
    let mut map: BTreeMap<(Vec<char>,Vec<char>),usize> = BTreeMap::new();
    for i in 0..1<<n {
        let mut v1 = vec![];
        let mut v2 = vec![];
        for j in 0..n {
            if 1<<j & i != 0 {
                v1.push(s[j]);
            } else {
                v2.push(s[j]);
            }
        }
        *map.entry((v1,v2)).or_insert(0) += 1;
    }
    let mut map2: BTreeMap<(Vec<char>,Vec<char>),usize> = BTreeMap::new();
    for i in 0..1<<n {
        let mut v1 = vec![];
        let mut v2 = vec![];
        for j in 0..n {
            if 1<<j & i != 0 {
                v1.push(s[n+j]);
            } else {
                v2.push(s[n+j]);
            }
        }
        v1.reverse();
        v2.reverse();
        *map2.entry((v1,v2)).or_insert(0) += 1;
    }
    let mut ans = 0;
    for (k,v) in map.iter() {
        if let Some(x) = map2.get(k) {
            ans += v * x;
        }
    }
    println!("{}",ans);
}
