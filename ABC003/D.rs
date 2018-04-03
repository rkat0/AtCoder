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

const D: usize = 1000000007;

fn count(x: usize, y: usize, d: usize,l: usize) -> usize {
    let r = x * y - d - l;
    let mut ans = count2(r,d,l);
    if r >= x && y >= 2 {
        ans += (D - count2(r-x,d,l)) * 2;
        if r >= 2*x && y >= 3 {
            ans += count2(r-2*x,d,l);
        }
    }
    if r >= y && x >= 2 {
        ans += (D - count2(r-y,d,l)) * 2;
        if r >= 2*y && x >= 3 {
            ans += count2(r-2*y,d,l);
        }
    }
    if x >= 2 && y >= 2 && r >= x+y-1 {
        ans += count2(r-x-y+1,d,l) * 4;
        if r >= 2*(x+y-2) && x >= 3 && y >= 3 {
            ans += count2(r-2*(x+y-2),d,l);
        }
        if r >= 2*x+y-2 && y >= 3 {
            ans += (D - count2(r-(2*x+y-2),d,l)) * 2;
        }
        if r >= x+2*y-2 && x >= 3 {
            ans += (D - count2(r-(x+2*y-2),d,l)) * 2;
        }
    }
    ans % D
}

fn count2(r: usize, d: usize,l: usize) -> usize {
    let xy = r + d + l;
    let (a,b,c) = sort3(d,l,r);
    let mut v: Vec<usize> = (c+1..xy+1).rev().collect();
    for i in (1..b+1).rev() {
        let mut x = i;
        for j in 0..v.len() {
            let g = gcd(x,v[j]);
            x /= g;
            v[j] /= g;
            if x == 1 { break; }
        }
    }
    for i in (1..a+1).rev() {
        let mut x = i;
        for j in 0..v.len() {
            let g = gcd(x,v[j]);
            x /= g;
            v[j] /= g;
            if x == 1 { break; }
        }
    }
    v.iter().fold(1,|c,x| c * x % D)
}

fn sort3(x: usize, y: usize, z: usize) -> (usize,usize,usize) {
    let mut v = [x,y,z];
    v.sort();
    (v[0],v[1],v[2])
}

fn gcd(x: usize,y: usize) -> usize {
    let mut a = std::cmp::max(x,y);
    let mut b = std::cmp::min(x,y);
    while a % b != 0 {
        let c = a % b;
        a = b;
        b = c;
    }
    b
}

fn main() {
    let rc = read_vec::<usize>();
    let (r,c) = (rc[0],rc[1]);
    let xy = read_vec::<usize>();
    let (x,y) = (xy[0],xy[1]);
    let dl = read_vec::<usize>();
    let (d,l) = (dl[0],dl[1]);

    let ans = (r-x+1) * (c-y+1) * count(x,y,d,l) % D;
    println!("{}",ans);
}
