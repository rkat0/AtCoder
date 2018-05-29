use std::io::*;
use std::collections::HashMap;
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

type NodeId = usize;

struct Node<T> {
    id: NodeId,
    val: T,
    con: Vec<(NodeId,i64)>
}

struct NodeArena<T> {
    arena: Vec<Node<T>>
}

impl<T> NodeArena<T> {
    fn new() -> NodeArena<T> {
        NodeArena {arena: Vec::new()}
    }

    fn alloc(&mut self, value: T) -> NodeId {
        let id = self.arena.len();
        let node = Node {id: id, val: value, con: Vec::new()};
        self.arena.push(node);
        id
    }

    fn get(&self, id: NodeId) -> &Node<T> {
        &self.arena[id]
    }
    fn get_mut(&mut self,id: NodeId) -> &mut Node<T> {
        &mut self.arena[id]
    }
}

struct Graph<T> {
    map: HashMap<T,NodeId>,
    arena: NodeArena<T>
}

impl<T> Graph<T> where T: std::hash::Hash + std::cmp::Eq + Copy {
    fn new() -> Graph<T> {
        Graph {map: HashMap::new(), arena: NodeArena::new()}
    }

    fn contain(&mut self, v: &T) -> bool {
        self.map.contains_key(v)
    }

    fn vertices(&self) -> HashSet<NodeId> {
        self.map.values().cloned().collect()
    }

    fn get_node_id(&self, i: NodeId) -> &Node<T> {
        self.arena.get(i)
    }

    fn add_vertex(&mut self, v: T) {
        if !self.contain(&v) {
            let nid = self.arena.alloc(v);
            self.map.insert(v,nid);
        }
    }

    fn add_edge_w(&mut self, v1: T, v2: T, w: i64) {
        if !self.contain(&v1) {
            self.add_vertex(v1);
        }
        if !self.contain(&v2) {
            self.add_vertex(v2);
        }
        let id1 = *self.map.get(&v1).unwrap();
        let id2 = *self.map.get(&v2).unwrap();
        {
            let n1 = self.arena.get_mut(id1);
            n1.con.push((id2,w));
        }
        {
            let n2 = self.arena.get_mut(id2);
            n2.con.push((id1,-w));
        }
    }
}

fn search(g: &Graph<i64>, set: &HashSet<NodeId>) -> bool {
    if set.is_empty() {
        return true;
    }
    let r = *set.iter().nth(0).unwrap();
    let mut dist = HashMap::new();
    dist.insert(r,0);
    if search2(g,r,&mut dist) {
        let setd = set.difference(&dist.keys().cloned().collect()).cloned().collect();
        search(g, &setd)
    } else {
        false
    }
}

fn search2(g: &Graph<i64>, v: NodeId, d: &mut HashMap<NodeId,i64>) -> bool {
    let node = g.get_node_id(v);
    let dv = *d.get(&v).unwrap();
    for nw in node.con.iter() {
        let (n,w) = (nw.0,nw.1);
        if let Some(&dist) = d.get(&n) {
            if dist != dv + w {
                return false;
            }
            continue;
        }
        d.insert(n,dv + w);
        if !search2(g,n,d) {
            return false;
        }
    }
    return true;
}

fn main() {
    let v = read_vec::<usize>();
    let (n,m) = (v[0],v[1]);
    let mut g: Graph<i64> = Graph::new();
    for i in 0..m {
        let lrd = read_vec::<i64>();
        g.add_edge_w(lrd[0],lrd[1],lrd[2]);
    }
    let ans = search(&g,&g.vertices());
    println!("{}",if ans { "Yes" } else { "No" });
}
