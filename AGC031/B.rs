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

use std::collections::BTreeMap;

fn main() {
    input!{
        n: usize,
        v: [usize; n]
    }
    let d = 1000000007;
    let mut map: BTreeMap<usize, Vec<usize>> = BTreeMap::new();
    for i in 0..n {
        map.entry(v[i]).or_insert_with(Vec::new).push(i);
    }
    let css: Vec<Vec<usize>> = map.values().map(|x| x.clone()).collect();
    let mut line: Vec<usize> = (0..n).collect();
    for cs in css {
        for i in 1..cs.len() {
            line[cs[i]] = cs[i - 1];
        }
    }
    let mut dp: Vec<usize> = vec![0; n];
    dp[0] = 1;
    for i in 1..n {
        dp[i] = dp[i - 1];
        if line[i] < i - 1 {
            dp[i] = (dp[i] + dp[line[i]]) % d;
        }
    }
    println!("{}", dp[n - 1]);
}
