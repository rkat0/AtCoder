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

use std::cmp::max;

fn main() {
    input!{
        h: usize,
        w: usize,
        v: [chars; h]
    }
    let mut tabh: Vec<Vec<usize>> = vec![vec![0; w]; h];
    for i in 0..h {
        if v[i][0] == '.' {
            tabh[i][0] = 1;
        }
        for j in 1..w {
            if v[i][j] == '.' {
                tabh[i][j] = tabh[i][j - 1] + 1;
            }
        }
        for j in (0..w - 1).rev() {
            if v[i][j] == '.' && v[i][j + 1] == '.' {
                tabh[i][j] = tabh[i][j + 1];
            }
        }
    }
    let mut tabw: Vec<Vec<usize>> = vec![vec![0; w]; h];
    for j in 0..w {
        if v[0][j] == '.' {
            tabw[0][j] = 1;
        }
        for i in 1..h {
            if v[i][j] == '.' {
                tabw[i][j] = tabw[i - 1][j] + 1;
            }
        }
        for i in (0..h - 1).rev() {
            if v[i][j] == '.' && v[i + 1][j] == '.' {
                tabw[i][j] = tabw[i + 1][j];
            }
        }
    }
    let mut ans = 0;
    for i in 0..h {
        for j in 0..w {
            if v[i][j] == '#' {
                continue;
            }
            ans = max(ans, tabh[i][j] + tabw[i][j] - 1);
        }
    }
    println!("{}", ans);
}
