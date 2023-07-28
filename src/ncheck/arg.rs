pub fn parse(v: Vec<String>) -> Result<(String, String), ()> {
	if v.len() != 3 {
		Err(())
	} else {
		Ok((v[1].clone(), v[2].clone()))
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	
	#[test]
	fn arg_parse_test() {
		let v = vec![
			String::from("zero"),
			String::from("one"),
		];
		
		assert_eq!(parse(v), Err(()));
		
		let v = vec![
			String::from("zero"),
			String::from("one"),
			String::from("two"),
		];
		
		assert_eq!(parse(v), Ok(("one".to_string(), "two".to_string())));
	}
}