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
}

macro_rules! read_value {
    ($next:expr, ( $($t:tt),* )) => {
        ( $(read_value!($next, $t)),* )
    };

    ($next:expr, [ $t:tt ; $len:expr ]) => {
        (0..$len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
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

fn main() {
    input!{
        n: usize,
        d: usize,
        v: [usize; n]
    }
    let mut vs: Vec<(usize, usize)> = Vec::new();
    for i in 0..n {
        vs.push((i, v[i]));
    }
    vs.sort_by_key(|x| x.1);
    let mut bit1 = BIT::new(n);
    let mut bit2 = BIT::new(n);
    let mut es: Vec<(usize, usize, usize)> = Vec::new();
    bit1.update(vs[0].0, (vs[0].0, vs[0].1 + (n - vs[0].0) * d));
    bit2.update(n - 1 - vs[0].0, (vs[0].0, vs[0].1 + vs[0].0 * d));
    for i in 1..n {
        if vs[i].0 > 0 {
            let l = bit1.min(vs[i].0 - 1);
            if l.1 != usize::max_value() {
                es.push((l.0, vs[i].0, l.1 + v[vs[i].0] - (n - vs[i].0) * d));
            }
        }
        bit1.update(vs[i].0, (vs[i].0, vs[i].1 + (n - vs[i].0) * d));
        if vs[i].0 < n - 1 {
            let r = bit2.min(n - 2 - vs[i].0);
            if r.1 != usize::max_value() {
                es.push((vs[i].0, r.0, r.1 + v[vs[i].0] - vs[i].0 * d));
            }
        }
        bit2.update(n - 1 - vs[i].0, (vs[i].0, vs[i].1 + vs[i].0 * d));
    }
    es.sort_by_key(|x| x.2);
    let mut ans: usize = 0;
    let mut uf: UnionFind<usize> = UnionFind::new_points((0..n).collect());
    for (u, v, w) in es {
        if uf.is_same(&u, &v) {
            continue;
        }
        uf.union(u, v);
        ans += w;
    }
    println!("{}", ans);
}

struct BIT {
    v: Vec<(usize, usize)>,
    n: usize
}

impl BIT {
    fn new(n: usize) -> BIT {
        BIT{v: vec![(0, usize::max_value()); n + 1], n: n}
    }

    // update i th element by x
    fn update(&mut self, i: usize, x: (usize, usize)) {
        let mut id = i as i64 + 1;
        while id as usize <= self.n && self.v[id as usize].1 > x.1 {
            self.v[id as usize] = x;
            id += id & -id;
        }
    }

    // min in 0 to i th element
    fn min(&self, i: usize) -> (usize, usize) {
        let mut id = i + 1;
        let mut ret = (0, usize::max_value());
        while id > 0 {
            if ret.1 > self.v[id].1 {
                ret = self.v[id];
            }
            id &= id - 1;
        }
        ret
    }
}

use std::cmp;
use std::collections::HashMap;

struct UFNode {
    parent: usize,
    rank: usize
}

struct UnionFind<T> where T: cmp::Eq + std::hash::Hash {
    map: HashMap<T,usize>,
    nodes: Vec<UFNode>,
    next: usize,
    n_unions: usize
}

impl<T: cmp::Eq + std::hash::Hash> UnionFind<T> {
    fn new() -> UnionFind<T> {
        UnionFind{map: HashMap::new(), nodes: Vec::new(), next: 0, n_unions: 0}
    }

    fn new_points(v: Vec<T>) -> UnionFind<T> {
        let mut uf = UnionFind::new();
        for x in v {
            if uf.map.contains_key(&x) {
                continue;
            }
            uf.add_point(x);
        }
        uf
    }

    fn add_point(&mut self, x: T) -> usize {
        let id = self.next;
        self.map.insert(x,id);
        self.nodes.push(UFNode{parent: id, rank: 0});
        self.next += 1;
        self.n_unions += 1;
        id
    }

    fn get_id(&self, x: &T) -> Option<usize> {
        self.map.get(x).map(|x| *x)
    }

    fn find(&mut self, id: usize) -> usize {
        let p = self.nodes[id].parent;
        if p == id {
            return id;
        }
        let pr = self.find(p);
        self.nodes[id].parent = pr;
        pr
    }

    fn union(&mut self, x: T, y: T) {
        let idx = self.get_id(&x).unwrap_or_else(|| self.add_point(x));
        let idy = self.get_id(&y).unwrap_or_else(|| self.add_point(y));
        let xp = self.find(idx);
        let yp = self.find(idy);
        if xp == yp {
            return;
        }
        let rx = self.nodes[xp].rank;
        let ry = self.nodes[yp].rank;
        if rx < ry {
            self.nodes[xp].parent = yp;
        } else if rx > ry {
            self.nodes[yp].parent = xp;
        } else {
            self.nodes[yp].parent = xp;
            self.nodes[xp].rank += 1;
        }
        self.n_unions -= 1;
    }

    fn is_same(&mut self, x: &T, y: &T) -> bool {
        let idx = self.get_id(x);
        let idy = self.get_id(y);
        if idx == None || idy == None {
            false
        } else {
            self.find(idx.unwrap()) == self.find(idy.unwrap())
        }
    }
}
