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
        v: [usize; n]
    }
    let d = 1000000007;
    let hs: Vec<usize> = (1..n+1).map(|x| inv_m(x, d)).scan(0, |ac, x| {*ac = (*ac + x) % d; Some(*ac)}).collect();
    let mut ans = 0;
    for i in 0..n {
        ans = (ans + v[i] * (hs[i] + hs[n-i-1] - 1)) % d;
    }
    ans = ans * stair_m(n, d) % d;
    println!("{}",ans);
}

fn stair_m(n: usize, d: usize) -> usize {
    (1..n+1).fold(1, |ac, x| ac * x % d)
}

fn inv_m(n: usize, d: usize) -> usize {
    pow_m(n, d - 2, d)
}

fn pow_m(n: usize, mut p: usize, d: usize) -> usize {
    let mut ret = 1;
    let mut r = n;
    while p > 0 {
        if p % 2 == 0 {
            r = r * r % d;
            p /= 2;
        } else {
            ret  = ret * r % d;
            p -= 1;
        }
    }
    ret
}
