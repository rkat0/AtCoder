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
    let ps = vec![2,3,5,7,11,13,17,19,23,29,31,37,41,43,47];
    let mut pps = vec![0; 15];
    for i in 0..15 {
        let mut m = n;
        while m > 0 {
            m /= ps[i];
            pps[i] += m;
        }
    }
    for i in 0..15 {
        if pps[i] >= 74 {
            ans += 1;
        }
    }
    for i in 0..15 {
        for j in 0..15 {
            if i == j {
                continue;
            }
            if pps[i] >= 2 && pps[j] >= 24 {
                ans += 1;
            }
            if pps[i] >= 4 && pps[j] >= 14 {
                ans += 1;
            }
        }
    }
    for i in 0..15 {
        for j in 0..15 {
            if i == j {
                continue;
            }
            for k in j + 1..15 {
                if k == i {
                    continue;
                }
                if pps[i] >= 2 && pps[j] >= 4 && pps[k] >= 4 {
                    ans += 1;
                }
            }
        }
    }
    println!("{}", ans);
}
