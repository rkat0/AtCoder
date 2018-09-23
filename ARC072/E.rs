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

fn main() {
    input!{
        n: usize,
        d: usize,
        ds: [usize; n],
        q: usize,
        qs: [usize; q]
    }
    let mut p: Vec<usize> = ds.iter().scan(d, |ac,&x| {*ac = cmp::min(*ac,(*ac as i64 - x as i64).abs() as usize); Some(*ac)}).collect();
    p.insert(0,d);
    let mut min: Vec<usize> = vec![1;n+1];
    for i in 1..n+1 {
        min[n-i] = min[n-i+1];
        if min[n-i+1] * 2 > ds[n-i] {
            min[n-i] += ds[n-i];
        }
    }
    for x in qs {
        println!("{}",if p[x-1] >= min[x] {"YES"} else {"NO"});
    }
}
