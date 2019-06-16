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

use std::cmp::max;

fn main() {
    input!{
        n: usize,
        ga: usize,
        sa: usize,
        ba: usize,
        gb: usize,
        sb: usize,
        bb: usize
    }
    let mut v1: Vec<(usize, usize)> = vec![];
    let mut v2: Vec<(usize, usize)> = vec![];
    if ga < gb {
        v1.push((ga, gb));
    } else if ga > gb {
        v2.push((gb, ga));
    }
    if sa < sb {
        v1.push((sa, sb));
    } else if sa > sb {
        v2.push((sb, sa));
    }
    if ba < bb {
        v1.push((ba, bb));
    } else if ba > bb {
        v2.push((bb, ba));
    }
    v1.sort_by(|x, y| (y.1 * x.0).cmp(&(x.1 * y.0)));
    v2.sort_by(|x, y| (y.1 * x.0).cmp(&(x.1 * y.0)));
    let n1 = solve(n, &v1);
    let ans = solve(n1, &v2);
    println!("{}", ans);
}

fn solve(n: usize, v: &Vec<(usize, usize)>) -> usize {
    let k = v.len();
    let mut ret = 0;
    if k == 0 {
        ret = n;
    } else if k == 1 {
        ret = n / v[0].0 * v[0].1 + n % v[0].0;
    } else if k == 2 {
        for i in 0..n / v[0].0 + 1 {
            let r = n - v[0].0 * i;
            ret = max(ret, v[0].1 * i + r / v[1].0 * v[1].1 + r % v[1].0);
        }
    } else {
        for i in 0..n / v[0].0 + 1 {
            let r1 = n - v[0].0 * i;
            for j in 0..r1 / v[1].0 + 1 {
                let r2 = r1 - v[1].0 * j;
                ret = max(ret, v[0].1 * i + v[1].1 * j + r2 / v[2].0 * v[2].1 + r2 % v[2].0);
            }
        }
    }
    ret
}
