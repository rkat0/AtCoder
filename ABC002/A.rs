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
	println!("{}", std::cmp::max(v[0],v[1]));
}
