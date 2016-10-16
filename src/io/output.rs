use std::io::{Write, Error};
use std::fs::File;
use collections::Matrix;

/// Saves a character matrix to a file
pub fn save(matrix: Matrix<char>, filename: String) -> Result<(), Error>{
	let mut output = String::new();
	for y in 0..matrix.h {
		for x in 0..matrix.w {
			output.push(match *matrix.get(x, y) {
					Some(c) => c,
					None => ' '
			}
			);
		}
		output.push('\r'); // DOS-compatible line change
		output.push('\n');
	}
	let mut file = try!(File::create(filename)); //TODO: Filenames
	try!(file.write_all(output.as_bytes()));
	Ok(())
}

#[cfg(test)]
mod tests {
	
	use super::*;
	use std::char;
	use collections::Matrix;
	
	#[test]
	fn save_works() {
		let w = 3;
		let h = 4;
		let mut mat = Matrix::new(w, h);
		for y in 0..h {
			for x in 0..w {
				let char = char::from_u32((97 + y*w + x) as u32).unwrap();
				mat.set(x, y, char);
			}
		}
		match save(mat, String::from("test.txt")) {
			Ok(_) => {},
			Err(reason) => panic!("{:?}", reason)
		}
	}
}