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

use std::cmp::min;

fn main() {
    input!{
        n: usize,
        v: [usize; n]
    }
    let mut dp: Vec<[usize; 4]> = vec![[0, 0, 0, 0]; n + 1];
    let v_o: Vec<usize> = v.iter().clone().map(|x| (x + 1) % 2).collect();
    let v_e: Vec<usize> = v.iter().clone().map(|x| if *x == 0 { 2 } else { x % 2 }).collect();
    for i in 0..n {
        dp[i + 1][0] = dp[i][0] + v[i];
        dp[i + 1][1] = min(dp[i][0], dp[i][1]) + v_e[i];
        dp[i + 1][2] = min(min(dp[i][0], dp[i][1]), dp[i][2]) + v_o[i];
        dp[i + 1][3] = min(min(dp[i][0], dp[i][1]), min(dp[i][2], dp[i][3])) + v_e[i];
    }
    let total = dp[n][0];
    println!("{}", dp.iter().map(|x| min(min(x[0], x[1]), min(x[2], x[3])) + total - x[0]).min().unwrap());
}
