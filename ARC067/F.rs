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
        v: [usize; n-1],
        w: [[i64; m]; n]
    }
    let mut wt = vec![vec![0; n]; m];
    for i in 0..m {
        for j in 0..n {
            wt[i][j] = w[j][i];
        }
    }
    let mut dist = vec![0; n];
    for i in 1..n {
        dist[i] = dist[i - 1] + v[i - 1];
    }
    let mut tab: Vec<Vec<i64>> = vec![vec![0; n+1]; n+1];
    for i in 0..m {
        fill(&mut tab, 0, n, &wt[i]);
    }

    for i in 1..n+1 {
        tab[i][0] += tab[i - 1][0];
        tab[0][i] += tab[0][i - 1];
    }
    for i in 1..n+1 {
        for j in 1..n+1 {
            tab[i][j] += tab[i - 1][j] + tab[i][j - 1] - tab[i - 1][j - 1];
        }
    }

    let mut ans = 0;
    for i in 0..n {
        for j in i..n {
            let a = dist[j] - dist[i];
            let b = tab[i][j] as usize;
            if b > a && ans < b - a {
                ans = b - a;
            }
        }
    }
    println!("{}",ans);
}

fn fill(tab: &mut Vec<Vec<i64>>, l: usize, r: usize, v: &Vec<i64>) {
    if l >= r {
        return;
    }
    let mut k = 0;
    let mut max = 0;
    for i in l..r {
        if v[i] > max {
            k = i;
            max = v[i];
        }
    }
    tab[l][k] += max;
    tab[l][r] -= max;
    tab[k + 1][k] -= max;
    tab[k + 1][r] += max;
    fill(tab, l, k, v);
    fill(tab, k + 1, r, v);
}
