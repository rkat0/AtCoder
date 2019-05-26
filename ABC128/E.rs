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
use std::iter::FromIterator;
use std::cmp::min;

fn main() {
    input!{
        n: usize,
        q: usize,
        mut v: [(i64, i64, i64); n],
        d: [i64; q]
    }
    v.sort_by(|x, y| y.2.cmp(&x.2));
    let mut set: BTreeSet<i64> = BTreeSet::new();
    set.insert(i64::max_value());
    for i in 0..n {
        let (s, t, x) = v[i];
        v[i] = (s - x, t - x, x);
        set.insert(s - x);
        set.insert(t - x);
    }
    let vs: Vec<i64> = set.iter().cloned().collect();
    let map: BTreeMap<i64, usize> = BTreeMap::from_iter(set.iter().cloned().zip(0..));
    let m = vs.len();
    let mut segt = LSegT::new(m);
    for (s, t, x) in v {
        segt.update(*map.get(&s).unwrap(), *map.get(&t).unwrap() - 1, x);
    }
    for i in 0..q {
        let mut l = 0;
        let mut r = m;
        while l + 1 < r {
            let mid = (l + r) / 2;
            if vs[mid] > d[i] {
                r = mid;
            } else {
                l = mid;
            }
        }
        let ans = segt.get(l);
        println!("{}", if ans == i64::max_value() {-1} else {ans});
    }

}

struct LSegT {
    v: Vec<i64>,
    lazy: Vec<i64>,
    n: usize
}


impl LSegT {
    fn new(n: usize) -> LSegT {
        let mut s = 1;
        while s < n {
            s *= 2;
        }
        LSegT{v: vec![i64::max_value(); 2 * s - 1], lazy: vec![0; 2 * s - 1], n: s}
    }

    fn eval(&mut self, p: usize) {
        if self.lazy[p] == 0 {
            return;
        }
        self.v[p] = min(self.v[p], self.lazy[p]);
        if p < self.n - 1 {
            self.lazy[2 * p + 1] = self.lazy[p];
            self.lazy[2 * p + 2] = self.lazy[p];
        }
        self.lazy[p] = 0;
    }

    fn update(&mut self, l: usize, r: usize, x: i64) {
        let n = self.n;
        self.update_rec(l, r, x, 0, 0, n - 1);
    }

    fn update_rec(&mut self, l: usize, r: usize, x: i64, p: usize, pl: usize, pr: usize) {
        self.eval(p);
        if pl > r || pr < l {
            return;
        } else if l <= pl && pr <= r {
            self.lazy[p] = x;
            self.eval(p);
        } else {
            self.update_rec(l, r, x, 2 * p + 1, pl, (pl + pr - 1) / 2);
            self.update_rec(l, r, x, 2 * p + 2, (pl + pr + 1) / 2, pr);
            self.v[p] = std::cmp::min(self.v[2 * p + 1], self.v[2 * p + 2]);
        }
    }

    fn get(&mut self, idx: usize) -> i64 {
        let n = self.n;
        self.get_rec(idx, idx, 0, 0, n - 1)
    }

    fn get_rec(&mut self, l: usize, r: usize, p: usize, pl: usize, pr: usize) -> i64 {
        self.eval(p);
        if pl > r || pr < l {
            i64::max_value()
        } else if l <= pl && pr <= r {
            self.v[p]
        } else {
            let a = self.get_rec(l, r, 2 * p + 1, pl, (pl + pr - 1) / 2);
            let b = self.get_rec(l, r, 2 * p + 2, (pl + pr + 1) / 2, pr);
            std::cmp::min(a, b)
        }
    }
}
