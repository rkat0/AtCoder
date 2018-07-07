use std::io::*;
use std::collections::BinaryHeap;

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

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    pos: usize
}

impl Ord for State {
    fn cmp(&self, other: &State) -> std::cmp::Ordering {
        if self.cost == other.cost {
            self.pos.cmp(&other.pos)
        } else {
            other.cost.cmp(&self.cost)
        }
    }
}
impl PartialOrd for State {
    fn partial_cmp(&self, other: &State) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Clone)]
struct Edge {
    node: usize,
    cost: usize
}

fn dijkstra_all(mat: &Vec<Vec<Edge>>, s: usize) -> Vec<usize> {
    let mut heap = BinaryHeap::new();
    let mut cost: Vec<usize> = vec![usize::max_value();mat.len()];
    cost[s] = 0;
    heap.push(State {cost: 0, pos: s});
    while let Some(State {cost: c, pos: p}) = heap.pop() {
        if c > cost[p] {
            continue;
        }
        for e in &mat[p] {
            let next = State {cost: c + e.cost, pos : e.node};
            if next.cost < cost[next.pos] {
                heap.push(next);
                cost[next.pos] = next.cost;
            }
        }
    }
    cost
}

fn main() {
    let v = read_vec::<usize>();
    let (n,m,s,t) = (v[0],v[1],v[2]-1,v[3]-1);
    let w = read_mat::<usize>(m);
    let mut cy: Vec<Vec<Edge>> = vec![Vec::new();n];
    let mut cs: Vec<Vec<Edge>> = vec![Vec::new();n];
    for x in w {
        let u = x[0]-1;
        let v = x[1]-1;
        cy[u].push(Edge {node: v, cost: x[2]});
        cy[v].push(Edge {node: u, cost: x[2]});
        cs[u].push(Edge {node: v, cost: x[3]});
        cs[v].push(Edge {node: u, cost: x[3]});
    }
    let route1 = dijkstra_all(&cy, s);
    let route2 = dijkstra_all(&cs, t);
    let mut route_tot: Vec<(usize,usize)> = (0..n).map(|i| (route1[i] + route2[i], i)).collect();
    route_tot.sort_by(|a,b| a.0.cmp(&b.0));
    let mut idx = 0;
    for y in 0..n {
        while route_tot[idx].1 < y {
            idx += 1;
        }
        println!("{}",10usize.pow(15) - route_tot[idx].0);
    }
}
