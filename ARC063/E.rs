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

#[allow(unused_imports)]
use std::cmp::{min, max};
#[allow(unused_imports)]
use std::collections::{HashMap, HashSet};

fn main() {
    input!{
        n: usize,
        es: [(usize1, usize1); n - 1],
        k: usize,
        vp: [(usize1, i64); k]
    }
    let mut t = vec![vec![]; n];
    for &(a, b) in &es {
        t[a].push(b);
        t[b].push(a);
    }
    let mut tab = vec![None; n];
    for &(v, p) in &vp {
        tab[v] = Some((p, p + 1));
    }
    dfs(vp[0].0, vp[0].0, &t, &mut tab, vp[0].1 % 2);
    if tab[vp[0].0].unwrap().0 >= tab[vp[0].0].unwrap().1 {
        println!("No");
        return;
    }
    println!("Yes");
    let mut ans = vec![0; n];
    ans[vp[0].0] = vp[0].1;
    dfs2(vp[0].0, vp[0].0, &t, &tab, &mut ans);
    for i in 0..n {
        println!("{}", ans[i]);
    }
}

fn dfs(r: usize, p: usize, t: &Vec<Vec<usize>>, tab: &mut Vec<Option<(i64, i64)>>, f: i64) {
    if let Some((a, _)) = tab[r] {
        if a.abs() % 2 != f {
            tab[r] = Some((a, a));
            return;
        }
    }
    let mut range = tab[r];
    for &c in &t[r] {
        if c == p {
            continue;
        }
        dfs(c, r, t, tab, 1 - f);
        if let Some((a, b)) = tab[c] {
            if a >= b {
                tab[r] = tab[c];
                return;
            }
            range = Some(range.map_or((a - 1, b + 1), |x| (max(a - 1, x.0), min(b + 1, x.1))));
        }
    }
    tab[r] = range;
}

fn dfs2(r: usize, p: usize, t: &Vec<Vec<usize>>, tab: &Vec<Option<(i64, i64)>>, v: &mut Vec<i64>) {
    for &c in &t[r] {
        if c == p {
            continue;
        }
        if let Some((a, b)) = tab[c] {
            if a <= v[r] + 1 && v[r] + 1 < b {
                v[c] = v[r] + 1;
            } else {
                v[c] = v[r] - 1;
            }
        } else {
            v[c] = v[r] + 1;
        }
        dfs2(c, r, t, tab, v);
    }
}
