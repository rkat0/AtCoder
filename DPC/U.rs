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
        a: [[i64; n]; n]
    }
    let mut tab: Vec<i64> = vec![0; 1 << n];
    for j in 1..n {
        let b = 1 << j;
        for i in 0..1 << j {
            tab[i | b] = tab[i];
            for k in 0..j {
                if 1 << k & i > 0 {
                    tab[i | b] += a[k][j];
                }
            }
        }
    }
    for i in 0..1 << n {
        let mut j: i64 = 0;
        loop {
            j = (j - i as i64) & i as i64;
            tab[i] = std::cmp::max(tab[i], tab[j as usize] + tab[i ^ j as usize]);
            if j as usize == i {
                break;
            }
        }
    }
    println!("{}", tab[(1 << n) - 1]);
}
