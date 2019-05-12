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
        x: usize
    }
    let d = 998244353;
    let mut stairs: Vec<usize> = vec![1; n + 1];
    for i in 1..n + 1 {
        stairs[i] = (stairs[i - 1] * i) % d;
    }
    let stairs_i: Vec<usize> = stairs.iter().map(|&s| inv_m(s, d)).collect();
    let mut ans = 0;
    for i in 0..n + 1 {
        for j in 0..n - i + 1 {
            let t = i + 2 * j;
            if t < x {
                ans += stairs[n] * stairs_i[n - i - j] % d * stairs_i[i] % d * stairs_i[j] % d;
                ans %= d;
            } else {
                let k = (t - x + 1) / 2;
                if (t - x + 1) % 2 == 1 || i > 0 && 2 * k > j {
                    continue;
                } else if 2 * k < x - 1 {
                    let jj = j - 2 * k;
                    ans += stairs[n] * stairs_i[n - i - j] % d * stairs_i[i + j] % d * stairs[i + jj] % d * stairs_i[i] % d * stairs_i[jj] % d;
                    ans %= d;
                } else if x % 2 == 1 {
                    ans += stairs[n] * stairs_i[n - i - j] % d * stairs_i[i + j] % d;
                    ans %= d;
                }
            }
        }
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

fn inv_m(n: usize, d: usize) -> usize {
    pow_m(n, d - 2, d)
}
