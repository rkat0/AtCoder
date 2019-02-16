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

use std::collections::{HashSet, VecDeque};

fn main() {
    input!{
        n: usize,
        v: [[usize1; 2]; n - 1]
    }
    if n == 1 {
        println!("2");
        return;
    }
    let d = 1000000007;
    let mut nodes: Vec<HashSet<usize>> = vec![HashSet::new(); n];
    for i in 0..n - 1 {
        nodes[v[i][0]].insert(v[i][1]);
        nodes[v[i][1]].insert(v[i][0]);
    }
    let mut tab: Vec<Vec<usize>> = vec![vec![1, 1]; n];
    let mut ch: Vec<Vec<usize>> = vec![Vec::new(); n];
    let mut ts: Vec<usize> = Vec::new();
    let mut s: VecDeque<usize> = VecDeque::new();
    for i in 0..n {
        if nodes[i].len() == 1 {
            s.push_back(i);
        }
    }

    loop {
        let x = s.pop_front().unwrap();
        ts.push(x);
        if nodes[x].len() == 0 {
            break;
        }
        let y = *nodes[x].iter().nth(0).unwrap();
        nodes[y].remove(&x);
        ch[y].push(x);
        if nodes[y].len() == 1 {
            s.push_back(y);
        }
    }
    for i in ts.iter().clone().map(|x| *x) {
        for j in ch[i].iter().clone().map(|x| *x) {
            tab[i][0] = tab[i][0] * (tab[j][0] + tab[j][1]) % d;
            tab[i][1] = tab[i][1] * tab[j][0] % d;
        }
    }
    let l = *ts.last().unwrap();
    println!("{}", (tab[l][0] + tab[l][1]) % d);
}
