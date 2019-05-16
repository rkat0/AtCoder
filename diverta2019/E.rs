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

use std::collections::HashMap;

fn main() {
    input!{
        n: usize,
        v: [usize; n]
    }
    let mut ac: Vec<usize> = vec![0; n + 1];
    for i in 0..n {
        ac[i + 1] = ac[i] ^ v[i];
    }
    let mut ans;
    let d = 1000000007;
    if ac[n] != 0 {
        let mut ls: Vec<usize> = Vec::new();
        let mut prev = 0;
        let mut c = 1;
        for i in 0..n {
            if ac[i + 1] == ac[n] || ac[i + 1] == 0 {
                if ac[i + 1] == prev {
                    c += 1;
                } else {
                    ls.push(c);
                    c = 1;
                    prev = ac[i + 1];
                }
            }
        }
        if c > 0 {
            ls.push(c);
        }
        ans = solve(&ls, d);
    } else {
        let mut zs: Vec<usize> = vec![0; n + 1];
        zs[0] = 1;
        for i in 0..n {
            zs[i + 1] = zs[i];
            if ac[i + 1] == 0 {
                zs[i + 1] += 1;
            }
        }
        ans = pow_m(2, zs[n] - 2, d);
        let mut map: HashMap<usize, Vec<usize>> = HashMap::new();
        for (i, x) in ac.iter().enumerate().filter(|x| *x.1 != 0) {
            map.entry(*x).or_insert(vec![0]).push(i);
        }
        for locs in map.values() {
            let mut ls: Vec<usize> = vec![zs[locs[1]]];
            let mut c = 1;
            for i in 1..locs.len() - 1 {
                let z = zs[locs[i + 1]] - zs[locs[i]];
                if z > 0 {
                    ls.push(c);
                    ls.push(z);
                    c = 1;
                } else {
                    c += 1;
                }
            }
            ls.push(c);
            let z = zs[n] - zs[locs[locs.len() - 1]];
            if z > 0 {
                ls.push(z);
            }
            ans = (ans + solve(&ls, d)) % d;
        }
    }
    println!("{}", ans);
}

fn solve(ls: &Vec<usize>, d: usize) -> usize {
    let n = ls.len();
    if n == 1 {
        return 1;
    }
    let mut dp: Vec<Vec<usize>> = vec![vec![0; 2]; n - 1];
    dp[0][0] = 1;
    for i in 1..n - 1 {
        if i % 2 == 0 {
            dp[i][0] = (dp[i - 1][0] + dp[i - 1][1] * ls[i]) % d;
            dp[i][1] = dp[i - 1][1];
        } else {
            dp[i][0] = dp[i - 1][0];
            dp[i][1] = (dp[i - 1][0] * ls[i] + dp[i - 1][1]) % d;
        }
    }
    if n % 2 == 1 {
        dp[n - 2][1]
    } else {
        dp[n - 2][0]
    }
}

fn pow_m(n: usize, mut p: usize, d: usize) -> usize {
    let mut ret = 1;
    let mut r = n;
    while p > 0 {
        if p % 2 == 0 {
            r = r * r % d;
            p /= 2;
        } else {
            ret = ret * r % d;
            p -= 1;
        }
    }
    ret
}
