use std::io::*;
use std::str::FromStr;

fn get_line(buf : &mut String) {
	let stdin = stdin();
	let _ = stdin.lock().read_line(buf);
}

fn get_line_as<T: FromStr>(sep: char) -> Vec<T>
where <T as FromStr>::Err: std::fmt::Debug {
	let mut buf = String::new();
	get_line(&mut buf);
	buf.trim().split(sep).map(|w| w.parse().unwrap()).collect()
}

fn get_as<T: FromStr>() -> T
where <T as FromStr>::Err: std::fmt::Debug {
	let mut buf = String::new();
	get_line(&mut buf);
	buf.trim().parse().unwrap()
}

fn main() {
    let v = get_line_as::<i32>(' ');
    let a = v[0]-v[4];
    let b = v[1]-v[5];
    let c = v[2]-v[4];
    let d = v[3]-v[5];
	println!("{}",f64::from((a*d-b*c).abs())/2.0);
}
