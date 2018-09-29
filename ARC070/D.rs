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
        k: usize,
        v: [usize; n]
    }
    let mut w = v;
    w.sort();
    if w[0] >= k {
        println!("0");
        return;
    }
    {
        let mut ps: Vec<bool> = vec![false;k];
        ps[0] = true;
        for i in 1..n {
            let mut psn = ps.clone();
            for j in 0..k {
                if j + w[i] >= k {
                    break;
                }
                if ps[j] == true {
                    psn[j + w[i]] = true;
                }
            }
            ps = psn;
        }
        if ps[k-w[0]..].iter().any(|&x| x) {
            println!("0");
            return;
        }
    }
    let mut l = 0;
    let mut r = n;
    while l + 1 < r {
        let m = (l + r) / 2;
        if w[m] >= k {
            r = m;
            continue;
        }
        let mut ps: Vec<bool> = vec![false;k];
        ps[0] = true;
        for i in 0..n {
            if i == m {
                continue;
            }
            let mut psn = ps.clone();
            for j in 0..k {
                if j + w[i] >= k {
                    break;
                }
                if ps[j] == true {
                    psn[j + w[i]] = true;
                }
            }
            ps = psn;
        }
        if ps[k-w[m]..].iter().any(|&x| x) {
            r = m;
        } else {
            l = m;
        }
    }
    println!("{}",r);
}
