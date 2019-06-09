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

use std::cmp::{max, min};

fn main() {
    input!{
        n: usize,
        s: chars,
        es: [(usize1, usize1); n - 1]
    }
    let mut t = vec![vec![]; n];
    for (u, v) in es {
        t[u].push(v);
        t[v].push(u);
    }
    let mut ans = usize::max_value();
    for i in 0..n {
        let (x, d, _) = dfs(i, i, &t, &s);
        if d == 0 {
            ans = min(ans, x);
        }
    }
    if ans == usize::max_value() {
        println!("-1");
    } else {
        println!("{}", ans);
    }
}

fn dfs(r: usize, p: usize, t: &Vec<Vec<usize>>, s: &Vec<char>) -> (usize, usize, usize) {
    let mut n = 0;
    let mut sum = 0;
    let mut m = 0;
    let mut v = s[r].to_digit(10u32).unwrap() as usize;
    for c in t[r].clone() {
        if c == p {
            continue;
        }
        let cs = dfs(c, r, t, s);
        n += cs.0;
        sum += cs.1 + cs.2;
        m = max(m, cs.0 + cs.1 + cs.2);
        v += cs.2;
    }
    if 2 * m >= sum + 2 * n {
        (2 * n + sum - m, 2 * m - sum - 2 * n, v)
    } else {
        (n + sum / 2, sum % 2, v)
    }
}
