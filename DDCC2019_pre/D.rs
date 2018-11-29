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

use std::cmp::{min,max};

fn main() {
    input!{
        v: [usize; 29]
    }
    let rems: Vec<(usize,usize)> = v.iter().skip(1).enumerate().map(|(i, x)| (x % (i + 2), i + 2)).collect();
    if let Some((ans, _)) = crt(&rems) {
        if ans <= 1000000000000 && v.iter().enumerate().all(|(i, x)| check_dig_sum(ans, *x, i + 2)) {
            println!("{}", ans);
            return;
        }
    }
    println!("invalid");
}

fn check_dig_sum(mut n: usize, m: usize, d: usize) -> bool {
    let mut s = 0;
    while n > 0 {
        s += n % d;
        n /= d;
    }
    s == m
}

fn gcd(n: usize, m: usize) -> usize {
    let mut l = min(n, m);
    let mut r = max(n, m);
    while l > 0 {
        let t = l;
        l = r % l;
        r = t;
    }
    r
}

fn crt(rems: &Vec<(usize, usize)>) -> Option<(usize, usize)> {
    if rems.len() == 0 {
        return Some((0, 1));
    }
    let (mut r, mut m) = rems[0];
    for i in 1..rems.len() {
        if let Some((x, y)) = crt2(r, m, rems[i].0, rems[i].1) {
            r = x;
            m = y;
        } else {
            return None;
        }
    }
    Some((r, m))
}

fn crt2(r1: usize, m1: usize, r2: usize, m2: usize) -> Option<(usize, usize)> {
    let d = gcd(m1, m2);
    let n2 = m2 / d;
    let l = m1 * n2;
    let k = (max(r1, r2) - min(r1, r2)) / d;
    if let Some((a, _)) = ex_euc(m1 as i64, m2 as i64, d as i64) {
        let mut x = r1 as i64;
        let p = (k as i64 * a) % n2 as i64 * m1 as i64;
        if r1 > r2 {
            x -= p;
        } else {
            x += p;
        }
        Some(((x % l as i64 + l as i64) as usize % l, l))
    } else {
        None
    }
}

fn ex_euc(a: i64, b: i64, c: i64) -> Option<(i64, i64)> {
    if a == 0 && b == 0 {
        return if c == 0 { Some((0,0)) } else { None };
    }
    let d = gcd(a.abs() as usize, b.abs() as usize) as i64;
    if c % d != 0 {
        return None;
    }
    Some(ex_euc_rec(a, b, c))
}

fn ex_euc_rec(a: i64, b: i64, c: i64) -> (i64, i64) {
    if b == 0 {
        (c / a, 0)
    } else {
        let (x, y) = ex_euc_rec(b, a % b, c);
        (y, x - a / b * y)
    }
}
