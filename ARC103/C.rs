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
    let mut even: HashMap<usize,usize> = HashMap::new();
    let mut odd: HashMap<usize,usize> = HashMap::new();
    even.insert(0,0);
    odd.insert(0,0);
    for i in 0..n {
        if i % 2 == 0 {
            let c = even.entry(v[i]).or_insert(0);
            *c += 1;
        } else {
            let c = odd.entry(v[i]).or_insert(0);
            *c += 1;
        }
    }
    let mut e: Vec<(usize,usize)> = even.iter().map(|x| (*x.0, *x.1)).collect();
    let mut o: Vec<(usize,usize)> = odd.iter().map(|x| (*x.0, *x.1)).collect();
    e.sort_by(|x,y| y.1.cmp(&x.1));
    o.sort_by(|x,y| y.1.cmp(&x.1));
    if e[0].0 == o[0].0 {
        let c1 = e[0].1 + o[1].1;
        let c2 = e[1].1 + o[0].1;
        println!("{}", n - std::cmp::max(c1, c2));
    } else {
        println!("{}", n - e[0].1 - o[0].1);
    }
}
