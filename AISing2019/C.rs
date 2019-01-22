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
    let mut g: Vec<Vec<usize>> = vec![Vec::new(); h * w];
    for i in 0..h {
        for j in 0..w - 1 {
            if v[i][j] != v[i][j + 1] {
                g[w * i + j].push(w * i + j + 1);
                g[w * i + j + 1].push(w * i + j);
            }
        }
    }
    for i in 0..h - 1 {
        for j in 0..w {
            if v[i][j] != v[i + 1][j] {
                g[w * i + j].push(w * i + j + w);
                g[w * i + j + w].push(w * i + j);
            }
        }
    }
    let mut done = vec![false; h * w];
    let mut ans = 0;
    for i in 0..h {
        for j in 0..w {
            let n = w * i + j;
            if done[n] {
                continue;
            }
            done[n] = true;
            let (bc, wc) = dfs(&g, n, 0, &mut done);
            ans += bc * wc;
        }
    }
    println!("{}", ans);
}

fn dfs(g: &Vec<Vec<usize>>, r: usize, t: usize, mem: &mut Vec<bool>) -> (usize, usize) {
    let mut bc = 0;
    let mut wc = 0;
    if t == 0 {
        bc += 1;
    } else {
        wc += 1;
    }
    for &c in g[r].iter().clone() {
        if mem[c] {
            continue;
        }
        mem[c] = true;
        let (bt, wt) = if t == 0 {
            dfs(g, c, 1, mem)
        } else {
            dfs(g, c, 0, mem)
        };
        bc += bt;
        wc += wt;
    }
    (bc, wc)
}
