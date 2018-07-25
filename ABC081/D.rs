use std::io::*;

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

fn main() {
    let n = read::<usize>();
    let v = read_vec::<i64>();
    let mut mi = 0;
    let mut max = 0;
    for i in 0..n {
        if v[i].abs() > max {
            mi = i;
            max = v[i].abs();
        }
    }
    if max == 0 {
        println!("0");
    } else {
        println!("{}", 2 * n - 1);
        for i in 0..n {
            println!("{} {}",mi+1,i+1);
        }
        if v[mi] > 0 {
            for i in 1..n {
                println!("{} {}",i,i+1);
            }
        } else {
            for i in 1..n {
                println!("{} {}",n-i+1,n-i);
            }
        }
    }
}
