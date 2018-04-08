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
    let v = read_vec::<i64>();
    let (r, mut g, mut b) = (v[0],v[1],v[2]);

    let mut ans = step(r);
    let mut rl = - r/2;
    let mut rr = (r-1)/2;

    let mut bl = - (b-1)/2;
    let mut br = b/2;
    if 100 + bl > rr - 100 {
        ans += step(b);
    } else {
        bl = rr - 199;
        br = - bl + 1;
        let b_t = 2*br;
        b -= b_t;
        ans += step(b_t);
        while b > 0 {
            let costr = br + 1;
            let costl = - bl + 1 - rl + 1 - rr;
            if costr <= costl {
                ans += costr;
                br += 1;
            } else {
                ans += costl;
                bl -= 1;
                rl -= 1;
                rr -= 1;
            }
            b -= 1;
        }
    }

    let mut gl = rr - 99;
    let mut gr = 99 + bl;
    let min_lr = cmp::min(gl.abs(),gr.abs());

    if gl < 0 && gr > 0 && 2 * min_lr + 1 >= g {
        ans += step(g);
    } else {
        let mut rg = true;
        let mut gb = true;
        if gl > gr || gr + gl == 0 {
            rg = false;
            gb = false;
        } else if min_lr == gr.abs() {
            if gr > 0 {
                gl = -gr;
            } else {
                gl = gr + 1;
            }
            gb = false;
        } else {
            if gl < 0 {
                gr = -gl;
            } else {
                gr = gl - 1;
            }
            rg = false;
        }
        let g_t = gr - gl + 1;
        g -= g_t;
        ans += step(g_t);
        while g > 0 {
            let costr = (gr+1).abs() + if gb { 0 } else { -bl.abs() + (br+1).abs() };
            let costl = (gl-1).abs() + if rg { 0 } else { (rl-1).abs() - rr.abs() };
            if costr <= costl {
                ans += costr;
                gr += 1;
                if !gb {
                    bl += 1;
                    br += 1;
                } else if gr - bl == 99 {
                    gb = false;
                }
            } else {
                ans += costl;
                gl -= 1;
                if !rg {
                    rl -= 1;
                    rr -= 1;
                } else if rr - gl == 99 {
                    rg = false;
                }
            }
            g -= 1;
        }
    }
    println!("{}",ans);
}

fn step(n: i64) -> i64 {
    let m = n / 2;
    if n % 2 == 1 { m * (m + 1) } else { m * m }
}
