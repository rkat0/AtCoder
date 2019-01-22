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
        a: [usize; n],
        b: [usize; n]
    }
    let sa: usize = a.iter().clone().sum();
    let sb: usize = b.iter().clone().sum();
    if sa < sb {
        println!("-1");
        return;
    }
    let mut ans = 0;
    let mut m = 0;
    let mut d: Vec<usize> = Vec::new();
    for i in 0..n {
        if a[i] < b[i] {
            ans += 1;
            m += b[i] - a[i];
        } else {
            d.push(a[i] - b[i]);
        }
    }
    if ans == 0 {
        println!("0");
        return;
    }
    d.sort_by(|x, y| y.cmp(x));
    let mut st = 0;
    for i in 0..d.len() {
        ans += 1;
        st += d[i];
        if st >= m {
            break;
        }
    }
    println!("{}", ans);
}
