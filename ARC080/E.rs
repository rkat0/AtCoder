use io::*;
use std::collections::BinaryHeap;
use std::cmp::Ordering;

#[derive(Eq)]

struct Node {
    v1: usize,
    v2: usize,
    child: Vec<(usize,usize)>
}

impl Ord for Node {
    fn cmp(&self, other: &Node) -> Ordering {
        other.v1.cmp(&self.v1)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Node) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Node) -> bool {
        self.v1 == other.v1
    }
}

struct SegTree {
    t: Vec<(usize,usize)>,
    len: usize
}

impl SegTree {
    fn new(v: &Vec<usize>) -> SegTree {
        let mut l = 1;
        while l < v.len() {
            l *= 2;
        }
        let mut t: Vec<(usize,usize)> = vec![(usize::max_value(),usize::max_value()); 2 * l - 1];
        for i in 0..v.len() {
            t[i + l - 1] = (i,v[i]);
        }
        for i in 1..l {
            let idx = l - i - 1;
            t[idx] = if t[2*idx+1].1 <= t[2*idx+2].1 {
                t[2*idx+1]
            } else {
                t[2*idx+2]
            }
        }
        SegTree{t: t, len: l}
    }

    fn range(&self, mut l: usize, mut r: usize) -> (usize,usize) {
        l += self.len - 1;
        r += self.len - 1;
        let mut ret = (0,usize::max_value());
        while l < r {
            if l % 2 == 0 && ret.1 > self.t[l].1 {
                ret = self.t[l];
            }
            l /= 2;
            if r % 2 == 0 && ret.1 > self.t[r - 1].1 {
                ret = self.t[r - 1];
            }
            r = (r - 1) / 2;
        }
        ret
    }
}

fn main() {
    let input = read_string();
    let mut sc = Scanner::new(&input);
    let n: usize = sc.next();
    let ps: Vec<usize> = sc.next_vec(n);
    let (ev,ov) = partition(&ps);
    let et: SegTree = SegTree::new(&ev);
    let ot: SegTree = SegTree::new(&ov);
    let mut pq: BinaryHeap<Node> = BinaryHeap::new();
    let head = solve(&et, &ot ,0, n);
    print!("{} {}", head.v1, head.v2);
    for (l,r) in head.child {
        pq.push(solve(&et, &ot, l, r));
    }
    loop {
        if pq.is_empty() {
            break;
        }
        let node = pq.pop().unwrap();
        print!(" {} {}", node.v1, node.v2);
        for (l,r) in node.child {
            pq.push(solve(&et, &ot, l, r));
        }
    }
    print!("\n");
}

fn partition(v: &Vec<usize>) -> (Vec<usize>,Vec<usize>) {
    let mut ev: Vec<usize> = vec![0; (v.len() + 1) / 2];
    let mut ov: Vec<usize> = vec![0; v.len() / 2];
    for i in 0..v.len() {
        if i % 2 == 0 {
            ev[i / 2] = v[i];
        } else {
            ov[i / 2] = v[i];
        }
    }
    (ev,ov)
}

fn solve(et: &SegTree, ot: &SegTree, l: usize, r: usize) -> Node {
    let (ei, e): (usize,usize);
    let (oi, o): (usize,usize);
    if l % 2 == 0 {
        let en = et.range((l + 1) / 2, (r + 1) / 2);
        ei = en.0 * 2;
        let on = ot.range(ei / 2, r / 2);
        oi = on.0 * 2 + 1;
        e = en.1;
        o = on.1;
    } else {
        let en = ot.range(l / 2, r / 2);
        ei = en.0 * 2 + 1;
        let on = et.range((ei + 1) / 2, (r + 1) / 2);
        oi = on.0 * 2;
        e = en.1;
        o = on.1;
    }
    let mut child: Vec<(usize,usize)> = Vec::new();
    if l < ei {
        child.push((l, ei));
    }
    if ei + 1 < oi {
        child.push((ei + 1, oi));
    }
    if oi + 1 < r {
        child.push((oi + 1, r));
    }
    Node{v1: e, v2: o, child: child}
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
