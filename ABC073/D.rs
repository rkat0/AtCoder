use std::io::*;
use std::cmp;
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

fn read_mat<T: std::str::FromStr>(n: usize) -> Vec<Vec<T>> {
    (0..n).map(|_| read_vec()).collect()
}

fn search(dist:&Vec<Vec<usize>>,rs:&Vec<usize>,n:usize,rest:&mut HashSet<usize>,d:usize,min:&mut usize) {
    if rest.is_empty() {
        *min = cmp::min(*min,d);
    }else{
        for &v in rest.clone().iter() {
            let d_t = d + dist[rs[n]-1][rs[v]-1];
            if d_t >= *min {
                continue;
            }
            rest.remove(&v);
            search(dist,rs,v,rest,d_t,min);
            rest.insert(v);
        }
    }
}

fn main() {
    let v = read_vec::<usize>();
    let (n,m,r) = (v[0],v[1],v[2]);
    let rs = read_vec::<usize>();
    let mut path = vec![vec![1<<30;n];n];
    for _i in 0..m {
        let v = read_vec::<usize>();
        path[v[0]-1][v[1]-1] = v[2];
        path[v[1]-1][v[0]-1] = v[2];
    }
    for i in 0..n {
        for j in 0..n {
            for k in 0..n {
                path[j][k] = cmp::min(path[j][k],path[j][i]+path[i][k]);
            }
        }
    }
    let mut min: usize = 1<<30;
    let all: HashSet<usize> = (0..r).collect();
    for i in 0..r-1 {
        let mut rest = all.clone();
        rest.remove(&i);
        search(&path,&rs,i,&mut rest,0,&mut min);
    }
    println!("{}",min);
}
