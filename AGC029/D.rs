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

use std::collections::HashSet;

fn main() {
    input!{
        h: usize,
        w: usize,
        n: usize,
        ps: [[usize; 2]; n]
    }
    if h == 1 {
        println!("1");
        return;
    }
    let mut tab = vec![Vec::new(); w];
    let mut set = HashSet::new();
    for i in 0..n {
        if ps[i][0] < ps[i][1] {
            continue;
        }
        tab[ps[i][1] - 1].push(ps[i][0]);
        set.insert(ps[i].clone());
    }
    let mut r = vec![0; w];
    let mut pos = 0;
    for i in 0..w {
        pos += 1;
        while set.contains(&vec![pos, i + 1]) {
            pos += 1;
        }
        let mut min = h + 1;
        for j in 0..tab[i].len() {
            if tab[i][j] > pos && min > tab[i][j] {
                min = tab[i][j];
            }
        }
        r[i] = min;
    }
    println!("{}", r.iter().min().unwrap() - 1);
}
