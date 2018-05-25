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
    let v = read_vec::<usize>();
    let (n,k) = (v[0],v[1]);
    let mut map: Vec<Vec<i64>> = vec![vec![0;2*k];k];
    for i in 0..n {
        let s = read::<String>();
        let w: Vec<&str> = s.trim().split_whitespace().collect();
        let x: usize = w[0].parse().ok().unwrap();
        let y: usize = w[1].parse().ok().unwrap();
        let xi = x % (2*k);
        let mut yi = y % (2*k);
        if w[2] == "W" {
            yi = (yi + k) % (2 * k);
        }
        if xi >= k {
            map[xi-k][(yi+k)%(2*k)] += 1;
        } else {
            map[xi][yi] += 1;
        }
    }

    for i in 1..k {
        map[i][0] += map[i-1][0];
    }
    for i in 1..2*k {
        map[0][i] += map[0][i-1];
    }
    for i in 1..k {
        for j in 1..2*k {
            map[i][j] += map[i-1][j] + map[i][j-1] - map[i-1][j-1];
        }
    }

    let mut max = map[k-1][k-1];
    for i in 1..k {
        let sum = map[k-1][k-1] + map[i-1][2*k-1] - 2*map[i-1][k-1];
        max = cmp::max(max,cmp::max(sum,n as i64 - sum));
    }
    for i in 0..k {
        let sum = map[k-1][i+k] - map[k-1][i];
        max = cmp::max(max,cmp::max(sum,n as i64 - sum));
        for j in 1..k {
            let sum = map[k-1][i+k] - map[k-1][i] + map[j-1][2*k-1] + 2*map[j-1][i] - 2*map[j-1][i+k];
            max = cmp::max(max,cmp::max(sum,n as i64 - sum));
        }
    }
    println!("{}",max);
}
