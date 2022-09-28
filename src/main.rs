use std::convert::From;

#[derive(Debug, Eq, PartialEq)]
enum Error {
	Split,
	Parse(String),
	DivisionByZero,
}

#[derive(Debug, Eq, PartialEq)]
struct DivisionByZeroError;
#[derive(Debug, Eq, PartialEq)]
struct SplitError;
#[derive(Debug, Eq, PartialEq)]
struct ParseError(String);

impl From<ParseError> for Error {
	fn from(e: ParseError) -> Self {
		Self::Parse(e.0)
	}
}

impl From<SplitError> for Error {
	fn from(_: SplitError) -> Self {
		Self::Split
	}
}

impl From<DivisionByZeroError> for Error {
	fn from(_: DivisionByZeroError) -> Self {
		Self::DivisionByZero
	}
}

fn split(s: String) -> Result<(String, String), SplitError> {
	let mut ss = s
		.split(',')
		.take(3)
		.map(String::from)
		.collect::<Vec<String>>();

	match (ss.pop(), ss.pop(), ss.pop()) {
		(Some(a), Some(b), None) => Ok((b, a)),
		_ => Err(SplitError),
	}
}

fn parse((a, b): (String, String)) -> Result<(i32, i32), ParseError> {
	match (a.parse::<i32>(), b.parse::<i32>()) {
		(Ok(x), Ok(y)) => Ok((x, y)),
		(Err(_), _) => Err(ParseError(a)),
		(_, Err(_)) => Err(ParseError(b)),
	}
}

fn safe_div((a, b): (i32, i32)) -> Result<i32, DivisionByZeroError> {
	if b == 0 {
		return Err(DivisionByZeroError);
	}

	Ok(a / b)
}

fn perform(input: String) -> Result<i32, Error> {
	Ok(safe_div(parse(split(input)?)?)?)
}

fn main() {
	let input = match std::env::args().nth(1) {
		Some(x) => x,
		None => {
			eprintln!("No input was given :<");
			std::process::exit(-1);
		}
	};

	match safe_div((1, 1)) {
		Ok(value) => println!("Result of safe division: {value}"),
		Err(_) => eprintln!("Can't divide by zero. :<"),
	}

	println!("Input: {input}");

	match perform(input) {
		Ok(value) => println!("Output: {value}"),
		Err(e) => match e {
			Error::Split => eprintln!("Couldn't split. :<"),
			Error::Parse(s) => eprintln!("'{s}' is not a number. :o"),
			Error::DivisionByZero => eprintln!("Can't divide by zero. :<"),
		},
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn div_by_zero() {
		assert_eq!(safe_div((1, 0)), Err(DivisionByZeroError));
	}

	#[test]
	fn div_by_non_zero() {
		assert_eq!(safe_div((1, 1)), Ok(1));
	}

	#[test]
	fn splitter() {
		let s = String::from("52,3");
		let fifty_two = String::from("52");
		let three = String::from("3");
		assert_eq!(split(s), Ok((fifty_two, three)));
	}

	#[test]
	fn parser() {
		let one = String::from("1");
		assert_eq!(parse((one.clone(), one)), Ok((1, 1)));
	}

	#[test]
	fn performer() {
		assert_eq!(perform("5,1".to_owned()), Ok(5));
	}
}
