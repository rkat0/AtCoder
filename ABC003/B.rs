use std::io::*;

fn read<T: std::str::FromStr>() -> T {
    let stdin = stdin();
    let mut buf = String::new();
	let _ = stdin.lock().read_line(&mut buf);
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
    let mut s = read::<String>();
    let mut t = read::<String>();
    let wild = vec!['a','t','c','o','d','e','r'];
    loop {
        if let (Some(cs),Some(ct)) = (s.pop(),t.pop()) {
            if !(cs==ct || (cs == '@' && wild.contains(&ct)) || (ct == '@' && wild.contains(&cs))) {
                println!("You will lose");
                break;
            }
        } else {
            println!("You can win");
            break;
        }
    }
}
