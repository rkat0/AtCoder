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
        n: usize
    }
    let d = 1000000007;
    let mut ans = 0;
    let pows: Vec<usize> = (0..n + 1).map(|x| pow_m(x, 10, d)).collect();
    let mut ps = vec![0; n + 1];
    let mut ps_sum = 0;
    for i in 1..n + 1 {
        ps[i] = (pows[i] + d - ps_sum) % d;
        ps_sum = (ps_sum + ps[i]) % d;
    }
    for i in 1..n+1 {
        if i * i > n {
            break;
        }
        ans = (ans + ps[i] * ps[i]) % d;
        for j in i+1..n+1 {
            if i * j > n {
                break;
            }
            ans = (ans + 2 * ps[i] * ps[j]) % d;
        }
    }
    println!("{}", ans);
}

fn pow_m(n: usize, mut p: usize, d: usize) -> usize {
    if n == 0 && p > 0 {
        return 0;
    }
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
