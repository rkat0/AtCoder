macro_rules! input {
    (source = $s:expr, $($r:tt)*) => {
        let mut iter = $s.split_whitespace();
        let mut next = || { iter.next().unwrap() };
        input_inner!{next, $($r)*}
    };
    ($($r:tt)*) => {
        let stdin = std::io::stdin();
        let mut bytes = std::io::Read::bytes(std::io::BufReader::new(stdin.lock()));
        let mut next = move || -> String {
            bytes
                .by_ref()
                .map(|r| r.unwrap() as char)
                .skip_while(|c| c.is_whitespace())
                .take_while(|c| !c.is_whitespace())
                .collect()
        };
        input_inner!{next, $($r)*}
    };
}

macro_rules! input_inner {
    ($next:expr) => {};
    ($next:expr, ) => {};

    ($next:expr, $var:ident : $t:tt $($r:tt)*) => {
        let $var = read_value!($next, $t);
        input_inner!{$next $($r)*}
    };

    ($next:expr, mut $var:ident : $t:tt $($r:tt)*) => {
        let mut $var = read_value!($next, $t);
        input_inner!{$next $($r)*}
    };
}

macro_rules! read_value {
    ($next:expr, ( $($t:tt),* )) => {
        ( $(read_value!($next, $t)),* )
    };

    ($next:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
    };

    ($next:expr, [ $t:tt ]) => {
        {
            let len = read_value!($next, usize);
            (0..len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
        }
    };

    ($next:expr, chars) => {
        read_value!($next, String).chars().collect::<Vec<char>>()
    };

    ($next:expr, usize1) => {
        read_value!($next, usize) - 1
    };

    ($next:expr, $t:ty) => {
        $next().parse::<$t>().expect("Parse error")
    };
}

use std::cmp::{min, max};

fn main() {
    input!{
        xs: i64,
        ys: i64,
        xt: i64,
        yt: i64,
        n: usize,
        v: [(i64, i64, usize); n]
    }
    let mut g: Vec<Vec<(usize, f64)>> = vec![vec![]; n + 2];
    for i in 0..n {
        for j in i + 1..n {
            let d = dist2d(v[i], v[j]);
            g[i].push((j, d));
            g[j].push((i, d));
        }
    }
    for i in 0..n {
        let ds = dist2d((xs, ys, 0), v[i]);
        g[n].push((i, ds));
        let dt = dist2d((xt, yt, 0), v[i]);
        g[i].push((n + 1, dt));
    }
    g[n].push((n + 1, dist2d((xs, ys, 0), (xt, yt, 0))));
    let ans: f64 = dijkstra(&g, n, n + 1).unwrap();
    println!("{}", ans);
}

fn dist2d(s: (i64, i64, usize), t: (i64, i64, usize)) -> f64 {
    let d2 = (s.0 - t.0) * (s.0 - t.0) + (s.1 - t.1) * (s.1 - t.1);
    let ret = (d2 as f64).sqrt() - (s.2 + t.2) as f64;
    if ret > 0.0 {
        ret
    } else {
        0.0
    }
}

use std::collections::BinaryHeap;
use std::cmp::Ordering;

#[derive(Debug, PartialEq)]
struct DijkstraState {
    v: usize,
    w: f64
}

impl Eq for DijkstraState {}

impl Ord for DijkstraState {
    fn cmp(&self, other: &Self) -> Ordering {
        if other.w < self.w {
            Ordering::Less
        } else if other.w > self.w {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    }
}

impl PartialOrd for DijkstraState {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

fn dijkstra(g: &Vec<Vec<(usize, f64)>>, s: usize, t: usize) -> Option<f64> {
    let n = g.len();
    let mut prev: Vec<Option<f64>> = vec![None; n];
    let mut pq: BinaryHeap<DijkstraState> = BinaryHeap::new();
    pq.push(DijkstraState{v: s, w: 0.0});
    prev[s] = Some(0.0);
    while let Some(s) = pq.pop() {
        if prev[s.v].map_or(false, |x| x < s.w) {
            continue;
        }
        if s.v == t {
            return Some(s.w);
        }
        for v in &g[s.v] {
            let d = s.w + v.1;
            if prev[v.0].map_or(true, |x| x > d) {
                prev[v.0] = Some(d);
                pq.push(DijkstraState{v: v.0, w: d});
            }
        }
    }
    None
}
