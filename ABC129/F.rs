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
        l: usize,
        mut a: usize,
        mut b: usize,
        m: usize
    }
    let mut d: Vec<usize> = vec![0; 19];
    let mut k = 1;
    loop {
        let t = 10usize.pow(k as u32) as usize;
        if t < a {
            k += 1;
            continue;
        }
        d[k] = (t - a + b - 1) / b;
        if d[k] >= l {
            d[k] = l;
            k += 1;
            break;
        }
        k += 1;
    }
    a = a % m;
    b = b % m;
    let mut v = vec![0, a, 1];
    let mut r = vec![vec![10, 1, 0], vec![0, 1, b], vec![0, 0, 1]];
    for i in 1..k {
        v = matvec(&matpow(&r, d[i] - d[i - 1], m), &v, m);
        r[0][0] = r[0][0] * 10 % m;
    }
    println!("{}", v[0]);
}

fn matmul(a: &Vec<Vec<usize>>, b: &Vec<Vec<usize>>, d: usize) -> Vec<Vec<usize>> {
    let l = a.len();
    let m = b.len();
    let n = b[0].len();
    let mut c: Vec<Vec<usize>> = vec![vec![0; n]; l];
    for i in 0..l {
        for j in 0..m {
            for k in 0..n {
                c[i][k] = (c[i][k] + a[i][j] * b[j][k]) % d
            }
        }
    }
    c
}

fn matvec(a: &Vec<Vec<usize>>, b: &Vec<usize>, d: usize) -> Vec<usize> {
    let l = a.len();
    let m = b.len();
    let mut c: Vec<usize> = vec![0; l];
    for i in 0..l {
        for j in 0..m {
            c[i] = (c[i] + a[i][j] * b[j]) % d
        }
    }
    c
}

fn matpow(a: &Vec<Vec<usize>>, mut p: usize, d: usize) -> Vec<Vec<usize>> {
    let n = a.len();
    let mut ret = vec![vec![0; n]; n];
    for i in 0..n {
        ret[i][i] = 1;
    }
    let mut r = a.clone();
    while p > 0 {
        if p % 2 == 0 {
            r = matmul(&r, &r, d);
            p /= 2;
        } else {
            ret = matmul(&ret, &r, d);
            p -= 1;
        }
    }
    ret
}
