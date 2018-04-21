use std::io::*;
use std::cmp;

fn read<T: std::str::FromStr>() -> T {
    let stdin = stdin();
    let mut buf = String::new();
	stdin.lock().read_line(&mut buf);
	buf.trim().parse().ok().unwrap()
}

fn read_vec<T: std::str::FromStr>() -> Vec<T> {
	read::<String>().trim().split_whitespace()
        .map(|w| w.parse().ok().unwrap()).collect()
}

fn read_mat<T: std::str::FromStr>(n: usize) -> Vec<Vec<T>> {
    (0..n).map(|_| read_vec()).collect()
}

fn main() {
    let v = read_vec::<i64>();
    let (n,c) = (v[0] as usize,v[1]);
    let sus = read_mat::<i64>(n);
    let mut cw: Vec<i64> = vec![0];
    let mut ccw: Vec<i64> = vec![0];
    let mut cwr: Vec<(usize,i64)> = vec![(0,0)];
    let mut ccwr: Vec<(usize,i64)> = vec![(0,0)];
    let mut cw_t = 0;
    let mut ccw_t = 0;
    for i in 0..n {
        cw_t += sus[i][1];
        ccw_t += sus[n-i-1][1];

        let mut tmp = cw_t - sus[i][0];
        let cw_new = cmp::max(cw[i],tmp);
        cw.push(cw_new);

        tmp = ccw_t - (c - sus[n-i-1][0]);
        let ccw_new = cmp::max(ccw[i],tmp);
        ccw.push(ccw_new);

        if i < n / 2 {
            tmp = cw_t - 2 * sus[i][0];
            if 0 < tmp {
                cwr.push((i+1,tmp));
            }

            tmp = ccw_t - 2 * (c - sus[n-i-1][0]);
            if 0 < tmp {
                ccwr.push((i+1,tmp));
            }
        }
    }
    let cwr_max = cwr.iter().map(|x| x.1 + ccw[n - x.0]).max().unwrap();
    let ccwr_max = ccwr.iter().map(|x| x.1 + cw[n - x.0]).max().unwrap();
    let ans = cmp::max(cmp::max(cw[n],ccw[n]),cmp::max(cwr_max,ccwr_max));
    println!("{}",ans);
}
