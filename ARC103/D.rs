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
        v: [[i64; 2]; n]
    }
    let odd = (v[0][0] + v[0][1]).abs() % 2;
    for i in 1..n {
        let x = v[i][0];
        let y = v[i][1];
        if (x + y).abs() % 2 != odd {
            println!("-1");
            return;
        }
    }
    let m = 31;
    if odd == 0 {
        println!("{}", m + 1);
        print!("1 ");
    } else {
        println!("{}", m);
    }
    for i in 0..m-1 {
        print!("{} ", 1 << i);
    }
    println!("{}", 1 << m-1);
    for i in 0..n {
        let mut x = v[i][0];
        let y = v[i][1];
        let mut moves: Vec<char> = Vec::new();
        if odd == 0 {
            x -= 1;
            moves.push('R');
        }
        let u = x + y;
        let v = x - y;
        let mut tu = ((1 << 31) - 1 - u) >> 1;
        let mut su = vec![false; 31];
        for j in 0..31 {
            su[j] = tu & 1 == 0;
            tu >>= 1;
        }
        let mut tv = ((1 << 31) - 1 - v) >> 1;
        let mut sv = vec![false; 31];
        for j in 0..31 {
            sv[j] = tv % 2 == 0;
            tv >>= 1;
        }
        for j in 0..31 {
            if su[j] {
                if sv[j] {
                    moves.push('R');
                } else {
                    moves.push('U');
                }
            } else if sv[j] {
                moves.push('D');
            } else {
                moves.push('L');
            }
        }
        println!("{}",moves.into_iter().collect::<String>());
    }
}
