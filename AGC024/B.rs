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
    let mut con: Vec<usize> = vec![0;n];
    for i in 0..n {
        let p = read::<usize>();
        if p == 1 {
            con[0] = 1;
        } else if con[p-2] == 0 {
            con[p-1] = 1;
        } else {
            con[p-1] = con[p-2] + 1;
        }
    }
    let max = con.iter().max().unwrap();
    println!("{}",n-max);
}
