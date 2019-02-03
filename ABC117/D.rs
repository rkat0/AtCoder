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
        k: usize,
        v: [usize; n]
    }
    if k == 0 {
        println!("{}", v.iter().map(|x| *x).sum::<usize>());
        return;
    }
    let m = 40;
    let mut tab: Vec<usize> = vec![0; m];
    let mut dp1: Vec<usize> = vec![0; m];
    for i in 0..m {
        let b = 1 << i;
        let mut c = 0;
        for j in 0..n {
            if v[j] & b > 0 {
                c += 1;
            }
        }
        tab[i] = c << i;
        dp1[i] = max(tab[i], (n << i) - tab[i]);
    }
    for i in 1..m {
        dp1[i] += dp1[i - 1];
    }
    let mut dp2: Vec<usize> = vec![0; m];
    if k & 1 > 0 {
        dp2[0] = dp1[0];
    } else {
        dp2[0] = tab[0];
    }
    for i in 1..m {
        let b = 1 << i;
        if k & b > 0 {
            dp2[i] = max(tab[i] + dp1[i - 1], (n << i) - tab[i] + dp2[i - 1]);
        } else {
            dp2[i] = tab[i] + dp2[i - 1];
        }
    }
    println!("{}", dp2[m - 1]);
}
