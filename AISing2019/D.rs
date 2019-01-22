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
        q: usize,
        a: [usize; n],
        xs: [usize; q]
    }
    let mut ac: Vec<usize> = vec![0; n + 1];
    for i in 0..n {
        ac[i + 1] = ac[i] + a[i];
    }
    let mut eo: Vec<usize> = vec![0; n];
    eo[0] = a[0];
    eo[1] = a[1];
    for i in 2..n {
        eo[i] = eo[i - 2] + a[i];
    }
    for i in 0..n {
        a[i] *= 3;
    }
    for i in 0..q {
        xs[i] = xs[i] * 3 - 1;
    }
    for i in 0..q {
        if xs[i] + 1 >= a[n - 1] {
            println!("{}", eo[n - 1]);
            continue;
        }
        let mut l = 0;
        let mut r = 3000000000;
        let mut li = 0;
        let mut ri = 0;
        while l + 1 < r {
            let m = (l + r) / 2;
            let lt = if xs[i] < a[0] + m {
                0
            } else {
                let mut l2 = -1;
                let mut r2 = n as i64 - 1;
                while l2 + 1 < r2 {
                    let m2 = (l2 + r2 + 1) / 2;
                    if a[m2 as usize] < xs[i] - m {
                        l2 = m2;
                    } else {
                        r2 = m2;
                    }
                }
                r2 as usize
            };
            let rt = {
                let mut l2 = 0;
                let mut r2 = n;
                while l2 + 1 < r2 {
                    let m2 = (l2 + r2) / 2;
                    if a[m2] <= xs[i] + m {
                        l2 = m2;
                    } else {
                        r2 = m2;
                    }
                }
                l2
            };
            if rt + 1 - lt <= n - 1 - rt {
                l = m;
                li = lt;
                ri = rt;
            } else {
                r = m;
            }
        }
        let mut ans = ac[n] - ac[ri + 1];
        if li > 0 && ri + 1 - li == n - 1 - ri {
            ans += eo[li - 1];
        } else if li > 1 && ri + 2 - li == n - 1 - ri {
            ans += eo[li - 2];
        }
        println!("{}", ans);
    }
}
