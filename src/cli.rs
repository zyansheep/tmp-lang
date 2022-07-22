use crate::{name::NamespaceMut, parse::parse};
use hashdb::LinkArena;

pub fn print_usage() {
	println!("asdsassadasd")
}

pub fn read_from_file(filename: &str) -> Result<String, String> {
	std::fs::read_to_string(filename).map_err(|_| "could not open file".into())
}

pub fn run_cli_args_file() -> Result<(), String> {
	let mut input_files: Vec<String> = vec![];
	for arg in std::env::args() {
		match arg {
			_ => input_files.push(arg),
		};
	}
	if input_files.len() == 0 {
		return Err("no input files".into());
	} else if (input_files.len() > 1) {
		return Err("multiple input files".into());
	}
	let file_content = read_from_file(input_files[0].as_str());
	let exprs = &LinkArena::new();
	let namespace = &mut NamespaceMut::new();
	let parsed = parse("[x y] x y", namespace, exprs).unwrap();
	Ok(())
}
