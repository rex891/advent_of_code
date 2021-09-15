fn main() {
	enum IpAddr {
		V4(String),
		V6(String),
	}
	use IpAddr::*;
	let home = V4(String::from("127"));
}
