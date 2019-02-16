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
        k: chars,
        d: usize
    }
    let m = 1000000007;
    let n = k.len();
    let mut tab: Vec<Vec<usize>> = vec![vec![0; d]; n];
    tab[0][0] = 1;
    for i in 1..n {
        let mut t: usize = 0;
        for j in 0..10 {
            t += tab[i - 1][(11 * d - 1 - j) % d];
        }
        for j in 0..d {
            t = t + tab[i - 1][j] - tab[i - 1][(j + 10 * d - 10) % d];
            tab[i][j] = t % m;
        }
    }
    let mut s = 0;
    let mut ans: usize = 0;
    for i in 0..n {
        let c = k[i].to_digit(10u32).unwrap() as usize;
        let mut t = 0;
        for j in 0..c {
            t += tab[n - 1 - i][(11 * d - s - j) % d];
        }
        ans = (ans + t) % m;
        s = (s + c) % d;
    }
    if s == 0 {
        ans += 1;
    }
    println!("{}", (ans + m - 1) % m);
}
