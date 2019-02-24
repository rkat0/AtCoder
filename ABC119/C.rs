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
        a: i64,
        b: i64,
        c: i64,
        v: [i64; n]
    }
    let ans = solve(a, b, c, n, &v);
    println!("{}", ans);
}

fn solve(a: i64, b: i64, c: i64, n: usize, v: &Vec<i64>) -> i64 {
    let mut ans = i64::max_value();
    for i in 1..1 << 2 * n {
        let mut m = 0;
        let mut x = 0;
        let mut y = 0;
        let mut z = 0;
        for j in 0..n {
            let k = (i >> 2 * j) & 3;
            if k == 1 {
                m += 1;
                x += v[j];
            } else if k == 2 {
                m += 1;
                y += v[j];
            } else if k == 3 {
                m += 1;
                z += v[j];
            }
        }
        if x == 0 || y == 0 || z == 0 {
            continue;
        }
        let mp = (a - x).abs() + (b - y).abs() + (c - z).abs() + 10 * m - 30;
        ans = std::cmp::min(ans, mp);
    }
    ans
}
