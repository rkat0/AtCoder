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
        k: usize,
        mut td: [[usize; 2]; n]
    }
    td.sort_by(|x, y| y[1].cmp(&x[1]));
    let mut tab: Vec<Vec<usize>> = Vec::new();
    let mut map: HashMap<usize, usize> = HashMap::new();
    for i in 0..n {
        if let Some(&idx) = map.get(&td[i][0]) {
            tab[idx].push(td[i][1]);
        } else {
            map.insert(td[i][0], tab.len());
            tab.push(vec![td[i][1]]);
        }
    }
    let mut ans: usize = 0;
    let mut max: usize = 0;
    let mut s: Vec<usize> = Vec::new();
    for i in 0..tab.len() {
        let mut ds = tab[i].clone();
        ans += ds[0] + 2 * i + 1;
        ds.remove(0);
        s.append(&mut ds);
        if s.len() + i + 1 >= k {
            s.sort_by(|x, y| y.cmp(&x));
            s.truncate(k - i - 1);
            max = std::cmp::max(max, ans + s.iter().clone().sum::<usize>());
        }
        if i == k - 1 {
            break;
        }
    }
    println!("{}", max);
}
