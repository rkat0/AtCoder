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

use std::cmp::{min, max};

fn main() {
    input!{
        a: usize,
        b: usize,
        q: usize,
        mut s: [i64; a],
        mut t: [i64; b],
        xs: [i64; q]
    }
    s.push(0);
    s.sort();
    t.push(0);
    t.sort();
    for x in xs {
        let sli = bsearch_l(x, &s);
        let tli = bsearch_l(x, &t);
        let ans = if sli == a && tli == b {
            x - min(s[sli], t[tli])
        } else if sli == a {
            if t[tli] >= s[sli] {
                x - s[sli]
            } else if tli == 0 {
                min(x - 2 * s[sli] + t[1], 2 * t[1] - s[sli] - x)
            } else {
                min(min(x - t[tli], x - 2 * s[sli] + t[tli + 1]), 2 * t[tli + 1] - s[sli] - x)
            }
        } else if tli == b {
            if s[sli] >= t[tli] {
                x - t[tli]
            } else if sli == 0 {
                min(x - 2 * t[tli] + s[1], 2 * s[1] - t[tli] - x)
            } else {
                min(min(x - s[sli], x - 2 * t[tli] + s[sli + 1]), 2 * s[sli + 1] - t[tli] - x)
            }
        } else {
            let mut ret = max(s[sli + 1], t[tli + 1]) - x;
            if sli > 0 && tli > 0 {
                ret = min(ret, x - min(s[sli], t[tli]));
            }
            if t[tli + 1] > s[sli + 1] && tli > 0 {
                ret = min(ret, 2 * s[sli + 1] - x - t[tli]);
            }
            if s[sli + 1] > t[tli + 1] && sli > 0 {
                ret = min(ret, 2 * t[tli + 1] - x - s[sli]);
            }
            if t[tli] < s[sli] {
                ret = min(ret, x - 2 * s[sli] + t[tli + 1]);
            }
            if t[tli] > s[sli] {
                ret = min(ret, x - 2 * t[tli] + s[sli + 1]);
            }
            ret
        };
        println!("{}", ans);
    }
}

fn bsearch_l(x: i64, v: &Vec<i64>) -> usize {
    let mut l = 0;
    let mut r = v.len();
    while l + 1 < r {
        let m = (l + r) / 2;
        if v[m] <= x {
            l = m;
        } else {
            r = m;
        }
    }
    l
}
