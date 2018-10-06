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
        v: [[i64; 3]; n]
    }
    for i in 0..101 {
        'next: for j in 0..101 {
            let mut h: i64 = 0;
            for k in 0..n {
                if v[k][2] == 0 {
                    continue;
                }
                h = v[k][2] + (i - v[k][0]).abs() + (j - v[k][1]).abs();
                break;
            }
            for k in 0..n {
                let dh = v[k][2] - std::cmp::max(h - (i - v[k][0]).abs() - (j - v[k][1]).abs(), 0);
                if dh != 0 {
                    continue 'next;
                }
            }
            println!("{} {} {}", i, j, h);
            return;
        }
    }
}
