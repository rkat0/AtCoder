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
        n: usize,
        ae: [(usize1, usize1); n - 1],
        m: usize,
        be: [(usize1, usize1); m - 1],
    }
    let mut ta: Vec<Vec<usize>> = vec![Vec::new(); n];
    for (x, y) in ae {
        ta[x].push(y);
        ta[y].push(x);
    }
    let mut tb: Vec<Vec<usize>> = vec![Vec::new(); m];
    for (x, y) in be {
        tb[x].push(y);
        tb[y].push(x);
    }
    let mut da: Vec<usize> = vec![0; n];
    tree_dists(&ta, &mut da);
    let mut db: Vec<usize> = vec![0; m];
    tree_dists(&tb, &mut db);
    da.sort();
    db.sort();
    let l = max(da[n - 1], db[m - 1]);
    let mut ans = 0;
    let mut acc = 0;
    let mut j = m;
    for i in 0..n {
        while j > 0 && da[i] + db[j - 1] + 1 >= l {
            j -= 1;
            acc += db[j] + 1;
        }
        ans += da[i] * (m - j) + acc + j * l;
    }
    println!("{}", ans);
}

fn tree_dists(t: &Vec<Vec<usize>>, d: &mut Vec<usize>) {
    let n = t.len();
    let mut mem: Vec<[usize; 2]> = vec![[0, 0]; n];
    dfs1(0, 0, t, d, &mut mem);
    dfs2(0, 0, t, d, &mut mem);
}

fn dfs1(r: usize, p: usize, t: &Vec<Vec<usize>>, d: &mut Vec<usize>, mem: &mut Vec<[usize; 2]>) {
    if r != p && t[r].len() == 1 {
        mem[r][0] = r;
        return;
    }
    let mut v1 = 0;
    let mut m1 = 0;
    let mut m2 = 0;
    for &v in t[r].iter() {
        if v == p {
            continue;
        }
        dfs1(v, r, t, d, mem);
        if m1 < d[v] {
            m2 = m1;
            m1 = d[v];
            v1 = v;
        } else if m2 < d[v] {
            m2 = d[v];
        }
    }
    m1 += 1;
    m2 += 1;
    mem[r] = [v1, m2];
    d[r] = m1;
}

fn dfs2(r: usize, p: usize, t: &Vec<Vec<usize>>, d: &mut Vec<usize>, mem: &mut Vec<[usize; 2]>) {
    if t[r].len() == 1 {
        if r != p {
            return;
        }
        let v = t[r][0];
        if d[r] == 1 {
            d[v] = 1;
            return;
        }
        d[v] = d[r] - 1;
        mem[v][1] = max(mem[v][1], 1);
        dfs2(v, r, t, d, mem);
        return;
    }
    for &v in t[r].iter() {
        if v == p {
            continue;
        }
        let dr = if v != mem[r][0] {
            d[r] + 1
        } else {
            mem[r][1] + 1
        };
        if d[v] < dr {
            mem[v][0] = r;
            mem[v][1] = d[v];
            d[v] = dr;
        } else if mem[v][1] < dr {
            mem[v][1] = dr;
        }
        dfs2(v, r, t, d, mem);
    }
}
