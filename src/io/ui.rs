use std::io::{self, Error, ErrorKind};
use std::str::FromStr;

/// Reads input from stdin and returns it wrapped in a result.
pub fn get_input() -> Result<String, Error> {
	let mut result = String::new();
	try!(io::stdin().read_line(&mut result));
	Ok(result)
}

/// Reads input and parses it to any type that satisfies the trait FromStr.
/// The type can be inferred beautifully!
pub fn get_parsed_input<T: FromStr>() -> Result<T, Error> {
	let string = match get_input() {
		Ok(s) => s,
		Err(reason) => return Err(reason)
	};
	match string.trim().parse() {
		Ok(result) => Ok(result),
		Err(_) => Err(Error::new(ErrorKind::InvalidInput, 
				"Couldn't parse input to given type!"))
	}
}