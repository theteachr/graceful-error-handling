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

fn main() -> Result<(), &'static str> {
	let input = std::env::args().nth(1).ok_or("No input given :<")?;

	println!("Input: {input}");

	match perform(input) {
		Ok(value) => println!("Output: {value}"),
        Err(Error::Split) => eprintln!("Couldn't split. :<"),
		Err(Error::Parse(s)) => eprintln!("'{s}' is not a number. :o"),
		Err(Error::DivisionByZero) => eprintln!("Can't divide by zero. :<"),
	}

    Ok(())
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
	fn split_invalid_empty() {
		let s = String::from("");
		assert_eq!(split(s), Err(SplitError));
	}

	#[test]
	fn split_invalid_excess_numbers() {
		let s = String::from("523,1,2");
		assert_eq!(split(s), Err(SplitError));
	}

	#[test]
	fn split_invalid_single_number() {
		let s = String::from("523");
		assert_eq!(split(s), Err(SplitError));
	}

	#[test]
	fn split_valid() {
		let s = String::from("52,3");
		let fifty_two = String::from("52");
		let three = String::from("3");
		assert_eq!(split(s), Ok((fifty_two, three)));
	}

	#[test]
	fn parse_b() {
		let ten_thousand = String::from("1");
		let four = String::from("b");
		assert_eq!(parse((ten_thousand, four)), Err(ParseError("b".to_owned())));
	}

	#[test]
	fn parse_a() {
		let ten_thousand = String::from("a");
		let four = String::from("4");
		assert_eq!(parse((ten_thousand, four)), Err(ParseError("a".to_owned())));
	}

	#[test]
	fn parse_ten_thousand_and_four() {
		let ten_thousand = String::from("10000");
		let four = String::from("4");
		assert_eq!(parse((ten_thousand, four)), Ok((10000, 4)));
	}

	#[test]
	fn parse_one() {
		let one = String::from("1");
		assert_eq!(parse((one.clone(), one)), Ok((1, 1)));
	}

	#[test]
	fn perform_works() {
		assert_eq!(perform("5,1".to_owned()), Ok(5));
	}
}
