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
    let d = 1000000007;
    let mut tab: Vec<Vec<usize>> = vec![vec![0; 3] ;61];
    tab[60][0] = 1;
    for i in (0..60).rev() {
        let b = (n >> i) & 1;
        if b == 1 {
            tab[i][0] = tab[i + 1][0];
            tab[i][1] = (tab[i + 1][0] + tab[i + 1][1]) % d;
            tab[i][2] = (2 * tab[i + 1][1] + 3 * tab[i + 1][2]) % d;
        } else {
            tab[i][0] = (tab[i + 1][0] + tab[i + 1][1]) % d;
            tab[i][1] = tab[i + 1][1];
            tab[i][2] = (tab[i + 1][1] + 3 * tab[i + 1][2]) % d;
        }
    }
    println!("{}", tab[0].iter().sum::<usize>() % d);
}
