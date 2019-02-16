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

fn main() {
    input!{
        h: usize,
        w: usize,
        v: [chars; h]
    }
    let d = 1000000007;
    let mut tab: Vec<Vec<usize>> = vec![vec![0; w]; h];
    tab[0][0] = 1;
    for i in 1..w {
        if v[0][i] == '#' {
            break;
        }
        tab[0][i] = tab[0][i - 1];
    }
    for i in 1..h {
        if v[i][0] == '#' {
            break;
        }
        tab[i][0] = tab[i - 1][0];
    }
    for i in 1..h {
        for j in 1..w {
            if v[i][j] == '#' {
                continue;
            }
            tab[i][j] = (tab[i][j - 1] + tab[i - 1][j]) % d;
        }
    }
    println!("{}", tab[h - 1][w - 1]);
}
