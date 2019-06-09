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
pub struct State(usize, usize, usize);

impl PartialOrd for State {
    fn partial_cmp(&self, other: &State) -> Option<Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl Ord for State {
    fn cmp(&self, other: &State) -> Ordering {
        self.0.cmp(&other.0)
    }
}

use std::cmp::max;

fn main() {
    input!{
        n: usize,
        x: usize,
        v: [(usize, usize, usize); n]
    }
    let mut maxes: Vec<(usize, usize)> = vec![(0, 0); n];
    let mut score: usize = 0;
    for i in 0..n {
        score += v[i].0 * v[i].1;
        maxes[i] = (v[i].0 * v[i].1 + (x - v[i].0) * v[i].2, i);
    }
    maxes.sort_by(|x, y| y.0.cmp(&x.0));
    let mut ac: Vec<usize> = vec![0; n + 1];
    for i in 0..n {
        ac[i + 1] = ac[i] + maxes[i].0;
    }
    let mut order = vec![0; n];
    for i in 0..n {
        order[maxes[i].1] = i;
    }
    let mut l: usize = 0;
    let mut r: usize = v.iter().map(|x| x.0).sum();
    while l + 1 < r {
        let m = (l + r) / 2;
        let k = m % x;
        let q = m / x;
        let mut smax = 0;
        for i in 0..n {
            let mut s = ac[q];
            if order[i] < q {
                s -= maxes[order[i]].0 - maxes[q].0;
            }
            if k > v[i].0 {
                s += v[i].0 * v[i].1 + (k - v[i].0) * v[i].2;
            } else {
                s += k * v[i].1;
            }
            smax = max(smax, s);
        }
        if smax >= score {
            r = m;
        } else {
            l = m;
        }
    }
    println!("{}", r);
}
