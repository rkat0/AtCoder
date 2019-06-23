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

#[allow(unused_imports)]
use std::cmp::{min, max};

fn main() {
    input!{
        n: usize,
        k: usize
    }
    let d = 1000000007;
    let mut ds = divisors(n);
    ds.sort();
    let m = ds.len();
    let mut ans = 0;
    let mut v = vec![0; m];
    for i in 0..m {
        v[i] = pow_m(k, (ds[i] + 1) / 2, d);
        for j in 0..i {
            if ds[i] % ds[j] == 0 {
                v[i] = (v[i] + d - v[j]) % d;
            }
        }
        if ds[i] % 2 == 0 {
            ans += v[i] * ds[i] / 2;
        } else {
            ans += v[i] * ds[i];
        }
        ans %= d;
    }
    println!("{}", ans);
}

fn factor(n: usize) -> (Vec<usize>, Vec<usize>) {
    if n == 1 {
        return (vec![1], vec![1]);
    }
    let mut m = n;
    let mut ps = vec![];
    let mut rs = vec![];
    let mut r = 0;
    while m % 2 == 0 {
        m /= 2;
        r += 1;
    }
    if r > 0 {
        ps.push(2);
        rs.push(r);
        r = 0
    }
    let mut p = 3;
    while p * p <= m {
        while m % p == 0 {
            m /= p;
            r += 1;
        }
        if r > 0 {
            ps.push(p);
            rs.push(r);
            r = 0;
        }
        p += 2;
    }
    if m > 1 {
        ps.push(m);
        rs.push(1);
    }
    (ps, rs)
}

fn divisors(n: usize) -> Vec<usize> {
    let (ps, rs) = factor(n);
    let l = rs.len();
    let mut s = vec![0; l];
    let mut ret = vec![];
    while s[l - 1] <= rs[l - 1] {
        ret.push(ps.iter().zip(s.iter()).map(|(p, &r)| p.pow(r as u32)).product());
        s[0] += 1;
        let mut i = 0;
        while i < l - 1 && s[i] > rs[i] {
            s[i] = 0;
            s[i + 1] += 1;
            i += 1;
        }
    }
    ret
}

fn pow_m(n: usize, mut p: usize, d: usize) -> usize {
    let mut r = n;
    let mut ret = 1;
    while p > 0 {
        if p % 2 == 0 {
            r = r * r % d;
            p /= 2;
        } else {
            ret = ret * r % d;
            p -= 1;
        }
    }
    ret
}
