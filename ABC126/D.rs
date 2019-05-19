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

use std::collections::HashMap;

fn main() {
    input!{
        n: usize,
        es: [(usize1, usize1, usize); n - 1]
    }
    let mut t: HashMap<usize, Vec<(usize, usize)>> = HashMap::new();
    for (u, v, w) in es.iter().cloned() {
        t.entry(u).or_insert(Vec::new()).push((v, w % 2));
        t.entry(v).or_insert(Vec::new()).push((u, w % 2));
    }
    let mut ans = vec![0; n];
    dfs(0, 0, &t, &mut ans);
    for i in 0..n {
        println!("{}", ans[i]);
    }
}

fn dfs(n: usize, p: usize, t: &HashMap<usize, Vec<(usize, usize)>>, c: &mut Vec<usize>) {
    for (v, w) in t.get(&n).unwrap().iter().cloned() {
        if v == p {
            continue;
        }
        if w == 0 {
            c[v] = c[n];
        } else {
            c[v] = c[n] ^ 1;
        }
        dfs(v, n, t, c);
    }
}
