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
        v: [i64; n]
    }
    let mut w = v;
    w.sort();
    let ans1 = solve(&w);
    w.reverse();
    let ans2 = solve(&w);
    println!("{}", std::cmp::max(ans1,ans2));
}

fn solve(v: &Vec<i64>) -> i64 {
    let n = v.len();
    let mut ret = 0;
    let mut i = 1;
    let mut j = n-1;
    let mut l = v[0];
    let mut r = v[0];
    while i <= j {
        ret += (v[j] - r).abs();
        r = v[j];
        j -= 1;
        if i > j {
            break;
        }

        ret += (v[j] - l).abs();
        l = v[j];
        j -= 1;
        if i > j {
            break;
        }

        ret += (v[i] - r).abs();
        r = v[i];
        i += 1;
        if i > j {
            break;
        }

        ret += (v[i] - l).abs();
        l = v[i];
        i += 1;
    }
    ret
}
