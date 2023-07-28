mod ncheck;

use ncheck::arg;
use ncheck::file;
use ncheck::data::Data;
use std::{env, process};

pub fn run() {
	let (fname_sch, fname_dwg) = arg::parse(env::args().collect()).unwrap_or_else(|_| {
		eprintln!("usage: netlist.exe (schematic-netlist-filename) (drawing-netlist-filename)");
		process::exit(1);
	});
		
	let fread_sch = file::read(&fname_sch).unwrap_or_else(|err| {
		eprintln!("error opening {fname_sch}: {err}");
		process::exit(1);
	});
	
	let fread_dwg = file::read(&fname_dwg).unwrap_or_else(|err| {
		eprintln!("error opening {fname_dwg}: {err}");
		process::exit(1);
	});
	
	let mut data = Data::new();
	
	println!("\n{}", data.run(&fread_sch, &fread_dwg));
}
