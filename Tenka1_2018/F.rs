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

use std::collections::BTreeMap;

fn main() {
    input!{
        n: usize,
        v: [usize; n]
    }
    let d = 998244353;

    let mut w = v;
    if w[0] == w[n - 1] {
        let mut i = 1;
        while i < n && w[0] == w[i] {
            i += 1;
        }
        if i < n {
            let mut l = w[0..i].into_iter().map(|x| *x).collect();
            w = w[i..n].into_iter().map(|x| *x).collect();
            w.append(&mut l);
        }
    }

    let mut counts: BTreeMap<usize, (usize, usize)> = BTreeMap::new();
    let mut counts_vec: Vec<(usize, usize)> = Vec::new();
    let mut c = 0;
    let mut num = w[0];
    for i in 0..w.len() {
        let x = w[i];
        if x == num {
            c += 1;
        } else {
            if counts.insert(num, (c, counts_vec.len() + 1)) != None {
                println!("0");
                return;
            }
            counts_vec.push((num,c));
            c = 1;
            num = x;
        }
    }
    if counts.insert(num, (c, counts_vec.len() + 1)) != None {
        println!("0");
        return;
    }
    counts_vec.push((num,c));
    counts_vec.insert(0, (num,c));
    let t = counts_vec[1];
    counts_vec.push(t);

    if w.iter().all(|x| *x == 1) {
        println!("{}", (1..n+1).fold(1, |ac, x| ac * x % d));
        return;
    }

    let len;
    if let Some(c) = counts.get(&1) {
        len = c.0;
    } else {
        println!("0");
        return;
    }

    let mut ans = 1;
    let mut lsum = 0;
    let mut p = 0;
    for m in 1..n+1 {
        if let Some(x) = counts.get(&m) {
            let (c, i) = *x;
            if c > len {
                println!("0");
                return;
            }
            if m > counts_vec[i - 1].0 && m > counts_vec[i + 1].0 {
                ans = ans * (len - c + 1) % d;
                p -= 1;
            } else if m < counts_vec[i - 1].0 && m < counts_vec[i + 1].0 {
                if c != len {
                    println!("0");
                    return;
                }
                p += 1;
            }
            lsum += c;
        } else {
            let mul = lsum + 1 - (len - 1) * p - m;
            ans = ans * mul % d;
            if mul == 0 {
                break;
            }
        }
    }
    println!("{}", ans);
}
