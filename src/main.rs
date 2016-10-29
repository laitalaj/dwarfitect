extern crate rand;
extern crate dwarfilib;
use dwarfilib::genetics::breeding;
use dwarfilib::io::{output, input};
use dwarfilib::io::ui::{get_input_loop, get_parsed_input_loop};
use std::{thread, time};

/// A main-function that brings everything together under a text UI.
/// # Panics
/// Panics if unable to read or save for some reason
fn main() { //TODO: Move all this to actual functions
	println!("############\n#DWARFITECT#\n############\n");
	let mut input_file = String::new();
	let mut valid_file = false;
	while !valid_file {
		println!("Input file name: ");
		input_file = get_input_loop();
		valid_file = input::exists(&String::from(input_file.trim()));
		if !valid_file {
			println!("\"{}\" doesn't exist in the dwarfitect binary folder!",
			input_file.trim());
		}
	}
	println!("Output file name: ");
	let output_file = get_input_loop();
	println!("Population size: ");
	let pop_size: usize = get_parsed_input_loop();
	println!("Generations: ");
	let generations: usize = get_parsed_input_loop();
	println!("");
	let mut rng = rand::thread_rng();
	let bp = input::read(String::from(input_file.trim()));
	let (genes, targets) = bp.compile();
	let result = breeding::breeder(genes, targets, pop_size, generations, &mut rng);
	let layout = result.as_layout();
	let matrix = layout.as_char_matrix();
	match output::save_matrix(matrix, String::from(output_file.trim())) {
		Err(reason) => println!("\nERROR! Couldn't save the result! ({:?})", reason),
		Ok(_) => println!("\nSuccesfully saved result to {}", output_file.trim())
	};
	let sleep_time = time::Duration::new(5, 0);
	thread::sleep(sleep_time);
}