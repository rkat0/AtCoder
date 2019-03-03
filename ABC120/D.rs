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

fn main() {
    input!{
        n: usize,
        m: usize,
        v: [(usize1, usize1); m]
    }
    let mut ans: Vec<usize> = vec![0; m];
    ans[m - 1] = n * (n - 1) / 2;
    let mut uf: UnionFind<usize> = UnionFind::new_points((0..n).collect());
    for i in (0..m - 1).rev() {
        let x = v[i + 1].0;
        let y = v[i + 1].1;
        ans[i] = ans[i + 1];
        if uf.is_same(&x, &y) {
            continue;
        }
        ans[i] -= uf.size(&x) * uf.size(&y);
        uf.union(x, y);
    }
    for i in 0..m {
        println!("{}", ans[i]);
    }
}

use std::cmp;
use std::collections::{HashSet, HashMap};

struct UFNode {
    parent: usize,
    rank: usize,
    size: usize
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
        self.nodes.push(UFNode{parent: id, rank: 0, size: 1});
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
        let size = self.nodes[xp].size + self.nodes[yp].size;
        self.nodes[xp].size = size;
        self.nodes[yp].size = size;
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
    
    fn size(&mut self, x: &T) -> usize {
        let id = self.get_id(x).unwrap();
        let idp = self.find(id);
        self.nodes[idp].size
    }
}
