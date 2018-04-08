use std::io::*;
use std::collections::HashMap;
use std::cmp;

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
    let mut hm: HashMap<String,i32> = HashMap::new();
    for i in 0..n {
        let s = read::<String>();
        if let Some(&v) = hm.get(&s) {
            hm.insert(s,v+1);
        } else {
            hm.insert(s,1);
        }
    }
    let m = read::<usize>();
    for i in 0..m {
        let t = read::<String>();
        if let Some(&v) = hm.get(&t) {
            hm.insert(t,v-1);
        } else {
            hm.insert(t,-1);
        }
    }
    let mut ans = 0;
    for &v in hm.values() {
        ans = cmp::max(ans,v);
    }
    println!("{}",ans);
}
