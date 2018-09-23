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
        m: usize
    }
    let mut x = m;
    let mut ans = 1;
    let d = 1000000007;
    let mut c = 0;
    while x % 2 == 0 {
        c += 1;
        x /= 2;
    }
    ans = ans * h_m(n,c,d) % d;

    let mut ps: Vec<usize> = Vec::new();
    let mut p = 3;
    let mut is_prime: bool;
    while p * p <= x {
        is_prime = true;
        for x in ps.clone() {
            if p % x == 0 {
                is_prime = false;
                break;
            }
            if p < x * x {
                break;
            }
        }
        if is_prime {
            ps.push(p);
        }
        p += 2;
    }

    for q in ps {
        c = 0;
        while x % q == 0 {
            c += 1;
            x /= q;
        }
        ans = ans * h_m(n,c,d) % d;
    }
    if x != 1 {
        ans = ans * n % d;
    }
    println!("{}",ans);
}


fn h_m(n: usize, m: usize, d: usize) -> usize {
    comb_m(n+m-1, m, d)
}

fn stair_m(n: usize, m: usize, d: usize) -> usize {
    (n..m+1).fold(1, |ac,x| ac * x % d)
}

fn comb_m(n: usize, r: usize, d: usize) -> usize {
    stair_m(n+1-r, n, d) * inv_m(stair_m(1, r, d),d) % d
}

fn inv_m(n: usize, d: usize) -> usize {
    pow_m(n,d-2,d)
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
