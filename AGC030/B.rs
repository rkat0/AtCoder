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

use std::cmp::max;

fn main() {
    input!{
        l: usize,
        n: usize,
        v: [usize; n]
    }
    let mut ans = max(v[n - 1], l - v[0]);
    let mut sl: Vec<usize> = v.iter().clone().scan(0, |ac,&x| {*ac += x; Some(*ac)}).collect();
    let mut sr: Vec<usize> = v.iter().clone().rev().map(|x| l - x).scan(0, |ac,x| {*ac += x; Some(*ac)}).collect();
    sl.insert(0, 0);
    sr.insert(0, 0);
    for i in 1..n {
        let t = if 2 * i > n {
            let t1 = v[i - 1] + 2 * (sl[i - 1] - sl[2 * i - 1 - n] + sr[n - i]);
            let t2 = l - v[i] + 2 * (sl[i] - sl[2 * i - n] + sr[n - i - 1]);
            max(t1, t2)
        } else if 2 * i < n {
            let t1 = v[i - 1] + 2 * (sl[i - 1] + sr[n - i] - sr[n - 2 * i]);
            let t2 = l - v[i] + 2 * (sl[i] + sr[n - i - 1] - sr[n - 2 * i - 1]);
            max(t1, t2)
        } else {
            let t1 = v[i - 1] + 2 * (sl[i - 1] + sr[n - i]);
            let t2 = l - v[i] + 2 * (sl[i] + sr[n - i - 1]);
            max(t1, t2)
        };
        if t > ans {
            ans = t;
        }
    }
    println!("{}", ans);
}
