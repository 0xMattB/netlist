use super::Data;
use super::Entry;

pub fn compare(data: &mut Data) -> String {
	compare_direct(data);
	compare_indirect(data);
	
	generate_response(data)
}

fn compare_direct(data: &mut Data ) {
	let mut isch = 0;
	
	while isch < data.sch.len() {
		if let Ok(idwg) = data.dwg.binary_search_by_key(&data.sch[isch].net, |d| d.net.clone() ) {
			vector_remove_matches(&mut data.sch[isch].nodes, &mut data.dwg[idwg].nodes);
			
			if data.sch[isch].nodes.len() == 0 && data.dwg[idwg].nodes.len() == 0 {
				data.sch.remove(isch);
				data.dwg.remove(idwg);
				continue;
			}
		}
		
		isch += 1;
	}
}

fn compare_indirect(data: &mut Data ) {
	let mut isch = 0;
	
	while isch < data.sch.len() {
		for idwg in 0..data.dwg.len() {
			if vector_compare(&data.sch[isch].nodes, &data.dwg[idwg].nodes) {
				data.sch.remove(isch);
				data.dwg.remove(idwg);
				continue;
			}
		}
		
		isch += 1;
	}
}

fn generate_response(data: &Data) -> String {
	let mut response = String::new();
	
	if data.sch.len() == 0 && data.dwg.len() == 0 {
		response.push_str("The two files match.");
	} else {
		response.push_str("Unmatched SCHEMATIC nodes:\n");
		add_data_to_response(&mut response, &data.sch);
		
		response.push_str("\nUnmatched DRAWING nodes:\n");
		add_data_to_response(&mut response, &data.dwg);
	}
	
	response
}

fn add_data_to_response(response: &mut String, d: &Vec<Entry>) {
	for chunk in d {
		if chunk.nodes.len() == 0 {
			continue;
		}
		
		response.push_str(&format!["* net : {}\n* node: ", chunk.net]);
		
		for n in &chunk.nodes {
			response.push_str(&format!["{} ", n]);
		}
		
		response.push_str("\n");
	}
}

fn vector_compare(a: &Vec<String>, b: &Vec<String>) -> bool {
	if a.len() != b.len() {
		return false;
	}
	
	for i in 0..a.len() {
		if a[i] != b[i] {
			return false;
		}
	}
	
	true
}

fn vector_remove_matches(v1: &mut Vec<String>, v2: &mut Vec<String>) {
	let mut i = 0;

	while i < v1.len() {
		let mut j = 0;
		let mut advance_i = true;

		while j < v2.len() {
			if v1[i] == v2[j] {
				v1.remove(i);
				v2.remove(j);
				advance_i = false;
				break;
			}

			j += 1;
		}
		
		if advance_i {
			i += 1;
		}
	}
}