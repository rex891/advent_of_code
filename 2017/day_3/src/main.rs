use std::cmp::min;

enum Facing {
	North,
	South,
	East,
	West,
}

fn main() {
	use crate::Facing::*;

	let mut remaining = 265149;
	let mut location: (isize, isize) = (0, 0);
	let mut direction = East;
	let mut first = true;
	let mut leg = 1;
	let mut _move = |mut rem| {
		// move and change facing diection
		let go = min(leg, rem);
		direction = match direction {
			North => {
				location.1 += go;
				West
			}
			South => {
				location.1 -= go;
				East
			}
			East => {
				location.0 += go;
				North
			}
			West => {
				location.0 -= go;
				South
			}
		};

		if !first {
			leg += 1;
		}
		first = !first;
		rem -= go;
		rem
	};
	while &remaining != &0isize {
		remaining = _move(remaining);
	}
	println!("{:?}", location);
}
