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
        v: [String; 2 * n - 1]
    }
    let mut tab = vec![vec![i64::min_value(); 3]; n];
    tab[0][0] = v[0].parse::<i64>().unwrap();
    for i in 1..n {
        let vi = v[2 * i].parse::<i64>().unwrap();
        if v[2 * i - 1] == "+" {
            tab[i][0] = max(tab[i - 1][0], tab[i - 1][1]) + vi;
            if max(tab[i - 1][1], tab[i - 1][2]) > i64::min_value() {
                tab[i][1] = max(tab[i - 1][1], tab[i - 1][2]) - vi;
                tab[i][2] = tab[i - 1][2] + vi;
            }
        } else {
            tab[i][1] = max(tab[i - 1][0], tab[i - 1][1]) - vi;
            tab[i][2] = max(tab[i - 1][1], tab[i - 1][2]) + vi;
        }
    }
    println!("{}", tab[n-1].iter().max().unwrap());
}
