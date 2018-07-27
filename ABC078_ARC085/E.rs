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

fn read_vec_char() -> Vec<char> {
    read::<String>().chars().collect()
}

fn read_mat<T: std::str::FromStr>(n: usize) -> Vec<Vec<T>> {
    (0..n).map(|_| read_vec()).collect()
}

fn read_mat_char(n: usize) -> Vec<Vec<char>> {
    (0..n).map(|_| read_vec_char()).collect()
}

fn main() {
    let n = read::<usize>();
    let v = read_vec::<i64>();
    let mut w: HashMap<(usize,usize),usize> = HashMap::new();
    let mut g: Graph<usize> = Graph::new();
    let mut ans = 0;
    for i in 1..n+1 {
        let c = v[i-1];
        if c < 0 {
            g.add_edge(0, i);
            w.insert((0,i), (-c) as usize);
        } else {
            ans += c as usize;
            g.add_edge(i, n+1);
            w.insert((i, n+1), c as usize);
        }
        let mut j = i * 2;
        while j <= n {
            g.add_edge(i, j);
            w.insert((i, j), usize::max_value());
            j += i;
        }
    }
    if g.contain(&0) && g.contain(&(n+1)) {
        ans -= g.max_flow_w(0, n+1, &w);
    }
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
struct Edge<T,W> {
    vs: Node<T>,
    ve: Node<T>,
    w: W
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

    fn alloc_vec(&mut self, values: Vec<T>) -> Vec<NodeId> {
        values.into_iter().map(|v| self.alloc(v)).collect()
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

    fn contain(&self, v: &T) -> bool {
        self.map.contains_key(v)
    }

    fn get_id(&self, v: &T) -> NodeId {
        *self.map.get(v).unwrap()
    }

    fn ids(&self) -> HashSet<NodeId> {
        self.map.values().cloned().collect()
    }

    fn vertices(&self) -> HashSet<T> {
        self.map.keys().cloned().collect()
    }

    fn edges(&self) -> Vec<(NodeId,NodeId)> {
        let ess: Vec<Vec<(NodeId,NodeId)>> = self.map.values().map(|&id|
            self.arena.get(id).con.iter().map(|&v|
                (id,v)
            ).collect()
        ).collect();
        let mut ret: Vec<(NodeId,NodeId)> = Vec::new();
        for mut es in ess {
            ret.append(&mut es);
        }
        ret
    }

    fn get_node_id(&self, id: NodeId) -> &Node<T> {
        self.arena.get(id)
    }

    fn get_node(&self, v: &T) -> &Node<T> {
        self.get_node_id(self.get_id(v))
    }

    fn get_mut_node_id(&mut self, v: NodeId) -> &mut Node<T> {
        self.arena.get_mut(v)
    }

    fn get_mut_node(&mut self, v: &T) -> &mut Node<T> {
        let id = self.get_id(v);
        self.arena.get_mut(id)
    }

    fn add_vertex(&mut self, v: T) -> Option<NodeId> {
        if !self.contain(&v) {
            let nid = self.arena.alloc(v);
            self.map.insert(v,nid);
            Some(nid)
        } else {
            None
        }
    }

    fn add_edge(&mut self, v1: T, v2: T) {
        // if !self.contain(&v1) {
        //     self.add_vertex(v1);
        // }
        // if !self.contain(&v2) {
        //     self.add_vertex(v2);
        // }
        let id1 = self.add_vertex(v1).unwrap_or_else(|| self.get_id(&v1));
        let id2 = self.add_vertex(v2).unwrap_or_else(|| self.get_id(&v2));
        self.add_edge_id(id1,id2);
    }

    fn add_nd_edge(&mut self, v1: T, v2: T) {
        // if !self.contain(&v1) {
        //     self.add_vertex(v1);
        // }
        // if !self.contain(&v2) {
        //     self.add_vertex(v2);
        // }
        let id1 = self.add_vertex(v1).unwrap_or_else(|| self.get_id(&v1));
        let id2 = self.add_vertex(v2).unwrap_or_else(|| self.get_id(&v2));
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

    fn has_edge(&self, v1: &T, v2: &T) -> bool {
        self.has_edge_id(self.get_id(v1),self.get_id(v2))
    }

    fn remove_edge(&mut self, v1: &T, v2: &T) {
        let id1 = self.get_id(v1);
        let id2 = self.get_id(v2);
        self.remove_edge_id(id1, id2);
    }

    fn remove_edge_id(&mut self, v1: NodeId, v2: NodeId) {
        let node = self.arena.get_mut(v1);
        // node.con.remove_item(&v2);     // remove_item() : nightly
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

    fn max_flow_w(&self, s: T, t: T, table: &HashMap<(T,T),usize>) -> usize {
        let mut res = self.clone();
        let mut f = 0;
        let mut w = table.clone();
        while let Some(path) = res.bfs(s,t) {
            let mut min = usize::max_value();
            for e in path.windows(2) {
                let n1 = self.get_node_id(e[0]);
                let n2 = self.get_node_id(e[1]);
                min = std::cmp::min(min,*w.get(&(n1.val,n2.val)).unwrap());
            }
            f += min;
            for e in path.windows(2) {
                let n1 = self.get_node_id(e[0]);
                let n2 = self.get_node_id(e[1]);
                *w.get_mut(&(n1.val,n2.val)).unwrap() -= min;
                *w.entry((n2.val,n1.val)).or_insert(0) += min;
                if *w.get(&(n1.val,n2.val)).unwrap() == 0 {
                    res.remove_edge_id(e[0],e[1]);
                }
                if !res.has_edge_id(e[1],e[0]) {
                    res.add_edge_id(e[1],e[0]);
                }
            }
        }
        f
    }
}
