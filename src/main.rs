//use sfml::graphics::{RenderTexture};
extern crate rand;
extern crate dwarfilib;
use std::cmp::Ordering::Equal;
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
	let mut population = breeding::generate_initial_population(
		genes, targets, 500, &mut rng
	);
//	let mut genes: Vector<Gene> = Vector::new();
//	for i in 1..17 {
//		let rect = Rect{ x: 0, y: 0, w: (i*13)%7 + 4, h: (i*5)%7 + 4};
//		genes.push(Gene::new(rect, i - 1));
//	}
//	let mut rng = rand::thread_rng();
//	let mut population = breeding::generate_initial_population(
//		genes, Vector::new(), 500, &mut rng);
	population.sort_by(|a, b| a.partial_cmp(b).unwrap_or(Equal));
	population.reverse();
	println!("{:?}: {:?}", population[0], population[0].genes[0]);
	println!("{:?}", population[0].genes[0].center());
	let layout = population[0].as_layout();
	let matrix = layout.as_char_matrix();
	match output::save_matrix(matrix, String::from("initial.txt")) {
		Err(reason) => panic!("{:?}", reason),
		Ok(_) => {}
	};
	population = breeding::breed_for(population, 2000, &mut rng);
	population.sort_by(|a, b| a.partial_cmp(b).unwrap_or(Equal));
	population.reverse();
	println!("{:?}: {:?}", population[0], population[0].genes[0]);
	println!("{:?}", population[0].genes[0].center());
	let layout = population[0].as_layout();
	let matrix = layout.as_char_matrix();
	match output::save_matrix(matrix, String::from("final.txt")) {
		Err(reason) => panic!("{:?}", reason),
		Ok(_) => {}
	};
}