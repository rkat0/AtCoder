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

use std::collections::HashMap;

fn main() {
    input!{
        n: usize,
        m: usize,
        v: [[usize1; 2]; n - 1]
    }
    let mut tree: Vec<Vec<usize>> = vec![Vec::new(); n];
    for i in 0..n - 1 {
        tree[v[i][0]].push(v[i][1]);
        tree[v[i][1]].push(v[i][0]);
    }
    let mut tab: Vec<HashMap<usize, usize>> = vec![HashMap::new(); n];
    let mut map: HashMap<usize, usize> = HashMap::new();
    let mut q: Vec<(usize, usize)> = Vec::new();
    let mut ans: Vec<usize> = vec![0; n];
    ans[0] = dfs(&tree, 0, 0, &mut tab, m, &mut q);
    let mut acl: Vec<Vec<usize>> = vec![vec![1]; n];
    let mut acr: Vec<Vec<usize>> = vec![Vec::new(); n];
    for i in 0..n {
        for j in 0..tree[i].len() {
            if let Some(&x) = tab[i].get(&tree[i][j]) {
                map.insert(tree[i][j], acr[i].len());
                acl[i].push(x);
                acr[i].push(x);
            }
        }
        acr[i].push(1);
        let l = acl[i].len();
        for j in 1..l {
            acl[i][j] = acl[i][j - 1] * (acl[i][j] + 1) % m;
            acr[i][l - j - 1] = acr[i][l - j] * (acr[i][l - j - 1] + 1) % m;
        }
    }
    let mut par: Vec<usize> = vec![0; n];
    for i in 1..n {
        let r = q[i].0;
        let p = q[i].1;
        ans[r] = *tab[p].get(&r).unwrap();
        let idx = *map.get(&r).unwrap();
        par[r] = acl[p][idx] % m * acr[p][idx + 1] % m * (par[p] + 1) % m;
        ans[r] = ans[r] * (par[r] + 1) % m;
    }
    for i in 0..n {
        println!("{}", ans[i]);
    }
}

fn dfs(t: &Vec<Vec<usize>>, r: usize, p: usize, mem: &mut Vec<HashMap<usize, usize>>, m: usize, q: &mut Vec<(usize, usize)>) -> usize {
    q.push((r, p));
    let mut ret = 1;
    for i in 0..t[r].len() {
        let c = t[r][i];
        if c == p {
            continue;
        }
        let tmp = dfs(t, c, r, mem, m, q);
        mem[r].insert(c, tmp);
        ret = ret * (tmp + 1) % m;
    }
    ret
}
