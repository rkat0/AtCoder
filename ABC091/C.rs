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
    let n = read::<usize>();
    let mut rv = read_mat::<i32>(n);
    rv.sort_by_key(|v| v[0]);
    let mut bv = read_mat::<i32>(n);
    bv.sort_by_key(|v| v[0]);
    let mut ans = 0;
    for b in bv {
        let mut maxy = -1;
        let mut maxi = 0;
        for i in 0..rv.len() {
            if rv[i][0] >= b[0] {break;}
            if rv[i][1] < b[1] && rv[i][1] >= maxy {
                maxy = rv[i][1];
                maxi = i;
            }
        }
        if maxy >= 0 {
            ans += 1;
            rv.remove(maxi);
        }
    }
    println!("{}",ans);
}
