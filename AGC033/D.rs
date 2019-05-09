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

use std::cmp::{max, min};

fn main() {
    input!{
        h: usize,
        w: usize,
        v: [chars; h]
    }
    let mut maxc = 0;
    while 1 << maxc <= h * w {
        maxc += 1;
    }
    maxc += 1;
    let mut tabh: Vec<Vec<Vec<usize>>> = Vec::with_capacity(h);
    for x in 0..h {
        tabh.push(vec![vec![0; h - x]; w]);
    }
    let mut tabw: Vec<Vec<Vec<usize>>> = Vec::with_capacity(h);
    for x in 0..h {
        let mut t: Vec<Vec<usize>> = Vec::with_capacity(w);
        for y in 0..w {
            t.push(vec![0; w - y]);
        }
        tabw.push(t);
    }
    for x in 0..h {
        tabh[x][w - 1][0] = 1;
        for y in (0..w - 1).rev() {
            if v[x][y] == v[x][y + 1] {
                tabh[x][y][0] = tabh[x][y + 1][0] + 1;
            } else {
                tabh[x][y][0] = 1;
            }
        }
    }
    for x in 0..h {
        for y in 0..w {
            for z in x + 1..h {
                if v[x][y] == v[z][y] {
                    tabh[x][y][z - x] = min(tabh[x][y][z - x - 1], tabh[z][y][0]);
                } else {
                    tabh[x][y][z - x] = 0;
                }
            }
        }
    }
    for y in 0..w {
        tabw[h - 1][y][0] = 1;
        for x in (0..h - 1).rev() {
            if v[x][y] == v[x + 1][y] {
                tabw[x][y][0] = tabw[x + 1][y][0] + 1;
            } else {
                tabw[x][y][0] = 1;
            }
        }
    }
    for y in 0..w {
        for x in 0..h {
            for z in y + 1..w {
                if v[x][y] == v[x][z] {
                    tabw[x][y][z - y] = min(tabw[x][y][z - y - 1], tabw[x][z][0]);
                } else {
                    tabw[x][y][z - y] = 0;
                }
            }
        }
    }
    for c in 1..maxc {
        if tabh[0][0][h - 1] == w {
            println!("{}", c - 1);
            return;
        }
        let mut next_tabh = tabh.clone();
        let mut next_tabw = tabw.clone();
        for y in 0..w {
            'x_loop: for x in 0..h {
                let mut len = w - y - 1;
                for z in 0..h - x {
                    let a = tabh[x][y][z];
                    if a == 0 {
                        for i in 0..len + 1 {
                            next_tabw[x][y][i] = max(next_tabw[x][y][i], z);
                        }
                        continue 'x_loop;
                    }
                    let b = if y + a < w {tabh[x][y + a][z]} else {0};
                    next_tabh[x][y][z] = max(next_tabh[x][y][z], a + b);
                    while a + b - 1 < len {
                        next_tabw[x][y][len] = max(next_tabw[x][y][len], z);
                        len -= 1;
                    }
                }
                for i in 0..len + 1 {
                    next_tabw[x][y][i] = max(next_tabw[x][y][i], h - x);
                }
            }
        }
        for x in 0..h {
            'y_loop: for y in 0..w {
                let mut len = h - x - 1;
                for z in 0..w - y {
                    let a = tabw[x][y][z];
                    if a == 0 {
                        for i in 0..len + 1 {
                            next_tabh[x][y][i] = max(next_tabh[x][y][i], z);
                        }
                        continue 'y_loop;
                    }
                    let b = if x + a < h {tabw[x + a][y][z]} else {0};
                    next_tabw[x][y][z] = max(next_tabw[x][y][z], a + b);
                    while a + b - 1 < len {
                        next_tabh[x][y][len] = max(next_tabh[x][y][len], z);
                        len -= 1;
                    }
                }
                for i in 0..len + 1 {
                    next_tabh[x][y][i] = max(next_tabh[x][y][i], w - y);
                }
            }
        }
        tabh = next_tabh;
        tabw = next_tabw;
    }
    println!("{}", maxc - 1);
}
