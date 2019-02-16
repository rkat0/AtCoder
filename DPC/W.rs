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

use std::collections::HashMap;

fn main() {
    input!{
        n: usize,
        m: usize,
        v: [(usize, usize, i64); m]
    }
    let mut map: HashMap<usize, Vec<(usize, i64)>> = HashMap::new();
    for (l, r, a) in v {
        map.entry(r).or_insert(Vec::new()).push((l, a));
    }
    let mut seg: LSegT = LSegT::new(n + 1);
    for i in 1..n + 1 {
        let m = seg.max(0, i - 1);
        seg.add(i, i, m);
        if let Some(x) = map.get(&i) {
            for &(l, a) in x {
                seg.add(l, i, a);
            }
        }
    }
    println!("{}", seg.max(0, n));
}

#[derive(Debug)]
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
        LSegT{v: vec![0; 2 * s - 1], lazy: vec![0; 2 * s - 1], n: s}
    }

    fn eval(&mut self, p: usize) {
        if self.lazy[p] == 0 {
            return;
        }
        self.v[p] += self.lazy[p];
        if p < self.n - 1 {
            self.lazy[2 * p + 1] += self.lazy[p];
            self.lazy[2 * p + 2] += self.lazy[p];
        }
        self.lazy[p] = 0;
    }

    fn add(&mut self, l: usize, r: usize, x: i64) {
        let n = self.n;
        self.add_rec(l, r, x, 0, 0, n - 1);
    }

    fn add_rec(&mut self, l: usize, r: usize, x: i64, p: usize, pl: usize, pr: usize) {
        self.eval(p);
        if pl > r || pr < l {
            return;
        } else if l <= pl && pr <= r {
            self.lazy[p] += x;
            self.eval(p);
        } else {
            self.add_rec(l, r, x, 2 * p + 1, pl, (pl + pr - 1) / 2);
            self.add_rec(l, r, x, 2 * p + 2, (pl + pr + 1) / 2, pr);
            self.v[p] = std::cmp::max(self.v[2 * p + 1], self.v[2 * p + 2]);
        }
    }

    fn max(&mut self, l: usize, r: usize) -> i64 {
        let n = self.n;
        self.max_rec(l, r, 0, 0, n - 1)
    }

    fn max_rec(&mut self, l: usize, r: usize, p: usize, pl: usize, pr: usize) -> i64 {
        self.eval(p);
        if pl > r || pr < l {
            i64::min_value()
        } else if l <= pl && pr <= r {
            self.v[p]
        } else {
            let a = self.max_rec(l, r, 2 * p + 1, pl, (pl + pr - 1) / 2);
            let b = self.max_rec(l, r, 2 * p + 2, (pl + pr + 1) / 2, pr);
            std::cmp::max(a, b)
        }
    }
}
