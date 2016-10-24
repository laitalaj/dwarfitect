use std::io::{self, Error, ErrorKind};

pub fn get_input() -> Result<String, Error> {
	let mut result = String::new();
	try!(io::stdin().read_line(&mut result));
	Ok(result)
}

pub fn get_usize() -> Result<usize, Error> {
	let string = match get_input() {
		Ok(s) => s,
		Err(reason) => return Err(reason)
	};
	match string.trim().parse() {
		Ok(i) => Ok(i),
		Err(_) => Err(Error::new(ErrorKind::InvalidInput, 
				"Couldn't parse input to a number!"))
	}
}