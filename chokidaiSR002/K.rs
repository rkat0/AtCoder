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

use std::collections::{BTreeSet, BTreeMap, VecDeque};
use std::iter::FromIterator;
use std::cmp::min;

fn main() {
    input!{
        n: usize,
        v: [(usize, usize); n]
    }
    let mut set: BTreeSet<usize> = BTreeSet::new();
    for (a, b) in v.iter().cloned() {
        set.insert(a);
        set.insert(b);
    }
    let map: BTreeMap<usize, usize> = BTreeMap::from_iter(set.iter().cloned().zip(0..));
    let m = map.len();
    let mut t: Vec<BTreeMap<usize, usize>> = vec![BTreeMap::new(); m];
    for (a, b) in v.iter().cloned() {
        let ai = *map.get(&a).unwrap();
        let bi = *map.get(&b).unwrap();
        *t[ai].entry(bi).or_insert(0) += 1;
        if ai != bi {
            *t[bi].entry(ai).or_insert(0) += 1;
        }
    }
    let mut used: Vec<bool> = vec![false; m];
    let mut ans = 0;
    for i in 0..m {
        if used[i] {
            continue;
        }
        let mut q: VecDeque<usize> = VecDeque::new();
        q.push_back(i);
        let mut nv = 0;
        let mut ne = 0;
        while !q.is_empty() {
            let r = q.pop_front().unwrap();
            if used[r] {
                continue;
            }
            for (c, d) in t[r].iter() {
                if used[*c] || *c == r {
                    ne += *d;
                    continue;
                }
                q.push_back(*c);
            }
            nv += 1;
            used[r] = true;
        }
        ans += min(nv, ne);
    }
    println!("{}", ans);
}
