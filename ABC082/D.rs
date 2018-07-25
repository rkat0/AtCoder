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

fn read_vec_char() -> Vec<char> {
    read::<String>().chars().collect()
}

fn read_mat<T: std::str::FromStr>(n: usize) -> Vec<Vec<T>> {
    (0..n).map(|_| read_vec()).collect()
}

fn read_mat_char(n: usize) -> Vec<Vec<char>> {
    (0..n).map(|_| read_vec_char()).collect()
}

fn sum_check(v: &Vec<i64>, x: i64) -> bool {
    let m = v.iter().sum::<i64>() - x.abs();
    if m < 0 || m % 2 != 0 {
        return false;
    }
    let n: usize = m as usize / 2;
    let mut b: Vec<bool> = vec![false;n+1];
    b[0] = true;
    for x in v {
        for i in 0..cmp::max(0,n as i64 + 1 - *x) as usize {
            if b[i] {
                b[i + *x as usize] = true;
            }
        }
    }
    b[n]
}

fn main() {
    let s = read_vec_char();
    let v = read_vec::<i64>();
    let (mut x,y) = (v[0],v[1]);
    let mut moves: Vec<Vec<i64>> = vec![Vec::new();2];
    let mut tmp = 0;
    let mut d = 0;
    for c in s {
        if c == 'F' {
            tmp += 1;
        } else {
            moves[d].push(tmp);
            tmp = 0;
            d = (d + 1) % 2;
        }
    }
    moves[d].push(tmp);
    x -= moves[0][0];
    moves[0].remove(0);
    if sum_check(&moves[0],x) && sum_check(&moves[1],y) {
        println!("Yes");
    } else {
        println!("No");
    }
}
