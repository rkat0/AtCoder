use std::io::*;
use std::collections::HashMap;

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
    let wh = read_vec::<usize>();
    let (w,h) = (wh[0],wh[1]);
    let n = read::<usize>();
    let xy = read_mat::<usize>(n);
    let mut dp: HashMap<(usize,usize,usize,usize),usize> = HashMap::new();
    let ans = solve(0,0,w,h,&xy.iter().map(|xy| (xy[0]-1,xy[1]-1)).collect(),&mut dp);
    println!("{}",ans);
}

fn solve(w0: usize, h0: usize, w: usize, h: usize, ms: &Vec<(usize,usize)>, dp: &mut HashMap<(usize,usize,usize,usize),usize>) -> usize {
    if w0 == w || h0 == h { return 0; }
    if let Some(&x) = dp.get(&(w0,h0,w,h)) {
        return x;
    }
    let mut ans = 0;
    let max = (w - w0) * (h - h0);
    let s0 = w - w0 + h - h0 - 1;
    for (x,y) in ms.iter().cloned() {
        if x >= w0 && x < w && y >= h0 && y < h {
            let tmp = s0 + solve(w0,h0,x,y,&ms,dp) + solve(w0,y+1,x,h,&ms,dp) + solve(x+1,h0,w,y,&ms,dp) + solve(x+1,y+1,w,h,&ms,dp);
            if ans < tmp {
                ans = tmp;
                if ans == max {
                    break;
                }
            }
        }
    }
    dp.insert((w0,h0,w,h),ans);
    ans
}
