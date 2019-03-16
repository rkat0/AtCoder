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
        k: usize1,
        p: [usize1; n],
        q: [usize1; n]
    }
    let mut tab: Vec<Vec<usize>> = vec![Vec::new(); 6];
    tab[0] = p.clone();
    tab[1] = q.clone();
    for i in 2..6 {
        tab[i] = perm_prod(&perm_inv(&tab[i - 2]), &tab[i - 1]);
    }
    let a = perm_pow(&perm_prod(&p, &tab[3]), k / 6);
    let ans = perm_prod(&perm_prod(&perm_inv(&a), &tab[k % 6]), &a);
    print!("{}", ans[0] + 1);
    for i in 1..n {
        print!(" {}", ans[i] + 1);
    }
    println!("");
}

fn perm_prod(p: &Vec<usize>, q: &Vec<usize>) -> Vec<usize> {
    let n = p.len();
    (0..n).map(|i| q[p[i]]).collect()
}

fn perm_pow(a: &Vec<usize>, mut p: usize) -> Vec<usize> {
    let n = a.len();
    let mut ret: Vec<usize> = (0..n).collect();
    let mut r = a.clone();
    while p > 0 {
        if p % 2 == 0 {
            r = perm_prod(&r, &r);
            p /= 2;
        } else {
            ret = perm_prod(&ret, &r);
            p -= 1;
        }
    }
    ret
}

fn perm_inv(p: &Vec<usize>) -> Vec<usize> {
    let n = p.len();
    let mut ret: Vec<usize> = vec![0; n];
    for i in 0..n {
        ret[p[i]] = i;
    }
    ret
}
