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

fn main() {
    input!{
        n: usize,
        m: usize,
        v: [[usize1; 2]; n + m - 1]
    }
    let mut t: Vec<Vec<usize>> = vec![Vec::new(); n];
    let mut rev: Vec<Vec<usize>> = vec![Vec::new(); n];
    for i in 0..n + m - 1 {
        t[v[i][0]].push(v[i][1]);
        rev[v[i][1]].push(v[i][0]);
    }
    let mut tab: Vec<[usize; 2]> = vec![[0, 0]; n];
    for i in 0..n {
        if t[i].is_empty() {
            let _ = dfs(&rev, i, &mut tab);
        }
    }
    for i in 0..n {
        println!("{}", tab[i][0]);
    }
}

fn dfs(t: &Vec<Vec<usize>>, v: usize, tab: &mut Vec<[usize; 2]>) -> usize {
    let mut len = 0;
    for &p in t[v].iter() {
        let lt = if tab[p][1] != 0 {
            tab[p][1] + 1
        } else {
            dfs(t, p, tab) + 1
        };
        if len < lt {
            len = lt;
            tab[v] = [p + 1, len];
        }
    }
    len
}
