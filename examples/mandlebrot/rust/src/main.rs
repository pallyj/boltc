fn main() {
    let size = 1600;

	let level = 1;

	let mut i = -(size / 2);

    let s = (size / 2) as f32;

	while i < (size / 2)
	{
		let mut j = -(size / 2);

		while j < (size / 2) {
			let fj = i as f32; // error inits dont have a type
			let fi = j as f32;
			let x = fj / s;
			let y = fi / s;

			let dist = get_distance(x, y);

			if dist > level {
				print!("**")
			} else {
				print!("  ")
			}

			j += 1;
		}

		println!();

		i += 1;
	}
}


const MAXIMUM: f32 = 16.0;
const MAX_ITERS: i32 = 1000;

fn get_distance(x: f32, y: f32) -> i32 {
	let (ci, cr) = (x, y - 0.5);
	let (mut zi, mut zr) = (0.0, 0.0);
	let mut i: i32 = 0;

	loop {
		i += 1;
		let temp = zr * zi;
		let zr2 = zr * zr;
		let zi2 = zi * zi;
		zr = zr2 - zi2 + cr;
		zi = temp + temp + ci;
		if zi2 + zr2 > MAXIMUM {
			return i
		}
		if i > MAX_ITERS {
			return 0
		}
	}
}