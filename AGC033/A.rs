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

use std::collections::VecDeque;

fn main() {
    input!{
        h: usize,
        w: usize,
        v: [chars; h]
    }
    let mut tab: Vec<Vec<usize>> = vec![vec![h + w; w]; h];
    let mut q: VecDeque<(usize, usize)> = VecDeque::new();
    for i in 0..h {
        for j in 0..w {
            if v[i][j] == '#' {
                tab[i][j] = 0;
                q.push_back((i, j));
            }
        }
    }
    while !q.is_empty() {
        let (x, y) = q.pop_front().unwrap();
        if x < h - 1 && tab[x + 1][y] == h + w {
            tab[x + 1][y] = tab[x][y] + 1;
            q.push_back((x + 1, y));
        }
        if x > 0 && tab[x - 1][y] == h + w {
            tab[x - 1][y] = tab[x][y] + 1;
            q.push_back((x - 1, y));
        }
        if y < w - 1 && tab[x][y + 1] == h + w {
            tab[x][y + 1] = tab[x][y] + 1;
            q.push_back((x, y + 1));
        }
        if y > 0 && tab[x][y - 1] == h + w {
            tab[x][y - 1] = tab[x][y] + 1;
            q.push_back((x, y - 1));
        }
    }
    println!("{}", tab.iter().map(|l| l.iter().max().unwrap()).max().unwrap());
}
