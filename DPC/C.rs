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
        n: usize,
        v: [[usize; 3]; n]
    }
    let mut tab: Vec<Vec<usize>> = vec![vec![0; 3]; n];
    tab[0][0] = v[0][0];
    tab[0][1] = v[0][1];
    tab[0][2] = v[0][2];
    for i in 1..n {
        tab[i][0] = max(tab[i - 1][1], tab[i - 1][2]) + v[i][0];
        tab[i][1] = max(tab[i - 1][0], tab[i - 1][2]) + v[i][1];
        tab[i][2] = max(tab[i - 1][0], tab[i - 1][1]) + v[i][2];
    }
    println!("{}", tab[n - 1].iter().max().unwrap());
}
