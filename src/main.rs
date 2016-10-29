//use sfml::graphics::{RenderTexture};
extern crate rand;
extern crate dwarfilib;
use dwarfilib::genetics::breeding;
use dwarfilib::io::{output, input};
use dwarfilib::io::ui::{get_input_loop, get_parsed_input_loop};

/// A main-function that brings everything together under a text UI.
/// # Panics
/// Panics if unable to read or save for some reason
fn main() { //TODO: Move all this to actual functions
	println!("Input file name: ");
	let input_file = get_input_loop();
	println!("Output file name: ");
	let output_file = get_input_loop();
	println!("Population size: ");
	let pop_size: usize = get_parsed_input_loop();
	println!("Generations: ");
	let generations: usize = get_parsed_input_loop();
	let mut rng = rand::thread_rng();
	let bp = input::read(String::from(input_file.trim()));
	let (genes, targets) = bp.compile();
	let result = breeding::breeder(genes, targets, pop_size, generations, &mut rng);
	let layout = result.as_layout();
	let matrix = layout.as_char_matrix();
	match output::save_matrix(matrix, String::from(output_file.trim())) {
		Err(reason) => panic!("{:?}", reason),
		Ok(_) => {}
	};
}