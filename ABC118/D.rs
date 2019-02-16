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

    ($next:expr, mut $var:ident : $t:tt $($r:tt)*) => {
        let mut $var = read_value!($next, $t);
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
        mut v: [usize1; m]
    }
    v.sort_by(|x, y| y.cmp(&x));
    let tab = [2, 5, 5, 4, 5, 6, 3, 7, 6];
    let mut dig: Vec<usize> = vec![0; m];
    for i in 0..m {
        dig[i] = tab[v[i]];
    }
    let mut dp: Vec<Vec<usize>> = vec![Vec::new(); n + 1];
    dp[0] = vec![0; 9];
    for i in 0..n {
        if dp[i].len() == 0 {
            continue;
        }
        for j in 0..m {
            if i + dig[j] > n {
                continue;
            }
            let mut t = dp[i].clone();
            t[j] += 1;
            dp[i + dig[j]] = max_digs(dp[i + dig[j]].clone(), t);
        }
    }
    print_digs(&dp[n], &v);
}

fn max_digs(x: Vec<usize>, y: Vec<usize>) -> Vec<usize> {
    let xsum: usize = x.iter().clone().sum();
    let ysum: usize = y.iter().clone().sum();
    if xsum > ysum {
        return x;
    } else if xsum < ysum {
        return y;
    }
    for i in 0..x.len() {
        if x[i] > y[i] {
            return x;
        } else if x[i] < y[i] {
            return y;
        }
    }
    x
}

fn print_digs(x: &Vec<usize>, v: &Vec<usize>) {
    for i in 0..x.len() {
        for j in 0..x[i] {
            print!("{}", v[i] + 1);
        }
    }
    println!("");
}
