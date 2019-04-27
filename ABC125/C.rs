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
    let segt = SegT::from_vec(v);
    let mut ans = 0;
    for i in 0..n {
        ans = std::cmp::max(ans, segt.get(i));
    }
    println!("{}", ans);
}

fn gcd(n: usize, m: usize) -> usize {
    let mut l = std::cmp::min(n, m);
    let mut r = std::cmp::max(n, m);
    while l > 0 {
        let t = l;
        l = r % l;
        r = t;
    }
    r
}

#[derive(Debug)]
struct SegT {
    v: Vec<usize>,
    n: usize
}

impl SegT {
    fn from_vec(xs: Vec<usize>) -> SegT {
        let l = xs.len();
        let mut s = 1;
        while s < l {
            s *= 2;
        }
        let mut v = vec![0; 2 * s - 1];
        for i in 0..l {
            v[i + s - 1] = xs[i];
        }
        for i in 2..s + 1 {
            let p = s - i;
            v[p] = gcd(v[2 * p + 1], v[2 * p + 2]);
        }
        SegT{v: v, n: s}
    }

    fn get(&self, idx: usize) -> usize {
        let n = self.n;
        self.get_rec(idx, 0, 0, n - 1)
    }

    fn get_rec(&self, idx: usize, p: usize, pl: usize, pr: usize) -> usize {
        if pl > idx || pr < idx {
            self.v[p]
        } else if pl == idx && pr == idx {
            0
        } else {
            let a = self.get_rec(idx, 2 * p + 1, pl, (pl + pr - 1) / 2);
            let b = self.get_rec(idx, 2 * p + 2, (pl + pr + 1) / 2, pr);
            gcd(a, b)
        }
    }
}
