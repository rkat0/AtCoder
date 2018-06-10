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
    let (n,m) = (v[0],v[1]);
    let d = read_mat::<usize>(m);
    let c = read_mat::<usize>(n);
    let mut min3: Vec<Vec<(usize,usize)>> = vec![vec![(usize::max_value(),0);3];3];
    for col in 0..m {
        let mut tv: Vec<usize> = vec![0;3];
        for i in 0..n {
            for j in 0..n {
                tv[(i+j)%3] += d[c[i][j]-1][col];
            }
        }
        min3[0].push((tv[0],col));
        min3[0].sort_by(|a,b| a.0.cmp(&b.0));
        min3[0].remove(3);
        min3[1].push((tv[1],col));
        min3[1].sort_by(|a,b| a.0.cmp(&b.0));
        min3[1].remove(3);
        min3[2].push((tv[2],col));
        min3[2].sort_by(|a,b| a.0.cmp(&b.0));
        min3[2].remove(3);
    }
    let mut ans = usize::max_value();
    for i in 0..3 {
        for j in 0..3 {
            for k in 0..3 {
                if min3[0][i].1 != min3[1][j].1 && min3[1][j].1 != min3[2][k].1 && min3[0][i].1 != min3[2][k].1 {
                    ans = cmp::min(ans,min3[0][i].0 + min3[1][j].0 + min3[2][k].0);
                }
            }
        }
    }
    println!("{}",ans);
}
