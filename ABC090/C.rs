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
    let v = read_vec::<usize>();
    let (n,m) = (v[0],v[1]);
    let f1 = n==1;
    let f2 = m==1;
    let ans =
    if f1 && f2 {
        1
    }else if f1 {
        m-2
    }else if f2 {
        n-2
    }else{
        (n-2)*(m-2)
    };
    println!("{}",ans);
}
