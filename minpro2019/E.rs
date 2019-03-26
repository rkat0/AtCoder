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

fn main() {
    input!{
        n: usize,
        m: usize
    }
    let d = 1000000007;
    let mut st: Vec<usize> = vec![1; n + 1];
    for i in 1..n + 1 {
        st[i] = st[i - 1] * i % d;
    }
    let sti: Vec<usize> = st.iter().map(|&x| inv_m(x, d)).collect();
    let mut a: Vec<Vec<usize>> = vec![vec![0; n / 2 + 1]; n / 2 + 1];
    for i in 0..n / 2 + 1 {
        for j in 0..n / 2 + 1 {
            if i + j == n {
                a[i][j] = 1;
                continue;
            }
            for k in 0..std::cmp::min(i, j + 1) {
                a[i][j] = (a[i][j] + sti[i - k] * comb_m(n - i - j - 1, i - k - 1, &st, &sti, d) % d * comb_m(j, k, &st, &sti, d) % d * st[n - j - k] % d) % d
            }
        }
    }
    a = matpow_m(&a, m - 1, d);
    let mut x1 = vec![0; n / 2 + 1];
    x1[0] = 1;
    let xm = matvec_m(&a, &x1, d);
    let mut ans = 0;
    for i in 0..n / 2 + 1 {
        ans = (ans + xm[i] * st[n - i - 1]) % d;
    }
    println!("{}", ans);
}

fn pow_m(n: usize, mut p: usize, d: usize) -> usize {
    let mut r = n;
    let mut ret = 1;
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

fn comb_m(n: usize, r: usize, st: &[usize], sti: &[usize], d: usize) -> usize {
    if n < r {
        0
    } else {
        st[n] * sti[n - r] % d * sti[r] % d
    }
}

fn inv_m(n: usize, d: usize) -> usize {
    pow_m(n, d - 2, d)
}

fn matvec_m(a: &[Vec<usize>], b: &[usize], d: usize) -> Vec<usize> {
    let m = a.len();
    let n = b.len();
    let mut ret: Vec<usize> = vec![0; m];
    for i in 0..m {
        for j in 0..n {
            ret[i] = (ret[i] + a[i][j] * b[j]) % d;
        }
    }
    ret
}

fn matmul_m(a: &[Vec<usize>], b: &[Vec<usize>], d: usize) -> Vec<Vec<usize>> {
    let l = a.len();
    let m = b.len();
    let n = b[0].len();
    let mut ret: Vec<Vec<usize>> = vec![vec![0; n]; l];
    for i in 0..l {
        for j in 0..m {
            for k in 0..n {
                ret[i][k] = (ret[i][k] + a[i][j] * b[j][k]) % d;
            }
        }
    }
    ret
}

fn matpow_m(a: &[Vec<usize>], mut p: usize, d: usize) -> Vec<Vec<usize>> {
    let n = a.len();
    let mut r: Vec<Vec<usize>> = a.to_vec();
    let mut ret = vec![vec![0; n]; n];
    for i in 0..n {
        ret[i][i] = 1;
    }
    while p > 0 {
        if p % 2 == 0 {
            r = matmul_m(&r, &r, d);
            p /= 2;
        } else {
            ret = matmul_m(&ret, &r, d);
            p -= 1;
        }
    }
    ret
}
