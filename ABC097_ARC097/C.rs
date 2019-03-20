use std::io::*;
use std::collections::HashSet;

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
    let mut s = read_vec_char();
    let k = read::<usize>();
    let mut subs: Vec<Vec<char>> = vec![vec!['z','z'];5];
    let mut open: Vec<bool> = vec![false;5];
    for c in s {
        let mut cv = vec![c];
        for i in 0..4 {
            let ii = 3-i;
            if open[ii] {
                let mut tmp = subs[ii].clone();
                tmp.push(c);
                open[ii]=false;
                for j in ii+1..5 {
                    if subs[j] < tmp {
                        continue;
                    } else if subs[j] == tmp {
                        open[j]=true;
                        break;
                    } else {
                        subs.insert(j,tmp);
                        open.insert(j,true);
                        subs.pop();
                        open.pop();
                        break;
                    }
                }
            }
        }
        for i in 0..5 {
            if subs[i] < cv {
                continue;
            } else if subs[i] == cv {
                open[i]=true;
                break;
            } else {
                subs.insert(i,cv);
                open.insert(i,true);
                subs.pop();
                open.pop();
                break;
            }
        }
    }
    println!("{}",subs[k-1].iter().map(|x| *x).collect::<String>());
}
