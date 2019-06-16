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
        v: [(f64, f64, chars); n]
    }
    let fmax = 10f64.powi(8i32);
    let mut ps: Vec<Vec<f64>> = vec![vec![fmax, -fmax, fmax, -fmax]; 4];
    for (x, y, d) in v {
        let i = if d[0] == 'R' {
            0
        } else if d[0] == 'L' {
            1
        } else if d[0] == 'U' {
            2
        } else {
            3
        };
        ps[i][0] = ps[i][0].min(x);
        ps[i][1] = ps[i][1].max(x);
        ps[i][2] = ps[i][2].min(y);
        ps[i][3] = ps[i][3].max(y);
    }

    let mut ts: Vec<f64> = Vec::new();
    let mut tmp = ps[2][0].min(ps[3][0]);
    let mut t1 = tmp - ps[0][0];
    let mut t2 = ps[1][0] - tmp;
    ts.push(t1);
    ts.push(t2);
    ts.push((t1 + t2) / 2.0);
    let xmin = [ps[0][0], tmp, ps[1][0]];
    tmp = ps[2][1].max(ps[3][1]);
    t1 = ps[1][1] - tmp;
    t2 = tmp - ps[0][1];
    ts.push(t1);
    ts.push(t2);
    ts.push((t1 + t2) / 2.0);
    let xmax = [ps[1][1], tmp, ps[0][1]];
    tmp = ps[0][2].min(ps[1][2]);
    t1 = tmp - ps[2][2];
    t2 = ps[3][2] - tmp;
    ts.push(t1);
    ts.push(t2);
    ts.push((t1 + t2) / 2.0);
    let ymin = [ps[2][2], tmp, ps[3][2]];
    tmp = ps[0][3].max(ps[1][3]);
    t1 = ps[3][3] - tmp;
    t2 = tmp - ps[2][3];
    ts.push(t1);
    ts.push(t2);
    ts.push((t1 + t2) / 2.0);
    let ymax = [ps[3][3], tmp, ps[2][3]];
    ts.push(0.0);
    let mut ans: f64 = fmax * fmax * 4.0;
    for &t in ts.iter() {
        if t < 0.0 {
            continue;
        }
        let x1 = (xmin[0] + t).min(xmin[2] - t).min(xmin[1]);
        let x2 = (xmax[0] - t).max(xmax[2] + t).max(xmax[1]);
        let y1 = (ymin[0] + t).min(ymin[2] - t).min(ymin[1]);
        let y2 = (ymax[0] - t).max(ymax[2] + t).max(ymax[1]);
        ans = ans.min((x2 - x1) * (y2 - y1));
    }
    println!("{}", ans);
}
