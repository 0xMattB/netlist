use std::{fs, error::Error};

pub fn read(filename: &str) -> Result<String, Box<dyn Error>> {
	let contents = fs::read_to_string(filename)?;
	
	Ok(contents)
}

#[cfg(test)]
mod tests {
	use super::*;
	
	#[test]
	fn file_read_test() {
		assert_eq!(read("test.txt").unwrap(), "test file.".to_string());
	}
}