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

fn divide2(s: usize, t: usize, cum: &Vec<i64>) -> Vec<i64> {
    let mut l = s;
    let mut r = t-1;
    let mut m = (l+r) / 2;
    let mut m1 = cum[m] - cum[s-1];
    let mut m2 = cum[t] - cum[m];
    let mut dmin = m1 - m2;
    if dmin == 0 {
        return vec![m1,m2];
    } else if dmin > 0 {
        r = m;
    } else {
        l = m;
    }
    while l + 1 < r {
        m = (l+r) / 2;
        let t1 = cum[m] - cum[s-1];
        let t2 = cum[t] - cum[m];
        let dt = t1 - t2;
        if dmin.abs() > dt.abs() {
            m1 = t1;
            m2 = t2;
            dmin = dt;
        }
        if dt == 0 {
            return vec![m1,m2];
        } else if dt > 0 {
            r = m;
        } else {
            l = m;
        }
    }
    let t1 = cum[l] - cum[s-1];
    let t2 = cum[t] - cum[l];
    let dt = t1 - t2;
    if dmin.abs() > dt.abs() {
        m1 = t1;
        m2 = t2;
        dmin = dt;
    }
    let t1 = cum[r] - cum[s-1];
    let t2 = cum[t] - cum[r];
    let dt = t1 - t2;
    if dmin.abs() > dt.abs() {
        m1 = t1;
        m2 = t2;
    }
    return vec![m1,m2];
}

fn main() {
    let n = read::<usize>();
    let a = read_vec::<i64>();
    let mut cum: Vec<i64> = vec![0;n+1];
    for i in 0..n {
        cum[i+1] = cum[i] + a[i];
    }
    let mut ans = -1;
    for p in 3..n {
        let m1 = divide2(1,p-1,&cum);
        let m2 = divide2(p,n,&cum);
        let min = cmp::min(cmp::min(m1[0],m1[1]),cmp::min(m2[0],m2[1]));
        let max = cmp::max(cmp::max(m1[0],m1[1]),cmp::max(m2[0],m2[1]));
        if ans < 0 || ans > max - min {
            ans = max - min;
        }
    }
    println!("{}",ans);
}
