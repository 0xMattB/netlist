use super::Entry;

pub fn parse(source: &str, target: &str, breakup: &str, ignore: &str) -> Vec<Entry> {
	let mut s = String::from(source);
	
	remove_to(&mut s, target);
	let chunks = break_chunks(&s, breakup);
	
	chunk_parse(&chunks, ignore)
}

fn remove_to(s: &mut String, target: &str) {
	if let Some(n) = s.find(target) {
		s.replace_range(..n, "");
		*s = s.trim().to_string();
	}
	
	if let Some(n) = s.find('\n') {
		s.replace_range(..n, "");
		*s = s.trim().to_string();
	}
}

fn break_chunks(s: &str, pattern: &str) -> Vec<String> {
	let s = s.split(pattern);
	let mut chunks: Vec<String> = Vec::new();
	
	for chunk in s {
		if chunk.len() > 0 {
			chunks.push(chunk.trim().to_string());
		}
	}
	
	chunks
}

fn chunk_parse(chunks: &Vec<String>, ignore: &str) -> Vec<Entry> {					
	let mut entry_list: Vec<Entry> = Vec::new();				
	let mut entry = Entry::new();				
					
	for chunk in chunks {				
		let mut read_net = true;			
					
		for line in chunk.lines() {			
			if read_net {		
				entry.net = line.to_string();	
				read_net = false;	
			} else if line != ignore {		
				for c in line.split_whitespace() {	
					entry.nodes.push(c.to_string());
				}	
			}		
		}			
					
		entry_list.push(entry);			
		entry = Entry::new();			
	}				
					
	entry_list				
}					

#[cfg(test)]
mod tests {
	use super::*;
	
	#[test]
	fn data_parse_sch_test() {
		let test_string = "\
*PADS-PCB*
*PART*
C1     CC0004

*NET*
*SIGNAL* +5V
A1.1 CR1.2 B1.40
*SIGNAL* GND
D11.2 F1.3 EE2.1
A1.2 C14.2 B2.8
*END*";
	
		let parsed = parse(test_string, "*NET*", "*SIGNAL*", "*END*");
		
		assert!(
			if parsed[0].net == "+5V".to_string() &&
				parsed[0].nodes[0] == "A1.1".to_string() &&
				parsed[0].nodes[1] == "CR1.2".to_string() &&
				parsed[0].nodes[2] == "B1.40".to_string() &&
				
				parsed[1].net == "GND".to_string() &&
				parsed[1].nodes[0] == "D11.2".to_string() &&
				parsed[1].nodes[1] == "F1.3".to_string() &&
				parsed[1].nodes[2] == "EE2.1".to_string() &&
				parsed[1].nodes[3] == "A1.2".to_string() &&
				parsed[1].nodes[4] == "C14.2".to_string() &&
				parsed[1].nodes[5] == "B2.8".to_string() {
					true
				} else {
					false
				}
		);
	}
	
	#[test]
	fn data_parse_dwg_test() {
		let test_string = "
*NET LIST ...*

*SIG GND
D11.2                   F1.3                    EE2.1
A1.2                    C14.2                   B2.8
*SIG +5V
A1.1                    CR1.2                   B1.40
";
	
		let parsed = parse(test_string, "NET LIST", "*SIG", "");
		
		assert!(
			if parsed[0].net == "GND".to_string() &&
				parsed[0].nodes[0] == "D11.2".to_string() &&
				parsed[0].nodes[1] == "F1.3".to_string() &&
				parsed[0].nodes[2] == "EE2.1".to_string() &&
				parsed[0].nodes[3] == "A1.2".to_string() &&
				parsed[0].nodes[4] == "C14.2".to_string() &&
				parsed[0].nodes[5] == "B2.8".to_string() &&
				
				parsed[1].net == "+5V".to_string() &&
				parsed[1].nodes[0] == "A1.1".to_string() &&
				parsed[1].nodes[1] == "CR1.2".to_string() &&
				parsed[1].nodes[2] == "B1.40".to_string() {
					true
				} else {
					false
				}
		);
	}
}