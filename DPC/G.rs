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
use std::cmp::max;

fn main() {
    input!{
        n: usize,
        m: usize,
        es: [(usize1, usize1); m]
    }
    let mut tree: Vec<Vec<usize>> = vec![Vec::new(); n];
    for (u, v) in es {
        tree[u].push(v);
    }
    let mut tab: HashMap<(usize, usize), usize> = HashMap::new();
    let mut ans = 0;
    for i in 0..n {
        ans = max(ans, dfs(&tree, i, i, &mut tab));
    }
    println!("{}", ans - 1);
}

fn dfs(t: &Vec<Vec<usize>>, r: usize, p: usize, tab: &mut HashMap<(usize, usize), usize>) -> usize {
    if let Some(&x) = tab.get(&(p, r)) {
        return x;
    }
    let mut tmp = 0;
    for &c in t[r].iter() {
        if c == p {
            continue;
        }
        tmp = max(tmp, dfs(t, c, r, tab));
    }
    tab.insert((p, r), tmp + 1);
    tmp + 1
}
