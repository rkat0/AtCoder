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
    let n = read::<usize>();
    let mut s: Vec<char> = vec!['1','2','3','4','5','6'];
    s = rot(s,n/5%6);
    s = swaps(s,n%5);
    for c in s {
        print!("{}",c);
    }
    println!("");
}

fn swaps(s: Vec<char>, n: usize) -> Vec<char> {
    let mut res = s;
    for i in 0..n {
        res.swap(i,i+1);
    }
    res
}

fn rot(s: Vec<char>, n: usize) -> Vec<char> {
    let (l,r) = s.split_at(n);
    let mut res: Vec<char> = r.to_vec();
    res.append(&mut l.to_vec());
    res
}
