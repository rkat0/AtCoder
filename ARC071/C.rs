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

use std::cmp;
use std::collections::BTreeMap;

fn main() {
    input!{
        n: usize,
        ss: [chars; n]
    }
    let mut cmap: BTreeMap<char,usize> = BTreeMap::new();
    for c in ss[0].clone() {
        let count = cmap.entry(c).or_insert(0);
        *count += 1;
    }
    for i in 1..n {
        let mut cmap2: BTreeMap<char,usize> = BTreeMap::new();
        for c in ss[i].clone() {
            let count = cmap2.entry(c).or_insert(0);
            *count += 1;
        }
        for (c,v) in cmap.iter_mut() {
            if let Some(v2) = cmap2.get(&c) {
                *v = cmp::min(*v,*v2);
            } else {
                *v = 0;
            }
        }
    }
    for (k,v) in cmap.iter() {
        let cs = vec![*k;*v];
        print!("{}", cs.into_iter().collect::<String>());
    }
    print!("\n");
}
