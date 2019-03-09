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

use std::collections::BTreeSet;

fn main() {
    input!{
        _n: usize,
        q: usize,
        qs: [[usize]; q]
    }
    let d = 1000000007;
    let mut xs: BTreeSet<usize> = BTreeSet::new();
    for q in qs.iter() {
        xs.insert(q[0]);
        if q.len() == 1 {
            xs.insert(q[0] + 1);
        } else {
            xs.insert(q[1] + 1);
        }
    }
    let xs: Vec<usize> = xs.into_iter().collect();
    let mut fibs: Vec<[usize; 4]> = Vec::with_capacity(xs.len() - 1);
    for i in 0..xs.len() - 1 {
        let len = xs[i + 1] - xs[i];
        let f = fib3_m(len, d);
        fibs.push([f[0], f[1], f[1], f[2]]);
    }
    let mut segt: SegT = SegT::new(fibs, d);
    for qi in qs {
        if qi.len() == 1 {
            segt.update(xs.binary_search(&qi[0]).unwrap());
        } else {
            let li = xs.binary_search(&qi[0]).unwrap();
            let ri = xs.binary_search(&(qi[1] + 1)).unwrap();
            println!("{}", segt.get(li, ri)[1]);
        }
    }
}

fn fib3_m(mut n: usize, d: usize) -> [usize; 3] {
    if n == 0 {
        return [1, 0, 0];
    }
    let mut r = [1, 1, 0];
    let mut ret = [1, 0, 1];
    while n > 0 {
        if n % 2 == 0 {
            r = [(r[0] * r[0] + r[1] * r[1]) % d, r[1] * (r[0] + r[2]) % d, (r[1] * r[1] + r[2] * r[2]) % d];
            n /= 2;
        } else {
            ret = [(ret[0] * r[0] + ret[1] * r[1]) % d, (ret[0] * r[1] + ret[1] * r[2]) % d, (ret[1] * r[1] + ret[2] * r[2]) % d];
            n -= 1;
        }
    }
    ret
}

fn matmul_m(l: [usize; 4], r: [usize; 4], d: usize) -> [usize; 4] {
    let p0 = (l[0] * r[0] + l[1] * r[2]) % d;
    let p1 = (l[0] * r[1] + l[1] * r[3]) % d;
    let p2 = (l[2] * r[0] + l[3] * r[2]) % d;
    let p3 = (l[2] * r[1] + l[3] * r[3]) % d;
    [p0, p1, p2, p3]
}

#[derive(Debug)]
struct SegT {
    v: Vec<[usize; 4]>,
    n: usize,
    d: usize
}

impl SegT {
    fn new(xs: Vec<[usize; 4]>, d: usize) -> SegT {
        let l = xs.len();
        let mut s = 1;
        while s < l {
            s *= 2;
        }
        let mut v: Vec<[usize; 4]> = vec![[0, 0, 0, 0]; 2 * s - 1];
        for i in 0..xs.len() {
            v[s - 1 + i] = xs[i];
        }
        let mut segt = SegT{v: v, n: s, d: d};
        for i in (0..s - 1).rev() {
            segt.v[i] = matmul_m(segt.v[2 * i + 1], segt.v[2 * i + 2], d);
        }
        segt
    }

    fn update(&mut self, i: usize) {
        let n = self.n;
        self.update_rec(i, 0, 0, n);
    }

    fn update_rec(&mut self, i: usize, p: usize, pl: usize, pr: usize) {
        if pl > i || pr <= i {
            return;
        } else if p < self.n - 1 {
            let l = 2 * p + 1;
            let r = 2 * p + 2;
            self.update_rec(i, l, pl, (pl + pr) / 2);
            self.update_rec(i, r, (pl + pr) / 2, pr);
            self.v[p] = matmul_m(self.v[l], self.v[r], self.d);
        } else {
            if self.v[p][0] == 0 {
                self.v[p][0] = 1;
                self.v[p][1] = 1;
            } else {
                self.v[p][0] = 0;
                self.v[p][1] = 0;
            }
        }
    }

    fn get(&mut self, l: usize, r: usize) -> [usize; 4] {
        if l > r {
            return [0, 0, 0, 0];
        }
        let n = self.n;
        self.get_rec(l, r, 0, 0, n)
    }

    fn get_rec(&mut self, l: usize, r: usize, p: usize, pl: usize, pr: usize) -> [usize; 4] {
        if pl >= r || pr <= l {
            [1, 0, 0, 1]
        } else if l <= pl && pr <= r {
            self.v[p]
        } else {
            let a = self.get_rec(l, r, 2 * p + 1, pl, (pl + pr) / 2);
            let b = self.get_rec(l, r, 2 * p + 2, (pl + pr) / 2, pr);
            matmul_m(a, b, self.d)
        }
    }
}
