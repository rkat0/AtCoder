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
    let v = read_vec::<i64>();
    let acm: Vec<i64> = v.iter().scan(0,|st, &x| {*st += x; Some(*st)}).collect();
    let mut ans = 0;
    let mut count: HashMap<i64,usize> = HashMap::new();
    for i in 0..n {
        if acm[i] == 0 {
            ans += 1;
        }
        if let Some(c) = count.get_mut(&acm[i]) {
            ans += *c;
            *c += 1;
            continue;
        }
        count.insert(acm[i], 1);
    }
    println!("{}",ans);
}
