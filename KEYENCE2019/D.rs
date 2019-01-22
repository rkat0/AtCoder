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
        m: usize,
        a: [usize; n],
        b: [usize; m]
    }
    let d = 1000000007;
    let mut sa = a;
    sa.sort_by(|x, y| y.cmp(&x));
    let mut sb = b;
    sb.sort_by(|x, y| y.cmp(&x));
    if sa[0] != n * m || sb[0] != n * m {
        println!("0");
        return;
    }
    for i in 0..n - 1 {
        if sa[i] == sa[i + 1] {
            println!("0");
            return;
        }
    }
    for i in 0..m - 1 {
        if sb[i] == sb[i + 1] {
            println!("0");
            return;
        }
    }
    let mut ans: usize = 1;
    let mut ida = 0;
    let mut idb = 0;
    for i in (1..n * m).rev() {
        let mut tr = ida + 1;
        let mut tc = idb + 1;
        if (ida < n - 1 && i == sa[ida + 1]) || (idb < m - 1 && i == sb[idb + 1]) {
            if ida < n - 1 && i == sa[ida + 1] {
                tr = 1;
                ida += 1;
            }
            if idb < m - 1 && i == sb[idb + 1] {
                tc = 1;
                idb += 1;
            }
            ans = ans * (tr * tc) % d;
        } else {
            ans = ans * (tr * tc - (n * m - i)) % d;
        }
    }
    println!("{}", ans);
}
