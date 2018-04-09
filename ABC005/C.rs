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

fn main() {
    let t = read::<usize>();
    let n = read::<usize>();
    let av = read_vec::<usize>();
    let m = read::<usize>();
    let bv = read_vec::<usize>();
    let mut ans = "no";
    if n >= m {
        let mut ai = 0;
        let mut bi = 0;
        while n - ai >= m - bi && ai < n {
            while ai < n && av[ai] + t < bv[bi] {
                ai += 1;
            }
            if ai == n || av[ai] > bv[bi] {
                break;
            }
            if av[ai] + t >= bv[bi] {
                bi += 1;
            }
            ai += 1;
            if bi == m {
                ans = "yes";
                break;
            }
        }
    }
    println!("{}",ans);
}
