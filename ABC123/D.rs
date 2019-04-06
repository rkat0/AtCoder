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
        x: usize,
        y: usize,
        z: usize,
        k: usize,
        mut xs: [usize; x],
        mut ys: [usize; y],
        mut zs: [usize; z]
    }
    xs.sort_by(|a, b| b.cmp(&a));
    ys.sort_by(|a, b| b.cmp(&a));
    zs.sort_by(|a, b| b.cmp(&a));
    let kth = {
        let mut l = 0;
        let mut r = 30000000001;
        while l + 1 < r {
            let m = (l + r) / 2;
            let ord = nth(m, &xs, &ys, &zs);
            if ord >= k {
                l = m;
            } else {
                r = m;
            }
        }
        l
    };
    let mut ans = Vec::new();
    for a in xs.iter() {
        for b in ys.iter() {
            for c in zs.iter() {
                if a + b + c < kth {
                    break;
                }
                ans.push(a + b + c);
            }
        }
    }
    ans.sort_by(|a, b| b.cmp(&a));
    for i in 0..k {
        println!("{}", ans[i]);
    }
}

fn nth(x: usize, xs: &Vec<usize>, ys: &Vec<usize>, zs: &Vec<usize>) -> usize {
    let mut ret = 0;
    for i in xs {
        for j in ys {
            let ij = i + j;
            if ij + zs[0] < x {
                continue;
            }
            let mut l = 0;
            let mut r = zs.len();
            while l + 1 < r {
                let m = (l + r) / 2;
                if ij + zs[m] >= x {
                    l = m;
                } else {
                    r = m;
                }
            }
            ret += r;
        }
    }
    ret
}
