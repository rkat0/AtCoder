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
        h: usize,
        w: usize,
        n: usize,
        v: [[usize; 2]; n]
    }
    let d = 1000000007;
    let mut ws = v;
    ws.sort_by(|x, y| if x[0] == y[0] {x[1].cmp(&y[1])} else {x[0].cmp(&y[0])});
    let mut tab: Vec<usize> = vec![0; n];
    let mut ss: Vec<usize> = vec![1; h + w];
    for i in 1..h + w {
        ss[i] = ss[i - 1] * i % d;
    }
    let sis: Vec<usize> = ss.iter().clone().map(|&x| inv_m(x, d)).collect();
    for i in 0..n {
        tab[i] = comb_mem(&ss, &sis, ws[i][0] + ws[i][1] - 2, ws[i][0] - 1, d);
        for j in 0..i {
            if ws[i][1] < ws[j][1] {
                continue;
            }
            tab[i] = (tab[i] + d - tab[j] * comb_mem(&ss, &sis, ws[i][0] + ws[i][1] - ws[j][0] - ws[j][1], ws[i][0] - ws[j][0], d) % d) % d;
        }
    }
    let mut ans = comb_mem(&ss, &sis, h + w - 2, h - 1, d);
    for i in 0..n {
        ans = (ans + d - tab[i] * comb_mem(&ss, &sis, h + w - ws[i][0] - ws[i][1], h - ws[i][0], d) % d) % d;
    }
    println!("{}", ans);
}

fn pow_m(n: usize, mut p: usize, d: usize) -> usize {
    let mut r = n;
    let mut ret = 1;
    while p > 0 {
        if p % 2 == 0 {
            r = r * r % d;
            p /= 2;
        } else {
            ret = ret * r % d;
            p -= 1;
        }
    }
    ret
}

// only when d is prime
fn inv_m(n: usize, d: usize) -> usize {
    pow_m(n, d - 2, d)
}

fn comb_mem(ss: &Vec<usize>, sis: &Vec<usize>, n: usize, r: usize, d: usize) -> usize {
    ss[n] * sis[n - r] % d * sis[r] % d
}
