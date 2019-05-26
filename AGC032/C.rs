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

use std::collections::HashSet;

fn main() {
    input!{
        n: usize,
        m: usize,
        v: [(usize1, usize1); m]
    }
    let mut g: Vec<HashSet<usize>> = vec![HashSet::new(); n];
    for &(a, b) in v.iter() {
        g[a].insert(b);
        g[b].insert(a);
    }
    let mut d4: Vec<usize> = Vec::new();
    let mut f6 = false;
    for i in 0..n {
        let d = g[i].len();
        if d % 2 != 0 {
            println!("No");
            return;
        } else if d >= 6 {
            f6 = true;
        } else if d == 4 {
            d4.push(i);
        }
    }
    if f6 {
        println!("Yes");
        return;
    }
    if d4.len() > 2 {
        println!("Yes");
        return;
    } else if d4.len() < 2 {
        println!("No");
        return;
    }
    let ans = flow(&g, d4[0], d4[1]);
    println!("{}", if ans {"Yes"} else {"No"});
}

fn flow(g: &Vec<HashSet<usize>>, s: usize, t: usize) -> bool {
    for &v in g[s].iter() {
        let mut ps: HashSet<usize> = vec![s].iter().cloned().collect();
        if !dfs(&g, v, t, &mut ps) {
            return true;
        }
    }
    false
}

fn dfs(g: &Vec<HashSet<usize>>, s: usize, t: usize, ps: &mut HashSet<usize>) -> bool {
    if s == t {
        return true;
    }
    for &c in g[s].iter() {
        if !ps.insert(c) {
            continue;
        }
        if dfs(g, c, t, ps) {
            return true;
        }
        ps.remove(&s);
    }
    return false;
}
