mod data_parse;
mod compare;

pub struct Data {
	sch: Vec<Entry>,
	dwg: Vec<Entry>,
}

pub struct Entry {
	net: String,
	nodes: Vec<String>,
}

impl Entry {
	fn new() -> Entry {
		Entry {
			net: String::new(),
			nodes: Vec::new(),
		}
	}
}

impl Data {
	pub fn new() -> Data {
		Data {
			sch: Vec::new(),
			dwg: Vec::new(),
		}
	}
	
	pub fn run(&mut self, sch_string: &str, dwg_string: &str) -> String {
		self.sch = data_parse::parse(&sch_string, "*NET*", "*SIGNAL*", "*END*");
		self.dwg = data_parse::parse(&dwg_string, "NET LIST", "*SIG", "");
		
		Self::sort(&mut self.sch);
		Self::sort(&mut self.dwg);
		
		compare::compare(self)
	}
	
	fn sort(dlist: &mut Vec<Entry>) {
		dlist.sort_by_key(|n| n.net.clone());
		
		for item in dlist {
			item.nodes.sort();
		}
	}
}

#[cfg(test)]
mod tests {
	use super::*;
	
	#[test]
	fn data_run_test() {
		let test_string_sch = "\
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

		let test_string_dwg = "
*NET LIST ...*

*SIG GND
D11.2                   F1.3                    EE2.2
A1.2                    C14.2                   B2.8
*SIG +5V
A1.1                    CR1.2                   B1.40
";

		let result = "\
Unmatched SCHEMATIC nodes:
* net : GND
* node: EE2.1 

Unmatched DRAWING nodes:
* net : GND
* node: EE2.2 
";

		let mut data = Data::new();
		
		assert_eq!(
			data.run(&test_string_sch, &test_string_dwg),
			result
		);	
	}
}