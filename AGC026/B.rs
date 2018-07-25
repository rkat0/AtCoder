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

fn main() {
    let t = read::<usize>();
    for i in 0..t {
        let v = read_vec::<i64>();
        let (mut a,b,c,d) = (v[0],v[1],v[2],v[3]);
        if a < b || d < b || a % b > c {
            println!("No");
        } else if b <= c+1 {
            println!("Yes");
        } else {
            let k = cmp::max(a+b-c-1,0) / b;
            a -= k * b;
            let e = d % b;
            if e == 0 {
                println!("Yes");
            } else if e < b-c {
                println!("No");
            } else {
                let mut g = b;
                let mut r = d;
                while r % g > 0 {
                    let t = g;
                    g = r % g;
                    r = t;
                }
                let x = if (c-a) % g == 0 { (c-a)/g + 1 } else { (c+g-a-1)/g };
                if x <= (b-a-1) / g {
                    println!("No");
                } else {
                    println!("Yes");
                }
            }
        }
    }
}
