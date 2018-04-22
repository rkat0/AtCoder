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

fn read_mat<T: std::str::FromStr>(n: usize) -> Vec<Vec<T>> {
    (0..n).map(|_| read_vec()).collect()
}

fn main() {
    let n = read::<usize>();
    let mut m: HashMap<String,usize> = HashMap::new();
    for i in 0..n {
        let s = read::<String>();
        if let Some(v) = m.get(&s).cloned() {
            m.insert(s,v+1);
        } else {
            m.insert(s,1);
        }
    }
    let ans = m.iter().max_by_key(|x| x.1).unwrap().0;
    println!("{}",ans);
}
