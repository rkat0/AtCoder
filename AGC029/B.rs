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

use std::collections::HashMap;

fn main() {
    input!{
        n: usize,
        v: [usize; n]
    }
    let mut w = v;
    w.sort_by(|a, b| b.cmp(&a));
    let mut map = HashMap::new();
    for i in 0..n {
        let x = map.entry(w[i]).or_insert(0);
        *x += 1;
    }
    let mut p = 1 << 31;
    let mut ans = 0;
    for i in 0..n {
        {
            let x = map.get_mut(&w[i]).unwrap();
            if *x == 0 {
                continue;
            } else {
                *x -= 1;
            }
        }

        while p >> 1 > w[i] {
            p >>= 1;
        }
        if let Some(y) = map.get_mut(&(p - w[i])) {
            if *y > 0 {
                ans += 1;
                *y -= 1;
            }
        }
    }
    println!("{}", ans);
}
