use std::io::*;
use std::collections::{HashMap,HashSet};

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
    let v = read_vec::<usize>();
    let (n,g,e) = (v[0],v[1],v[2]);
    let ps = read_vec::<usize>();
    let es = read_mat::<usize>(e);
    if g == 0 {
        println!("{}",0);
        return;
    }
    let mut net: Graph<usize> = Graph::new();
    net.add_vertex(0);
    for edge in es {
        net.add_nd_edge(edge[0], edge[1]);
    }
    for p in ps {
        net.add_edge(p, n);
    }
    let ans = net.max_flow(0,n);
    println!("{}",ans);
}


type NodeId = usize;

#[derive(Clone)]
struct NodeArena<T> {
    arena: Vec<Node<T>>
}

#[derive(Clone)]
struct Node<T> {
    id: NodeId,
    val: T,
    con: Vec<NodeId>
}

#[derive(Clone)]
struct Graph<T> {
    map: HashMap<T,NodeId>,
    arena: NodeArena<T>
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

impl<T> Graph<T> where T: std::hash::Hash + std::cmp::Eq + Copy {
    fn new() -> Graph<T> {
        Graph {map: HashMap::new(), arena: NodeArena::new()}
    }

    fn contain(&mut self, v: &T) -> bool {
        self.map.contains_key(v)
    }

    fn get_id(&self, v: &T) -> NodeId {
        *self.map.get(v).unwrap()
    }

    fn get_node_id(&self, id: NodeId) -> &Node<T> {
        self.arena.get(id)
    }

    fn get_mut_node_id(&mut self, v: NodeId) -> &mut Node<T> {
        self.arena.get_mut(v)
    }

    fn add_vertex(&mut self, v: T) {
        if !self.contain(&v) {
            let nid = self.arena.alloc(v);
            self.map.insert(v,nid);
        }
    }

    fn add_edge(&mut self, v1: T, v2: T) {
        if !self.contain(&v1) {
            self.add_vertex(v1);
        }
        if !self.contain(&v2) {
            self.add_vertex(v2);
        }
        let id1 = self.get_id(&v1);
        let id2 = self.get_id(&v2);
        self.add_edge_id(id1,id2);
    }

    fn add_nd_edge(&mut self, v1: T, v2: T) {
        if !self.contain(&v1) {
            self.add_vertex(v1);
        }
        if !self.contain(&v2) {
            self.add_vertex(v2);
        }
        let id1 = self.get_id(&v1);
        let id2 = self.get_id(&v2);
        self.add_edge_id(id1,id2);
        self.add_edge_id(id2,id1);
    }

    fn add_edge_id(&mut self, v1: NodeId, v2: NodeId) {
        let n1 = self.get_mut_node_id(v1);
        n1.con.push(v2);
    }

    fn has_edge_id(&self, v1: NodeId, v2: NodeId) -> bool {
        self.get_node_id(v1).con.contains(&v2)
    }

    fn remove_edge_id(&mut self, v1: NodeId, v2: NodeId) {
        let node = self.arena.get_mut(v1);
        for i in 0..node.con.len() {
            if node.con[i] == v2 {
                node.con.remove(i);
                break;
            }
        }
    }

    fn bfs(&self, v1: T, v2: T) -> Option<Vec<NodeId>> {
        let id1 = self.get_id(&v1);
        let id2 = self.get_id(&v2);
        let node = self.get_node_id(id1);
        let mut q = vec![(node,vec![id1])];
        let mut visited: HashSet<NodeId> = HashSet::new();
        visited.insert(id1);
        while q.len() > 0 {
            let state = q.remove(0);
            for id in state.0.con.clone() {
                if visited.insert(id) {
                    let mut path = state.1.to_vec();
                    path.push(id);
                    if id == id2 {
                        return Some(path);
                    }
                    q.push((self.get_node_id(id),path));
                }
            }
        }
        None
    }

    fn max_flow(&self, s: T, t: T) -> usize {
        let mut res = self.clone();
        let mut f = 0;
        while let Some(path) = res.bfs(s,t) {
            f += 1;
            for e in path.windows(2) {
                res.remove_edge_id(e[0],e[1]);
                if !res.has_edge_id(e[1],e[0]) {
                    res.add_edge_id(e[1],e[0]);
                }
            }
        }
        f
    }
}
