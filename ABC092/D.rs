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
    let v = read_vec::<usize>();
    let flip = v[0] > v[1];
    let (a,b) = if flip { (v[1],v[0]) } else { (v[0],v[1]) };
    let grid = solve(a,b);
    print_ans(grid,flip);
}

fn solve(a: usize, b: usize) -> Vec<Vec<i32>> {
    let mut grid = vec![vec![0;100];100];
    let mut x = a-1;
    let mut y = b;
    let mut h = 3;
    while x > 0 && y > 0 && h < 97 {
        for i in h..h+3 {
            for j in 0..100 {
                grid[i][j] = 1;
            }
        }
        h += 6;
        x -= 1;
        y -= 1;
    }

    'forx: for i in 0..10 {
        for j in 0..50 {
            if x == 0 {break 'forx;}
            grid[6*i+4][2*j] = 0;
            x -= 1;
        }
    }

    'fory: for i in 0..10 {
        for j in 0..50 {
            if y == 0 {break 'fory;}
            grid[6*i+1][2*j] = 1;
            y -= 1;
        }
    }
    grid
}

fn print_ans(grid: Vec<Vec<i32>>, flip: bool) {
    println!("{} {}",100,100);
    for l in grid {
        for c in l {
            print!("{}",if (c==0) ^ flip { '.' } else { '#' });
        }
        println!("")
    }
}
