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

use std::collections::{BTreeSet, HashMap};

fn main() {
    input!{
        n: usize,
        v: [[usize; 4]; n]
    }
    let mut ye: BTreeSet<usize> = BTreeSet::new();
    let mut yo: BTreeSet<usize> = BTreeSet::new();
    for s in v.iter().clone() {
        if s[3] & 1 == 0 {
            ye.insert(s[3]);
            yo.insert(s[3] + 1);
        } else {
            yo.insert(s[3]);
            ye.insert(s[3] + 1);
        }
        if (s[3] + s[1]) & 1 == 0 {
            ye.insert(s[3] + s[1]);
            yo.insert(s[3] + s[1] + 1);
        } else {
            yo.insert(s[3] + s[1]);
            ye.insert(s[3] + s[1] + 1);
        }
    }
    let ye: Vec<usize> = ye.into_iter().map(|x| x >> 1).collect();
    let yo: Vec<usize> = yo.into_iter().map(|x| x >> 1).collect();
    let mut qee: Vec<(usize, usize, usize)> = Vec::new();
    let mut qoo: Vec<(usize, usize, usize)> = Vec::new();
    let mut qeo: Vec<(usize, usize, usize)> = Vec::new();
    let mut qoe: Vec<(usize, usize, usize)> = Vec::new();
    for (r, rh, c, cw) in v.iter().map(|x| (x[2], x[0] + x[2], x[3], x[1] + x[3])) {
        let re = (r + 1) >> 1;
        let ro = r >> 1;
        let rhe = (rh + 1) >> 1;
        let rho = rh >> 1;
        let ce = (c + 1) >> 1;
        let co = c >> 1;
        let cwe = (cw + 1) >> 1;
        let cwo = cw >> 1;
        if (r + c) & 1 == 0 {
            qee.push((re, ce, cwe));
            qee.push((rhe, ce, cwe));
            qoo.push((ro, co, cwo));
            qoo.push((rho, co, cwo));
        } else {
            qeo.push((re, co, cwo));
            qeo.push((rhe, co, cwo));
            qoe.push((ro, ce, cwe));
            qoe.push((rho, ce, cwe));
        }
    }
    qee.sort_by_key(|x| x.0);
    qoo.sort_by_key(|x| x.0);
    qeo.sort_by_key(|x| x.0);
    qoe.sort_by_key(|x| x.0);
    let ans: usize = solve(&ye, &qee) + solve(&yo, &qoo) + solve(&yo, &qeo) + solve(&ye, &qoe);
    println!("{}", ans);
}

fn solve(xs: &Vec<usize>, qs: &Vec<(usize, usize, usize)>) -> usize {
    let n = xs.len();
    let mut map: HashMap<usize, usize> = HashMap::new();
    for i in 0..n {
        map.insert(xs[i], i);
    }
    let mut segt: LSegT = LSegT::new(&xs);
    let mut ret = 0;
    let mut prevh = 0;
    for &(h, l, r) in qs {
        if h > prevh {
            ret += segt.get(0, n - 1) * (h - prevh);
            prevh = h;
        }
        let &li = map.get(&l).unwrap();
        let &ri = map.get(&r).unwrap();
        segt.update(li, ri);
    }
    ret
}

struct LSegT {
    v: Vec<usize>,
    lazy: Vec<bool>,
    total: Vec<usize>,
    n: usize
}

impl LSegT {
    fn new(xs: &Vec<usize>) -> LSegT {
        let l = xs.len();
        let mut s = 1;
        while s < l {
            s *= 2;
        }
        let mut total = vec![0; 2 * s - 1];
        for i in 0..l - 1 {
            total[i + s - 1] = xs[i + 1] - xs[i];
        }
        for i in 2..s + 1 {
            let p = s - i;
            total[p] = total[2 * p + 1] + total[2 * p + 2];
        }
        LSegT{v: vec![0; 2 * s - 1], lazy: vec![false; 2 * s - 1], total: total, n: s}
    }

    fn eval(&mut self, p: usize) {
        if self.lazy[p] == false {
            return;
        }
        self.v[p] = self.total[p] - self.v[p];
        if p < self.n - 1 {
            self.lazy[2 * p + 1] = !self.lazy[2 * p + 1];
            self.lazy[2 * p + 2] = !self.lazy[2 * p + 2];
        }
        self.lazy[p] = false;
    }

    fn update(&mut self, l: usize, r: usize) {
        let n = self.n;
        self.update_rec(l, r, 0, 0, n);
    }

    fn update_rec(&mut self, l: usize, r: usize, p: usize, pl: usize, pr: usize) {
        self.eval(p);
        if pl >= r || pr <= l {
            return;
        } else if l <= pl && pr <= r {
            self.lazy[p] = !self.lazy[p];
            self.eval(p);
        } else {
            self.update_rec(l, r, 2 * p + 1, pl, (pl + pr) / 2);
            self.update_rec(l, r, 2 * p + 2, (pl + pr) / 2, pr);
            self.v[p] = self.v[2 * p + 1] + self.v[2 * p + 2];
        }
    }

    fn get(&mut self, l: usize, r: usize) -> usize {
        let n = self.n;
        self.get_rec(l, r, 0, 0, n)
    }

    fn get_rec(&mut self, l: usize, r: usize, p: usize, pl: usize, pr: usize) -> usize {
        self.eval(p);
        if pl >= r || pr <= l {
            0
        } else if l <= pl && pr <= r {
            self.v[p]
        } else {
            let a = self.get_rec(l, r, 2 * p + 1, pl, (pl + pr) / 2);
            let b = self.get_rec(l, r, 2 * p + 2, (pl + pr) / 2, pr);
            a + b
        }
    }
}
