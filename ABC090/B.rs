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

fn read_mat<T: std::str::FromStr>(n: usize) -> Vec<Vec<T>> {
    (0..n).map(|_| read_vec()).collect()
}

fn is_palindrome(n:u32) -> bool {
    let mut dig:Vec<u32> = Vec::new();
    let mut rev:Vec<u32> = Vec::new();
    let mut m = n;
    while m > 0 {
        dig.push(m % 10);
        rev.push(m % 10);
        m /= 10;
    }
    rev.reverse();
    dig.iter().zip(rev.iter()).all(|(x,y)| x == y)
}

fn main() {
    let v = read_vec::<u32>();
    let mut count = 0;
    for n in v[0]..v[1]+1 {
        if is_palindrome(n) {
            count+=1;
        }
    }
    println!("{}",count);
}
