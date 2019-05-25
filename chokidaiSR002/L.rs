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

fn main() {
    input!{
        n: usize,
        mut v: [(usize, usize); n]
    }
    for i in 0..n {
        if v[i].0 > v[i].1 {
            v[i] = (v[i].1, v[i].0);
        }
    }
    v.sort_by(|x, y| if x.0 == y.0 {y.1.cmp(&x.1)} else {x.0.cmp(&y.0)});
    let mut tab: Vec<(usize, usize)> = vec![(usize::max_value(), usize ::max_value()); n + 1];
    tab[0] = (0, 0);
    for (x, y) in v {
        let mut l = 0;
        let mut r = n;
        while l + 1 < r {
            let m = (l + r) / 2;
            if x <= tab[m].0 || y <= tab[m].1 {
                r = m;
            } else {
                l = m;
            }
        }
        tab[r] = (x, y);
    }
    for i in (0..n + 1).rev() {
        if tab[i].0 != usize::max_value() {
            println!("{}", i);
            return;
        }
    }
}
