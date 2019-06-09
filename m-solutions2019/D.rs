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

use std::cmp::Ordering;

#[derive(Eq, PartialEq, Clone, Debug)]
pub struct State(usize, usize);

impl PartialOrd for State {
    fn partial_cmp(&self, other: &State) -> Option<Ordering> {
        other.1.partial_cmp(&self.1)
    }
}

impl Ord for State {
    fn cmp(&self, other: &State) -> Ordering {
        other.1.cmp(&self.1)
    }
}

use std::collections::{BTreeSet, BinaryHeap};

fn main() {
    input!{
        n: usize,
        v: [(usize1, usize1); n - 1],
        mut c: [usize; n]
    }
    c.sort();
    let mut t: Vec<BTreeSet<usize>> = vec![BTreeSet::new(); n];
    for (a, b) in v {
        t[a].insert(b);
        t[b].insert(a);
    }
    let mut pq: BinaryHeap<State> = BinaryHeap::new();
    let mut done: Vec<bool> = vec![false; n];
    let mut ansv: Vec<usize> = vec![0; n];
    for i in 0..n {
        pq.push(State(i, t[i].len()));
    }
    let mut ans: usize = 0;
    let mut i = 0;
    while !pq.is_empty() && i < n {
        let State(u, d) = pq.pop().unwrap();
        if done[u] {
            continue;
        }
        ans += d * c[i];
        ansv[u] = c[i];
        i += 1;
        let set = t[u].clone();
        for &w in set.iter() {
            if done[w] {
                continue;
            }
            t[w].remove(&u);
            pq.push(State(w, t[w].len()));
        }
        done[u] = true;
    }
    println!("{}", ans);
    print!("{}", ansv[0]);
    for i in 1..n {
        print!(" {}", ansv[i]);
    }
    println!("");
}
