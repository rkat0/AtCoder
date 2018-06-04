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
    let n = read::<usize>();
    let mut ls: Vec<i64> = Vec::new();
    let mut rs: Vec<i64> = Vec::new();
    for i in 0.. n {
        let v = read_vec::<i64>();
        ls.push(v[0]);
        rs.push(v[1]);
    }
    ls.sort_by(|a,b| b.cmp(a));
    rs.sort();
    let mut li = 0;
    let mut ri = 0;
    let mut x = 0;
    let mut ans1 = 0;
    loop {
        if x < ls[li] {
            ans1 += ls[li] - x;
            x = ls[li];
            li += 1;
        } else {
            ans1 += x.abs();
            break;
        }
        if x > rs[ri] {
            ans1 += x - rs[ri];
            x = rs[ri];
            ri += 1;
        } else {
            ans1 += x.abs();
            break;
        }
    }
    li = 0;
    ri = 0;
    x = 0;
    let mut ans2 = 0;
    loop {
        if x > rs[ri] {
            ans2 += x - rs[ri];
            x = rs[ri];
            ri += 1;
        } else {
            ans2 += x.abs();
            break;
        }
        if x < ls[li] {
            ans2 += ls[li] - x;
            x = ls[li];
            li += 1;
        } else {
            ans2 += x.abs();
            break;
        }
    }
    let ans = cmp::max(ans1,ans2);
    println!("{}",ans);
}
