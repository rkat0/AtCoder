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

    ($next:expr, [ $t:tt ]) => {
        {
            let len = read_value!($next, usize);
            (0..len).map(|_| read_value!($next, $t)).collect::<Vec<_>>()
        }
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
    let d = 998244353;
    let s: usize = v.iter().cloned().sum();
    let mut tab = vec![0; s + 1];
    let mut tab2 = vec![0; s + 1];
    tab[0] = 1;
    tab2[0] = 1;
    let mut psum = 0;
    for i in 0..n {
        let mut next: Vec<usize> = tab.iter().clone().map(|x| x * 2 % d).collect();
        let mut next2 = tab2.clone();
        for j in 0..psum + 1 {
            next[j + v[i]] = (next[j + v[i]] + tab[j]) % d;
            next2[j + v[i]] = (next2[j + v[i]] + tab2[j]) % d;
        }
        psum += v[i];
        tab = next;
        tab2 = next2;
    }
    let mut ans = (pow_m(3, n, d) + 3 * (d - tab.iter().skip((s + 1) / 2).fold(0, |ac, x| (ac + x) % d))) % d;
    if s % 2 == 0 {
        ans = (ans + 3 * tab2[s / 2]) % d;
    }
    println!("{}", ans);
}

fn pow_m(n: usize, mut p: usize, d: usize) -> usize {
    let mut ret = 1;
    let mut r = n;
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
