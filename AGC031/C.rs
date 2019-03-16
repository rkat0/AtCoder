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
        a: usize,
        b: usize,
    }
    let mut t = a ^ b;
    let mut cnt = 0;
    while t > 0 {
        if t & 1 == 1 {
            cnt += 1;
        }
        t >>= 1;
    }
    if cnt % 2 == 0 {
        println!("NO");
        return;
    }
    println!("Yes");
    let ans = find(n, a ^ b);
    print!("{}", ans[0] ^ a);
    for i in 1..1 << n {
        print!(" {}", ans[i] ^ a);
    }
    println!("");
}

fn find(n: usize, a: usize) -> Vec<usize> {
    let mut ret = Vec::new();
    let mut mask = 1;
    let mut m = n;
    let mut ones = vec![];
    while mask <= a {
        if a & mask == 0 {
            mask <<= 1;
            continue;
        }
        if mask == 1 << n - 1 {
            let mut vec = grays(m).iter().map(|x| add_ones(*x, &ones)).map(|x| add_topone(x, n, ones.len() % 2 == 1)).collect();
            ret.append(&mut vec);
            m = 0;
            break;
        }
        m -= 1;
        let mut vec = grays(m).iter().map(|x| add_ones(*x, &ones)).map(|x| add_zero(x, mask)).map(|x| add_topone(x, n, ones.len() % 2 == 1)).collect();
        ones.push(mask);
        ret.append(&mut vec);
        mask <<= 1;
    }
    if m > 0 {
        let mut vec = grays(m).iter().map(|x| add_ones(*x, &ones)).map(|x| add_topone(x, n, ones.len() % 2 == 1)).collect();
        ret.append(&mut vec);
    }
    ret
}

fn add_ones(x: usize, ones: &Vec<usize>) -> usize {
    let mut ret = x;
    for mask in ones {
        let y = ret & mask - 1;
        ret = (ret - y << 1) | mask | y;
    }
    ret
}

fn add_topone(x: usize, n: usize, flag: bool) -> usize {
    if flag {
        x ^ 1 << n - 1
    } else {
        x
    }
}

fn add_zero(n: usize, mask: usize) -> usize {
    let y = n & mask - 1;
    (n - y << 1) + y
}

fn grays(n: usize) -> Vec<usize> {
    (0..1<<n).map(|x| x ^ (x >> 1)).collect()
}
