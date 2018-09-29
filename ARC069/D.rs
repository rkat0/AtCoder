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
        s: chars
    }
    let mut vs = vec![vec!['S','S'],vec!['S','W'],vec!['W','S'],vec!['W','W']];
    for v in vs.iter_mut() {
        if !order(v, &s) {
            continue;
        }
        for c in v {
            print!("{}", c);
        }
        print!("\n");
        return;
    }
    println!("-1");
}

fn order(v: &mut Vec<char>, o: &Vec<char>) -> bool {
    let l = o.len();
    for i in 1..l {
        if (v[i-1] == 'S') ^ (v[i] == 'S') ^ (o[i] == 'o') {
            v.push('S');
        } else {
            v.push('W');
        }
    }
    let last = v.pop().unwrap();
    last == v[0] && (v[1] == v[l-1]) ^ (v[0] == 'S') ^ (o[0] == 'o')
}
