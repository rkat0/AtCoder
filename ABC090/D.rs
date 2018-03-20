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

fn read_mat<T: std::str::FromStr>(n: usize) -> Vec<Vec<T>> {
    (0..n).map(|_| read_vec()).collect()
}

fn main() {
    let v = read_vec::<usize>();
    let (n,k) = (v[0],v[1]);
    let mut ans = 0;
    let a = n-k;
    for i in k+1..n+1 {
        let b = n-i+1;
        let mut r = b/i;
        let mut c = (r+1)*i;
        ans += r*(c-b)+i-k;
        loop {
            if c>a {
                ans-=r*(c-a-1);
                break;
            }
            r+=1;
            ans += r*i;
            c+=i;
        }
    }
    if k==0 {
        ans -= n;
    }
    println!("{}",ans);
}
