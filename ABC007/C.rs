use std::io::*;
use std::collections::VecDeque;

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
    let v = read_mat::<usize>(3);
    let m:Vec<Vec<char>> = (0..v[0][0]).map(|_| read::<String>().chars().collect()).collect();
    let ans = bfs(&m,v[1][0]-1,v[1][1]-1,v[2][0]-1,v[2][1]-1);
    println!("{}",ans);
}

fn bfs(m: &Vec<Vec<char>>, si: usize, sj: usize, gi: usize, gj: usize) -> i64 {
    if si == gi && sj == gj { return 0; }
    let mut q: VecDeque<(usize,usize)> = VecDeque::new();
    let mut tab: Vec<Vec<i64>> = vec![vec![-1;m[0].len()];m.len()];
    q.push_back((si,sj));
    tab[si][sj]=0;
    while let Some((i,j)) = q.pop_front() {
        for ij in [(i+1,j),(i,j+1),(i-1,j),(i,j-1)].iter() {
            let ii = ij.0;
            let jj = ij.1;
            if m[ii][jj] == '.' && tab[ii][jj] == -1 {
                tab[ii][jj] = tab[i][j]+1;
                q.push_back((ii,jj));
            }
        }
    }
    tab[gi][gj]
}
