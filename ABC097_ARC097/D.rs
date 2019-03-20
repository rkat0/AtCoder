use std::io::*;
use std::collections::HashSet;

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

fn find(uf: &mut Vec<(usize,usize)>, x: usize) -> usize {
    let p = uf[x].0;
    if p != x {
        let r = find(uf, p);
        uf[x].0 = r;
    }
    uf[x].0
}

fn main() {
    let v = read_vec::<usize>();
    let (n,m) = (v[0],v[1]);
    let p: Vec<usize> = read_vec::<usize>().iter().map(|x| x-1).collect();

    let mut uf: Vec<(usize,usize)> = (0..n).map(|x| (x,0)).collect();

    for i in 0..m {
        let xy = read_vec::<usize>();
        let (x,y) = (xy[0]-1,xy[1]-1);
        let xr = find(&mut uf,x);
        let yr = find(&mut uf,y);
        if xr == yr { continue; }
        if uf[xr].1 < uf[yr].1 {
            uf[xr].0 = yr;
        } else {
            uf[yr].0 = xr;
            if uf[xr].1 == uf[yr].1 {
                uf[xr].1 += 1;
            }
        }
    }
    let mut ans = 0;
    for i in 0..n {
        if find(&mut uf, i) == find(&mut uf, p[i]) {
            ans += 1;
        }
    }
    println!("{}",ans);
}
