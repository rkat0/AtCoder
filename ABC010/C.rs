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
    let v = read_vec::<i64>();
    let (sx,sy,tx,ty,t,v) = (v[0],v[1],v[2],v[3],v[4],v[5]);
    let n = read::<usize>();
    let mut ans = false;
    for i in 0..n {
        let xy = read_vec::<i64>();
        if (((sx-xy[0])*(sx-xy[0])+(sy-xy[1])*(sy-xy[1])) as f64).sqrt()
            + (((tx-xy[0])*(tx-xy[0])+(ty-xy[1])*(ty-xy[1])) as f64).sqrt() <= (t * v) as f64 {
            ans = true;
            break;
        }
    }
    println!("{}",if ans {"YES"} else {"NO"});
}
