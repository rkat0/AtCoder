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
    let mut ans = 0;
    for i in 3..10 {
        for j in 4..3usize.pow(i as u32) {
            let mut m = 0;
            let mut k = j;
            for l in 0..i {
                m *= 10;
                if k % 3 == 0 {
                    m += 3;
                } else if k % 3 == 1 {
                    m += 5;
                } else if k % 3 == 2 {
                    m += 7;
                }
                k /= 3;
            }
            if m <= n && is_753(m) {
                ans += 1;
            }
        }
    }
    println!("{}", ans);
}

fn is_753(mut n: usize) -> bool {
    let mut f3 = false;
    let mut f5 = false;
    let mut f7 = false;
    while n > 0 {
        let t = n % 10;
        if t == 3 {
            f3 = true;
        } else if t == 5 {
            f5 = true;
        } else if t == 7 {
            f7 = true;
        } else {
            return false;
        }
        n /= 10;
    }
    f3 && f5 && f7
}
