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
        h: [usize; n],
        a: [usize; n]
    }
    let mut tab = BIT::new(n + 1);
    for i in 0..n {
        let pre = tab.max(h[i] - 1);
        tab.update(h[i], pre + a[i] as i64);
    }
    println!("{}", tab.max(n));
}

struct BIT {
    v: Vec<i64>,
    n: usize
}

impl BIT {
    fn new(n: usize) -> BIT {
        BIT{v: vec![0; n + 1], n: n}
    }

    // update i th element by x
    fn update(&mut self, i: usize, x: i64) {
        let mut id = i as i64 + 1;
        while id <= self.n as i64 && self.v[id as usize] < x {
            self.v[id as usize] = x;
            id += id & -id;
        }
    }

    // max in 0 to i th element
    fn max(&self, i: usize) -> i64 {
        let mut id = i + 1;
        let mut ret = i64::min_value();
        while id > 0 {
            ret = std::cmp::max(ret, self.v[id]);
            id &= id - 1;
        }
        ret
    }
}
