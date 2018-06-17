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
    let v = read_vec::<usize>();
    let (n,m) = (v[0],v[1]);
    let w = read_mat::<i64>(n);
    let mut score = vec![Vec::with_capacity(n);8];
    for x in w {
        let s = x[0] + x[1] + x[2];
        score[0].push(s);
        score[1].push(s - 2*x[0]);
        score[2].push(s - 2*x[1]);
        score[3].push(s - 2*x[2]);
        score[4].push(-s + 2*x[0]);
        score[5].push(-s + 2*x[1]);
        score[6].push(-s + 2*x[2]);
        score[7].push(-s);
    }
    let mut ans = 0;
    for mut ss in score {
        ss.sort_by(|a,b| b.cmp(a));
        let t = ss.iter().take(m).sum();
        if t > ans {
            ans = t;
        }
    }
    println!("{}",ans);
}
