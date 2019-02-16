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
        mut v: [[i64; 3]; n]
    }
    v.sort_by_key(|x| x[0] + x[1]);
    let mut tab: Vec<Vec<i64>> = vec![vec![-1; 20001]; n + 1];
    tab[0][0] = 0;
    for i in 0..n {
        for j in 0..v[i][1] as usize + 1 {
            if tab[i][j] >= 0 {
                tab[i + 1][j + v[i][0] as usize] = max(tab[i][j + v[i][0] as usize], tab[i][j] + v[i][2]);
            }
        }
        for j in 0..20001 {
            tab[i + 1][j] = max(tab[i + 1][j], tab[i][j]);
        }
    }
    println!("{}", *tab[n].iter().max().unwrap());
}
