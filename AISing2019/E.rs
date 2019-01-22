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

use std::cmp::min;

fn main() {
    input!{
        n: usize,
        a: [i64; n],
        uv: [[usize1; 2]; n - 1]
    }
    let mut tr: Vec<Vec<usize>> = vec![Vec::new(); n];
    for e in uv {
        tr[e[0]].push(e[1]);
        tr[e[1]].push(e[0]);
    }
    let mut tab1: Vec<Vec<i64>> = vec![vec![i64::max_value(); n]; n];
    let mut tab2: Vec<Vec<i64>> = vec![vec![i64::max_value(); n]; n];
    dfs(&tr, &a, 0, 0, &mut tab1, &mut tab2);
    for i in 0..n {
        if tab1[0][i] < i64::max_value() || tab2[0][i] < 0 {
            println!("{}", i);
            return;
        }
    }
}

fn dfs(t: &Vec<Vec<usize>>, a: &Vec<i64>, r: usize, p: usize, tab1: &mut Vec<Vec<i64>>, tab2: &mut Vec<Vec<i64>>) -> usize {
    let n = t.len();
    let mut nc = 1;
    if a[r] > 0 {
        tab1[r][0] = a[r];
    }
    tab2[r][0] = a[r];
    for &c in t[r].iter() {
        if c == p {
            continue;
        }
        let mut tab3: Vec<i64> = vec![i64::max_value(); n];
        let mut tab4: Vec<i64> = vec![i64::max_value(); n];
        let ncs = dfs(t, a, c, r, tab1, tab2);
        for i in 0..nc {
            if tab1[r][i] == i64::max_value() {
                continue;
            }
            for j in 0..ncs {
                if tab1[c][j] < i64::max_value() {
                    tab3[i + j] = min(tab3[i + j], tab1[r][i] + tab1[c][j]);
                    tab3[i + j + 1] = min(tab3[i + j + 1], tab1[r][i]);
                }
                if tab2[c][j] < i64::max_value() {
                    tab4[i + j] = min(tab4[i + j], tab1[r][i] + tab2[c][j]);
                    if tab2[c][j] < 0 {
                        tab3[i + j + 1] = min(tab3[i + j + 1], tab1[r][i]);
                    }
                }
            }
        }
        for i in 0..nc {
            if tab2[r][i] == i64::max_value() {
                continue;
            }
            for j in 0..ncs {
                if tab1[c][j] < i64::max_value() {
                    tab4[i + j] = min(tab4[i + j], tab2[r][i] + tab1[c][j]);
                    tab4[i + j + 1] = min(tab4[i + j + 1], tab2[r][i]);
                }
                if tab2[c][j] < i64::max_value() {
                    tab4[i + j] = min(tab4[i + j], tab2[r][i] + tab2[c][j]);
                    if tab2[c][j] < 0 {
                        tab4[i + j + 1] = min(tab4[i + j + 1], tab2[r][i]);
                    }
                }
            }
        }
        nc += ncs;
        tab1[r] = tab3;
        tab2[r] = tab4;
    }
    nc
}
