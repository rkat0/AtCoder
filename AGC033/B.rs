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
        h: usize,
        w: usize,
        n: usize,
        sr: usize,
        sc: usize,
        mut s: chars,
        mut t: chars
    }
    s.reverse();
    t.reverse();
    let mut l = 1;
    let mut r = w;
    let mut u = 1;
    let mut d = h;
    for i in 0..n {
        match t[i] {
            'L' => {if r < w {r += 1;}},
            'R' => {if l > 1 {l -= 1;}},
            'U' => {if d < h {d += 1;}},
            'D' => {if u > 1 {u -= 1;}},
            _ => {}
        }
        match s[i] {
            'L' => {l += 1;},
            'R' => {r -= 1;},
            'U' => {u += 1;},
            'D' => {d -= 1;},
            _ => {}
        }
        if l > r || u > d {
            break;
        }
    }
    println!("{}", if l <= sc && sc <= r && u <= sr && sr <= d {"YES"} else {"NO"});
}
