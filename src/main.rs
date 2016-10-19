//use sfml::graphics::{RenderTexture};
extern crate rand;
extern crate dwarfilib;
use std::cmp::Ordering::Equal;
use std::io;
use dwarfilib::mapping::shapes::Rect;
use dwarfilib::genetics::genes::Gene;
use dwarfilib::genetics::breeding;
use dwarfilib::collections::Vector;
use dwarfilib::io::{output, input};

/// A makeshift main-function, used to test functionality.
fn main() { //TODO: Move all this to actual functions
	let bp = input::read("input.json".to_string());
	let (genes, targets) = bp.compile();
	let mut rng = rand::thread_rng();
	let mut pop_size = String::new();
	println!("Population size: ");
	io::stdin().read_line(&mut pop_size).expect("Failed read )-:");
	let pop_size: usize = pop_size.trim().parse().expect("Not a number )-:");
	let mut generations = String::new();
	println!("Generations: ");
	io::stdin().read_line(&mut generations).expect("Failed read )-:");
	let generations: usize = generations.trim().parse().expect("Not a number )-:");
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