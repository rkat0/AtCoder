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
    let n = read::<usize>();
    let mut d = read_mat::<usize>(n);
    for i in 1..n {
        d[0][i] += d[0][i-1];
        d[i][0] += d[i-1][0];
    }
    for i in 1..n {
        for j in 1..n {
            d[i][j] += d[i][j-1] + d[i-1][j] - d[i-1][j-1];
        }
    }
    let q = read::<usize>();
    for i in 0..q {
        let p = read::<usize>();
        let prod: Vec<(usize,usize)> = (1..sqrti(p)+1).map(|x| (x,cmp::min(p/x,n))).collect();
        let ans = prod.iter().map(|&pat| search(n,&d,pat)).max().unwrap();
        println!("{}",ans);
    }
}

fn sqrti(n: usize) -> usize {
    (n as f64).sqrt().floor() as usize
}

fn search(n: usize, d: &Vec<Vec<usize>>, pat:(usize,usize)) -> usize {
    let (x,y) = pat;
    let mut max = 0;
    for i in 0..n-x+1 {
        for j in 0..n-y+1 {
            max = cmp::max(max,count(d,i,j,x,y));
            max = cmp::max(max,count(d,j,i,y,x));
        }
    }
    max
}

fn count(d: &Vec<Vec<usize>>, i: usize, j: usize, h: usize, w: usize) -> usize {
    let i2 = i + h - 1;
    let j2 = j + w - 1;
    let mut s = d[i2][j2];
    if i > 0 && j > 0 {
        s += d[i-1][j-1];
    }
    if i > 0 {
        s -= d[i-1][j2];
    }
    if j > 0 {
        s -= d[i2][j-1];
    }
    s
}
