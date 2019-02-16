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
        n: usize,
        k: usize,
        v: [[usize; n]; n]
    }
    let d = 1000000007;
    let ans = matpow_m(&v, k, d).iter().map(|x| x.iter().fold(0, |ac, y| (ac + y) % d)).fold(0, |ac, x| (ac + x) % d);
    println!("{}", ans);
}

fn matpow_m(m: &Vec<Vec<usize>>, mut p: usize, d: usize) -> Vec<Vec<usize>> {
    let n = m.len();
    let mut ret = vec![vec![0; n]; n];
    let mut r = m.to_vec();
    for i in 0..n {
        ret[i][i] = 1;
    }
    while p > 0 {
        if p % 2 == 0 {
            r = matmat_m(&r, &r, d);
            p /= 2;
        } else {
            ret = matmat_m(&ret, &r, d);
            p -= 1;
        }
    }
    ret
}

fn matmat_m(a: &Vec<Vec<usize>>, b: &Vec<Vec<usize>>, d: usize) -> Vec<Vec<usize>> {
    let l = a.len();
    let m = b.len();
    let n = b[0].len();
    let mut c = vec![vec![0; l]; n];
    for i in 0..l {
        for j in 0..m {
            for k in 0..n {
                c[i][k] = (c[i][k] + a[i][j] * b[j][k]) % d;
            }
        }
    }
    c
}
