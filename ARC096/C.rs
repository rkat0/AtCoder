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

fn read_mat<T: std::str::FromStr>(n: usize) -> Vec<Vec<T>> {
    (0..n).map(|_| read_vec()).collect()
}

fn main() {
    let v = read_vec::<i64>();
    let (a,b,c) = (v[0],v[1],v[2]);
    let (mut x,mut y) = (v[3],v[4]);
    let mut ans = 0;
    if a + b > 2 * c && x > 0 && y > 0 {
        let m = cmp::min(x,y);
        ans += m * 2 * c;
        x -= m;
        y -= m;
    }
    if a >= 2 * c {
        ans += x * 2 * c;
        x = 0;
        y -= x;
    }
    if b >= 2 * c {
        ans += y * 2 * c;
        y = 0;
        x -= y;
    }
    if x > 0 {
        ans += x * a;
    }
    if y > 0 {
        ans += y * b;
    }
    println!("{}",ans);
}
