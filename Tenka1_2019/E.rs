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
use std::iter::FromIterator;

fn main() {
    input!{
        n: usize,
        mut v: [i64; n + 1]
    }
    v.reverse();
    let a0 = v[0];
    let mut ans = BTreeSet::from_iter(factor(gcdv(&v)).into_iter());
    let ps = primes(n as i64 + 1);
    for &p in ps.iter() {
        if a0 % p != 0 {
            continue;
        }
        let mut cs: Vec<i64> = vec![0; p as usize - 1];
        for i in 0..n + 1 {
            cs[i % (p - 1) as usize] += v[i] % p;
        }
        if cs.iter().all(|x| x % p == 0) {
            ans.insert(p);
        }
    }
    if ans.len() > 0 {
        for a in ans.iter() {
            println!("{}", a);
        }
    } else {
        println!("");
    }
}

fn factor(n: i64) -> Vec<i64> {
    let mut m = n.abs();
    let mut ret = vec![];
    if m % 2 == 0 {
        ret.push(2);
        while m % 2 == 0 {
            m /= 2;
        }
    }
    let mut p = 3;
    while m >= p * p {
        if m % p == 0 {
            ret.push(p);
            while m % p == 0 {
                m /= p;
            }
        }
        p += 2;
    }
    if m > 1 {
        ret.push(m);
    }
    ret
}

use std::cmp::{min,max};

fn gcdv(v: &Vec<i64>) -> i64 {
    let mut ret = 0;
    for &n in v.iter() {
        ret = gcd(ret, n);
    }
    ret
}

fn gcd(n: i64, m: i64) -> i64 {
    let mut l = min(n.abs(), m.abs());
    let mut r = max(n.abs(), m.abs());
    while l > 0 {
        let t = l;
        l = r % l;
        r = t;
    }
    r
}

fn primes(n: i64) -> Vec<i64> {
    if n < 2 {
        return vec![];
    }
    let mut ps = vec![2];
    let mut p = 3;
    while p <= n {
        if ps.iter().all(|x| p % x != 0) {
            ps.push(p);
        }
        p += 2;
    }
    ps
}
