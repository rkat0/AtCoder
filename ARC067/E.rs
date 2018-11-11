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
        a: usize,
        b: usize,
        c: usize,
        d: usize
    }
    let m = 1000000007;
    let mut stairs = vec![1; n + 1];
    for i in 1..n + 1 {
        stairs[i] = stairs[i-1] * i % m;
    }
    let stairs_i: Vec<usize> = stairs.iter().map(|&x| inv_m(x, m)).collect();
    let mut v: Vec<Vec<usize>> = vec![vec![0; n + 1]; b + 1];
    for i in 0..a {
        v[i][0] = 1;
    }
    for i in a..b + 1 {
        v[i][0] = 1;
        for j in 1..n + 1 {
            v[i][j] = v[i - 1][j];
            for k in c..std::cmp::min(d, j / i) + 1 {
                v[i][j] += v[i - 1][j - k * i] * stairs[n + i * k - j] % m * stairs_i[n - j] % m * stairs_i[k] % m * pow_m(stairs_i[i], k, m) % m;
            }
            v[i][j] %= m;
        }
    }
    println!("{}", v[b][n]);
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

fn inv_m(n: usize, d: usize) -> usize {
    pow_m(n, d - 2, d)
}
