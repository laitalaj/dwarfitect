//use sfml::graphics::{RenderTexture};
extern crate rand;
extern crate dwarfilib;
use dwarfilib::genetics::breeding;
use dwarfilib::io::{output, input};
use dwarfilib::io::ui::get_usize;

/// A makeshift main-function, used to test functionality.
fn main() { //TODO: Move all this to actual functions
	let bp = input::read("input.json".to_string());
	let (genes, targets) = bp.compile();
	let mut rng = rand::thread_rng();
	println!("Population size: ");
	let pop_size = get_usize().expect("Failed )-:");
	println!("Generations: ");
	let generations = get_usize().expect("Failed )-:");
	let result = breeding::breeder(genes, targets, pop_size, generations, &mut rng);
	println!("{:?}: {:?}", result, result.genes[0]);
	println!("{:?}", result.genes[0].center());
	let layout = result.as_layout();
	let matrix = layout.as_char_matrix();
	match output::save_matrix(matrix, String::from("final.txt")) {
		Err(reason) => panic!("{:?}", reason),
		Ok(_) => {}
	};
}