use std::io::{Write, Error};
use std::fs::File;
use collections::Matrix;

/// Saves a character matrix to a file
pub fn save(matrix: Matrix<char>) -> Result<(), Error>{
	let mut output = String::new();
	for y in 0..matrix.h {
		for x in 0..matrix.w {
			print!("{:?}", matrix.get(x, y));
			output.push(matrix.get(x, y).unwrap());
		}
		output.push('\r'); // DOS-compatible line change
		output.push('\n');
	}
	let mut file = try!(File::create("out.txt")); //TODO: Filenames
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
				print!("{}", char);
				mat.set(x, y, char);
			}
		}
		match save(mat) {
			Ok(_) => {},
			Err(reason) => panic!("{:?}", reason)
		}
	}
}