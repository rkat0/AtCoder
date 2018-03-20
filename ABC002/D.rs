use std::io::*;
use std::cmp;
use std::collections::HashSet;

fn read<T: std::str::FromStr>() -> T {
    let stdin = stdin();
    let mut buf = String::new();
	let _ = stdin.lock().read_line(&mut buf);
	buf.trim().parse().ok().unwrap()
}

fn read_vec<T: std::str::FromStr>() -> Vec<T> {
	read::<String>().trim().split_whitespace()
        .map(|w| w.parse().ok().unwrap()).collect()
}

fn read_mat<T: std::str::FromStr>(n: usize) -> Vec<Vec<T>> {
    (0..n).map(|_| read_vec()).collect()
}

fn solve(es:&Vec<Vec<usize>>,n:usize) -> usize {
    let mut graph: Vec<Vec<usize>> = Vec::new();
    let mut max = 1;

    for _i in 0..n {
        graph.push(Vec::new());
    }

    for e in es {
        let (v,w) = (e[0]-1,e[1]-1);
        graph[v].push(w);
        graph[w].push(v);
    }
    for i in 0..n {
        let mut has: HashSet<usize> = HashSet::new();
        has.insert(i as usize);
        search(&graph,i as usize,&mut has,&mut max);
    }
    max
}

fn search(g:&Vec<Vec<usize>>,i:usize,has:&mut HashSet<usize>,max:&mut usize) {
    for &v in g[i].iter() {
        if v >= i || has.contains(&v) || g[v].len() < *max {
            continue;
        }
        if g[v].iter().cloned().collect::<HashSet<usize>>().is_superset(&has) {
            has.insert(v);
            *max=cmp::max(*max,has.len());
            search(g,v,has,max);
            has.remove(&v);
        }
    }
}

fn main() {
    let v = read_vec::<usize>();
    let e = read_mat::<usize>(v[1]);
	println!("{}",solve(&e,v[0]));
}
