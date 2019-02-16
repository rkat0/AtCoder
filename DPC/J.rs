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
        v: [usize; n]
    }
    let mut c = [0, 0, 0];
    for i in 0..n {
        c[v[i] - 1] += 1;
    }
    let mut tab: Vec<Vec<Vec<f64>>> = vec![vec![vec![0.; c[0] + c[1] + c[2] + 1]; c[1] + c[2] + 1]; c[2] + 1];
    for i in 0..c[2] + 1 {
        for j in 0..c[1] + c[2] + 1 - i {
            for k in 0..c[0] + c[1] + c[2] + 1 - i - j {
                let l = i + j + k;
                if l == 0 {
                    continue;
                }
                tab[i][j][k] = n as f64 / l as f64;
                if i != 0 {
                    tab[i][j][k] += tab[i - 1][j + 1][k] * i as f64 / l as f64;
                }
                if j != 0 {
                    tab[i][j][k] += tab[i][j - 1][k + 1] * j as f64 / l as f64;
                }
                if k != 0 {
                    tab[i][j][k] += tab[i][j][k - 1] * k as f64 / l as f64;
                }
            }
        }
    }
    println!("{}", tab[c[2]][c[1]][c[0]]);
}
