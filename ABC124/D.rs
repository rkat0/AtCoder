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
        k: usize,
        s: chars
    }
    let mut ls: Vec<usize> = Vec::new();
    let mut c = 0;
    let mut prev = '1';
    for x in s {
        if x == prev {
            c += 1;
        } else {
            ls.push(c);
            c = 1;
            prev = x;
        }
    }
    ls.push(c);
    if prev == '0' {
        ls.push(0);
    }
    if 2 * k + 1 >= ls.len() {
        println!("{}", n);
        return;
    }
    let mut sum: usize = ls.iter().take(2 * k + 1).sum();
    let mut ans = sum;
    for i in 0..(ls.len() - 1) / 2 - k {
        sum += ls[2 * (i + k) + 1] + ls[2 * (i + k + 1)];
        sum -= ls[2 * i] + ls[2 * i + 1];
        ans = max(ans, sum);
    }
    println!("{}", ans);
}
