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

use std::collections::HashMap;

fn main() {
    input!{
        n: usize,
        m: usize
    }
    let mut mem: HashMap<(usize, usize), usize> = HashMap::new();
    let ans = count((1 << (n + 2)) - 3, m, &mut mem);
    println!("{}", ans);
}

fn count(n: usize, m: usize, mem: &mut HashMap<(usize, usize), usize>) -> usize {
    if let Some(x) = mem.get(&(n, m)) {
        return *x;
    }
    let x = if m > n - 1 {
        (n + 1) / 2
    } else if m > (n + 1) / 2 {
        count((n - 3) / 2, m - (n + 1) / 2, mem) + 1 + count((n - 3) / 2, (n - 3) / 2, mem)
    } else if m == (n + 1) / 2 {
        1 + count((n - 3) / 2, (n - 3) / 2, mem)
    } else if m > 1 {
        count((n - 3) / 2, m - 1, mem)
    } else {
        0
    };
    mem.insert((n, m), x);
    x
}
