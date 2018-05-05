use std::io::*;
use std::iter;

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
    let (h,w) = (v[0],v[1]);
    let mut s: Vec<Vec<char>> = (0..h).map(|_| read::<String>().chars().collect()).collect();

    for i in 0..h {
        s[i].insert(0,'.');
        s[i].push('.')
    }
    s.insert(0,iter::repeat('.').take(w+2).collect());
    s.push(iter::repeat('.').take(w+2).collect());    

    for i in 1..h-1 {
        for j in 1..w-1 {
            if s[i][j] == '.' {
                continue;
            }
            if s[i-1][j] == '.' && s[i][j-1] == '.' && s[i+1][j] == '.' && s[i][j+1] == '.' {
                println!("No");
                return;
            }
        }
    }
    println!("Yes");
}
