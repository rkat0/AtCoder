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

use std::cmp::{min,max};

fn main() {
    input!{
        h: usize,
        w: usize,
        s: [chars; h]
    }
    let mut ans = 0;
    let mut rv: Vec<Vec<usize>> = vec![vec![0; h + 1]; h + w - 1];
    let mut lv: Vec<Vec<usize>> = vec![vec![0; h + 1]; h + w - 1];
    for k in 0..h + w - 1 {
        let ii = if k < w { 0 } else { k - w + 1 };
        let ie = if k < h { k } else { h - 1 } + 1;
        for i in ii..ie {
            let jr = k - i;
            rv[k][i+1] = rv[k][i] + if s[i][jr] == '#' { 1 } else { 0 };
            let jl = i + w - k - 1;
            lv[k][i+1] = lv[k][i] + if s[i][jl] == '#' { 1 } else { 0 };
        }
        for i in ie..h {
            rv[k][i+1] = rv[k][i];
            lv[k][i+1] = lv[k][i];
        }
    }
    for k in 0..h + w - 1 {
        let ii = if k < w { 0 } else { k - w + 1 };
        let ie = if k < h { k } else { h - 1 } + 1;
        for i in ii..ie - 1 {
            if s[i][k - i] == '.' {
                continue;
            }
            for j in i + 1..ie {
                if s[j][k - j] == '.' {
                    continue;
                }
                let d = j - i;
                if k >= 2 * d {
                    ans += rv[k - 2 * d][i + 1] - rv[k - 2 * d][max(0, i as i64 - d as i64) as usize];
                }
                if k + 2 * d < h + w - 1 {
                    ans += rv[k + 2 * d][min(j + d + 1, h)] - rv[k + 2 * d][j]
                }
            }
        }
    }
    for k in 0..h + w - 1 {
        let ii = if k < w { 0 } else { k - w + 1 };
        let ie = if k < h { k } else { h - 1 } + 1;
        for i in ii..ie - 1 {
            if s[i][i + w - 1 - k] == '.' {
                continue;
            }
            for j in i + 1..ie {
                if s[j][j + w - 1 - k] == '.' {
                    continue;
                }
                let d = j - i;
                if k >= 2 * d {
                    ans += lv[k - 2 * d][i] - lv[k - 2 * d][max(0, i as i64 - d as i64 + 1) as usize];
                }
                if k + 2 * d < h + w - 1 {
                    ans += lv[k + 2 * d][min(j + d, h)] - lv[k + 2 * d][j + 1];
                }
            }
        }
    }
    println!("{}", ans);
}
