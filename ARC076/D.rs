use io::*;
use std::collections::HashMap;
use std::cmp;

fn main() {
    let input = read_string();
    let mut sc = Scanner::new(&input);
    let n: usize = sc.next();
    let mut xy: Vec<Vec<usize>> = sc.next_mat(2, n);
    for i in 0..n {
        xy[i].insert(0,i);
    }
    let mut es: Vec<(usize,usize,usize)> = Vec::new();
    xy.sort_by_key(|x| x[1]);
    for e in xy.windows(2) {
        es.push((e[1][1] - e[0][1], e[0][0], e[1][0]));
    }
    xy.sort_by_key(|x| x[2]);
    for e in xy.windows(2) {
        es.push((e[1][2] - e[0][2], e[0][0], e[1][0]));
    }
    es.sort_by_key(|x| x.0);
    let mut ans = 0;
    let mut uf: UnionFind<usize> = UnionFind::new();
    for (d,u,v) in es {
        if uf.is_same(u, v) {
            continue;
        }
        ans += d;
        uf.union(u ,v);
        if uf.size() == n && uf.n_unions() == 1 {
            break;
        }
    }
    println!("{}",ans);
}

struct UFNode {
    parent: usize,
    rank: usize
}

struct UnionFind<T> where T: cmp::Eq + std::hash::Hash {
    map: HashMap<T,usize>,
    nodes: Vec<UFNode>,
    n_unions: usize
}

impl<T: cmp::Eq + std::hash::Hash> UnionFind<T> {
    fn new() -> UnionFind<T> {
        UnionFind{map: HashMap::new(), nodes: Vec::new(), n_unions: 0}
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
        let id = self.nodes.len();
        self.map.insert(x,id);
        self.nodes.push(UFNode{parent: id, rank: 0});
        self.n_unions += 1;
        id
    }

    fn get_id(&mut self, x: T) -> usize {
        self.map.get(&x).map(|x| *x).unwrap_or_else(|| self.add_point(x))
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
        let idx = self.get_id(x);
        let idy = self.get_id(y);
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

    fn is_same(&mut self, x: T, y: T) -> bool {
        let idx = self.get_id(x);
        let idy = self.get_id(y);
        self.find(idx) == self.find(idy)
    }

    fn size(&self) -> usize {
        self.nodes.len()
    }

    fn n_unions(&self) -> usize {
        self.n_unions
    }
}

pub mod io {
    use std;
    use std::str::FromStr;

    pub struct Scanner<'a> {
        iter: std::str::SplitWhitespace<'a>,
    }

    impl<'a> Scanner<'a> {
        pub fn new(s: &'a str) -> Scanner<'a> {
            Scanner {
                iter: s.split_whitespace(),
            }
        }

        pub fn next<T: FromStr>(&mut self) -> T {
            let s = self.iter.next().unwrap();
            if let Ok(v) = s.parse::<T>() {
                v
            } else {
                panic!("Parse error")
            }
        }

        pub fn next_vec_len<T: FromStr>(&mut self) -> Vec<T> {
            let n: usize = self.next();
            self.next_vec(n)
        }

        pub fn next_vec<T: FromStr>(&mut self, n: usize) -> Vec<T> {
            (0..n).map(|_| self.next()).collect()
        }

        pub fn next_mat<T: FromStr>(&mut self, w: usize, h: usize) -> Vec<Vec<T>> {
            (0..h).map(|_| self.next_vec(w)).collect()
        }

        pub fn next_vec_char(&mut self) -> Vec<char> {
            self.next::<String>().chars().collect()
        }

        pub fn next_mat_char(&mut self, n: usize) -> Vec<Vec<char>> {
            (0..n).map(|_| self.next_vec_char()).collect()
        }
    }

    pub fn read_string() -> String {
        use std::io::Read;

        let mut s = String::new();
        std::io::stdin().read_to_string(&mut s).unwrap();
        s
    }

    pub fn read_line() -> String {
        let mut s = String::new();
        std::io::stdin().read_line(&mut s).unwrap();
        s.trim_right().to_owned()
    }
}
