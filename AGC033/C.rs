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

use std::collections::{BTreeSet, BTreeMap};
use std::cmp::max;

fn main() {
    input!{
        n: usize,
        v: [(usize1, usize1); n - 1]
    }
    let d = tree_diameter(&v);
    println!("{}", if d % 3 == 1 {"Second"} else {"First"});
}

fn tree_diameter(es: &Vec<(usize, usize)>) -> usize {
    let n = es.len() + 1;
    let mut t: Vec<BTreeSet<usize>> = vec![BTreeSet::new(); n];
    for (a, b) in es.iter().cloned() {
        t[a].insert(b);
        t[b].insert(a);
    }
    let mut tab: BTreeMap<(usize, usize), usize> = BTreeMap::new();
    let mut tab2: Vec<[usize; 2]> = vec![[0, 0]; n];
    let _ = dfs(0, 0, &t, &mut tab, &mut tab2);
    for &c in t[0].iter() {
        dfs2(c, 0, &t, &tab, &mut tab2);
    }
    tab2.iter().map(|x| x[0]).max().unwrap()
}

fn dfs(r: usize, p: usize, t: &Vec<BTreeSet<usize>>, tab: &mut BTreeMap<(usize, usize), usize>, tab2: &mut Vec<[usize; 2]>) -> usize {
    let mut d = 1;
    for &c in t[r].iter() {
        if c == p {
            continue;
        }
        let dist = dfs(c, r, t, tab, tab2);
        if dist > tab2[r][0] {
            tab2[r][1] = tab2[r][0];
            tab2[r][0] = dist;
        } else if dist > tab2[r][1] {
            tab2[r][1] = dist;
        }
        d = max(d, dist + 1);
    }
    tab.insert((p, r), d);
    d
}

fn dfs2(r: usize, p: usize, t: &Vec<BTreeSet<usize>>, tab: &BTreeMap<(usize, usize), usize>, tab2: &mut Vec<[usize; 2]>) {
    let d = if tab2[p][0] == *tab.get(&(p, r)).unwrap() {
        tab2[p][1] + 1
    } else {
        tab2[p][0] + 1
    };
    if d > tab2[r][0] {
        tab2[r][1] = tab2[r][0];
        tab2[r][0] = d;
    } else if d > tab2[r][1] {
        tab2[r][1] = d;
    }
    for &c in t[r].iter() {
        if c == p {
            continue;
        }
        dfs2(c, r, t, tab, tab2);
    }
}
