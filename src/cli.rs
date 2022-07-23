use hashdb::LinkArena;
use name::NamespaceMut;
use parse::parse;

mod expr;
mod name;
mod parse;

pub fn print_usage() {
	println!("USAGE: tmp-lang <filename>")
}

pub fn read_from_file(filename: &str) -> Result<String, String> {
	std::fs::read_to_string(filename).map_err(|_| "could not open file".into())
}

pub fn run_cli_args_file() -> Result<(), String> {
	let mut input_files: Vec<String> = vec![];
	for arg in std::env::args() {
		match arg.as_str() {
			"-i" => {
				cli_editor();
				return Ok(());
			}
			_ => input_files.push(arg),
		};
	}
	if input_files.is_empty() {
		return Err("no input files".into());
	} else if input_files.len() > 1 {
		return Err("multiple input files".into());
	}
	let file_content =
		read_from_file(input_files[0].as_str()).map_err(|_| "could not read file".to_owned())?;
	let exprs = &LinkArena::new();
	let namespace = &mut NamespaceMut::new();
	let _parsed = parse(file_content.as_str(), namespace, exprs).unwrap();
	Ok(())
}

fn cli_editor() {
	use rustyline::Editor;
	println!("tmp-lang cli editor!");
	let mut editor = Editor::<()>::new().unwrap();
	let readline = editor.readline("=> ");
	match readline {
		Ok(line) => println!("line: '{:?}'", line),
		Err(_) => println!("no input"),
	}
}

fn main() {
	cli_editor();
}
