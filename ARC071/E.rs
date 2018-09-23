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
        s: chars,
        t: chars,
        q: usize,
        v: [[usize; 4]; q]
    }
    let mut sc: Vec<usize> = s.iter().map(|c| if *c == 'A' {1} else {2}).scan(0,|ac,x| {*ac += x; Some(*ac)}).collect();
    sc.insert(0,0);
    let mut tc: Vec<usize> = t.iter().map(|c| if *c == 'A' {1} else {2}).scan(0,|ac,x| {*ac += x; Some(*ac)}).collect();
    tc.insert(0,0);
    for i in 0..q {
        if (sc[v[i][1]] - sc[v[i][0]-1]) % 3 == (tc[v[i][3]] - tc[v[i][2]-1]) % 3 {
            println!("YES");
        } else {
            println!("NO");
        }
    }
}
