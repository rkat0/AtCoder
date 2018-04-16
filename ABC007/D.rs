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
    let (l,r) = (v[0],v[1]);
    let ans = solve(r) - solve(l-1);
    println!("{}",ans);
}

fn solve(n: usize) -> usize {
    let mut m = n;
    let mut ds = Vec::new();
    while m > 0 {
        ds.push(m%10);
        m /= 10;
    }
    let mut ret = 0;
    while let Some(d) = ds.pop() {
        let dig: u32 = ds.len() as u32;
        if d <= 4 {
            ret += d * count(dig);
        }
        if d == 4 {
            ret += n % 10usize.pow(dig) + 1;
            break;
        }
        if d > 4 {
            ret += (d-1) * count(dig) + 10usize.pow(dig);
        }
        if d == 9 {
            ret += n % 10usize.pow(dig) + 1;
            break;
        }
    }
    ret
}

fn count(n: u32) -> usize {
    let mut ret = 0;
    for i in 0..n {
        ret = 8 * ret + 2 * 10usize.pow(i);
    }
    ret
}
