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
        n: usize
    }
    let d = 1000000007;
    let mut dp: Vec<Vec<usize>> = vec![vec![0; 64]; n + 1];
    for i in 0..64 {
        if !check4((i << 2) | 3) {
            dp[0][i] = 1;
        }
    }
    for i in 0..n {
        for j in 0..64 {
            for k in 0..4 {
                let t = (j << 2) | k;
                if !check4(t) {
                    dp[i + 1][t & 63] = (dp[i + 1][t & 63] + dp[i][j]) % d;
                }
            }
        }
    }
    println!("{}", dp[n][63]);
}

fn check4(s: usize) -> bool {
    let a = s >> 6;
    let b = (s >> 4) & 3;
    let c = (s >> 2) & 3;
    let d = s & 3;
    check3(a, b, c) || check3(b, c, d)
    || check3(b, a, c) || check3(a, c, d)
    || check3(a, c, b) || check3(c, b, d)
    || check3(a, b, d) || check3(b, d, c)
}

fn check3(a: usize, b: usize, c: usize) -> bool {
    a == 0 && b == 1 && c == 2
}
