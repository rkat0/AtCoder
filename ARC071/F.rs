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
    let m = 1000000007;
    let mut ac = vec![1;n+1];
    if n == 1 {
        println!("1");
        return;
    }
    ac[1] = 1;
    let mut b = 0;
    let mut sumb = 0;
    for i in 2..n+1 {
        b = (sumb + ac[i-2]) % m;
        sumb = (sumb + b) % m;
        ac[i] = (pow_m(2,i-1,m) + m - b) % m;
    }
    let mut ans = 0;
    for i in 0..n {
        if i < n - 2 {
            ans += ac[i] * (n-i-2) % m * (n-1) % m;
        }
        if i < n - 1 {
            ans += ac[i] * (i + 1) % m * n % m;
        } else {
            ans += ac[i] * (n - 1) % m;
        }
        ans %= m;
    }
    ans = (ans + ac[n]) % m;
    println!("{}",ans);
}

fn pow_m(n: usize, mut p: usize, m: usize) -> usize {
    let mut r = n;
    let mut ret = 1;
    while p > 0 {
        if p % 2 == 0 {
            r = r * r % m;
            p /= 2;
        } else {
            ret = ret * r % m;
            p -= 1;
        }
    }
    ret
}
