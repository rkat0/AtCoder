use std::io::*;
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
    let (n,k,q) = (v[0],v[1],v[2]);
    let a = read_vec::<usize>();
    let mut xs: Vec<usize> = Vec::new();
    for i in 0..n-k+1 {
        let mut m = a[i];
        for j in i+1..i+k {
            m = cmp::min(m,a[j]);
        }
        if !xs.contains(&m) {
            xs.push(m);
        }
    }
    let mut ans = usize::max_value();
    for x in xs {
        let mut ups: Vec<usize> = Vec::new();
        for sub in a.split(|&y| y < x) {
            let l = sub.len();
            if l < k {
                continue;
            }
            let mut ssub = sub.to_vec();
            ssub.sort();
            let mut ssub2 = ssub[0..l-k+1].to_vec();
            ups.append(&mut ssub2);
        }
        if ups.len() >= q {
            ups.sort();
            ans = cmp::min(ans,ups[q-1]-x);
        }
    }
    println!("{}",ans);
}
