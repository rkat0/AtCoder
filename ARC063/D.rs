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
        t: usize,
        v: [usize; n]
    }
    let mut ans = 0;
    let mut bmax = 0;
    let mut vmin = v[0];
    let mut nmin = 1;
    let mut vmax = v[0];
    let mut nmax = 0;
    for i in 1..n {
        if v[i] < vmin {
            if bmax < vmax - vmin {
                bmax = vmax - vmin;
                ans = min(nmin, nmax);
            } else if bmax == vmax - vmin {
                ans += min(nmin, nmax);
            }
            vmin = v[i];
            nmin = 1;
            vmax = v[i];
            nmax = 0;
        } else {
            if v[i] > vmax {
                vmax = v[i];
                nmax = 1;
            } else if v[i] == vmax {
                nmax += 1;
            }
        }
    }
    if bmax < vmax - vmin {
        ans = min(nmin, nmax);
    } else if bmax == vmax - vmin {
        ans += min(nmin, nmax);
    }
    println!("{}", ans);
}
