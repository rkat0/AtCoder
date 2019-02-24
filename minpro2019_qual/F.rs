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
        s: chars
    }
    let d = 998244353;
    let n = s.len();
    let mut dp: Vec<Vec<usize>> = vec![vec![0; n + 1]; n + 1];
    dp[0][0] = 1;
    for (i, &b) in s.iter().enumerate() {
        if b == '0' {
            dp[i + 1][0] = dp[i][0];
            for j in 1..i + 1 {
                dp[i + 1][j - 1] = (dp[i + 1][j - 1] + dp[i][j]) % d;
                dp[i + 1][j] = dp[i][j];
            }
        } else if b == '1' {
            for j in 0..i + 1 {
                dp[i + 1][j + 1] = dp[i][j];
                dp[i + 1][j] = (dp[i + 1][j] + dp[i][j]) % d;
            }
        } else {
            for j in 0..i {
                dp[i + 1][j + 1] = (dp[i + 1][j + 1] + dp[i][j]) % d;
                dp[i + 1][j + 2] = dp[i][j];
            }
            dp[i + 1][i + 1] = (dp[i + 1][i + 1] + dp[i][i]) % d;
        }
    }
    let mut ans = 0;
    for i in 0..n + 1 {
        ans += dp[n][i] * comb_m(n, i, d) % d;
    }
    println!("{}", ans % d);
}

fn prod_m(n: usize, m: usize, d: usize) -> usize {
    (n..m+1).fold(1, |ac, x| ac * x % d)
}

fn stair_m(n: usize, d: usize) -> usize {
    (1..n+1).fold(1, |ac, x| ac * x % d)
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

fn comb_m(n: usize, r: usize, d: usize) -> usize {
    prod_m(n - r + 1, n, d) * inv_m(stair_m(r, d), d) % d
}
