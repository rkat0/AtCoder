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
        mut v: [i64; n]
    }
    v.sort();
    let mut pos: Vec<i64> = vec![];
    let mut neg: Vec<i64> = vec![];
    neg.push(v[0]);
    let mut i = 1;
    while i < n && v[i] < 0 {
        neg.push(v[i]);
        i += 1;
    }
    if i == n {
        pos.push(neg.pop().unwrap());
    }
    while i < n {
        pos.push(v[i]);
        i += 1;
    }
    println!("{}", pos.iter().cloned().sum::<i64>() - neg.iter().cloned().sum::<i64>());
    let pl = pos.len();
    let nl = neg.len();
    let mut t;
    if pl >= nl {
        t = neg[nl - 1];
        for i in nl..pl {
            println!("{} {}", t, pos[i]);
            t -= pos[i];
        }
        println!("{} {}", pos[nl - 1], t);
        t = pos[nl - 1] - t;
        for i in (0..nl - 1).rev() {
            println!("{} {}", neg[i], t);
            t = neg[i] - t;
            println!("{} {}", pos[i], t);
            t = pos[i] - t;
        }
    } else {
        t = pos[pl - 1];
        for i in pl - 1..nl {
            println!("{} {}", t, neg[i]);
            t -= neg[i];
        }
        for i in (0..pl - 1).rev() {
            println!("{} {}", neg[i], t);
            t = neg[i] - t;
            println!("{} {}", pos[i], t);
            t = pos[i] - t;
        }
    }
}
