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

use std::collections::VecDeque;

fn main() {
    input!{
        n: usize,
        c: i64,
        v: [i64; n]
    }
    let mut q: VecDeque<(i64, i64)> = VecDeque::new();
    let mut tab: Vec<i64> = vec![0; n];
    q.push_back((-2 * v[0], v[0] * v[0]));
    for i in 1..n {
        tab[i] = {
            let &(a, b) = q.front().unwrap();
            a * v[i] + b
        };
        while q.len() > 1 {
            let &(a, b) = q.get(1).unwrap();
            if tab[i] > a * v[i] + b {
                tab[i] = a * v[i] + b;
                q.pop_front();
            } else {
                break;
            }
        }
        tab[i] += v[i] * v[i] + c;
        let a = -2 * v[i];
        let b = tab[i] + v[i] * v[i];
        while q.len() > 1 {
            let l = q.len();
            let &(a1, b1) = q.get(l - 2).unwrap();
            let &(a2, b2) = q.get(l - 1).unwrap();
            if (a1 - a2) * (b - b1) <= (a1 - a) * (b2 - b1) {
                q.pop_back();
            } else {
                break;
            }
        }
        q.push_back((a, b));
    }
    println!("{}", tab[n - 1]);
}
