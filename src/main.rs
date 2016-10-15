//use sfml::graphics::{RenderTexture};
extern crate rand;
extern crate dwarfilib;
use std::cmp::Ordering::Equal;
use dwarfilib::mapping::shapes::Rect;
use dwarfilib::genetics::genes::Gene;
use dwarfilib::genetics::breeding;
use dwarfilib::io::output;

/// A makeshift main-function, used to test functionality.
fn main() { //TODO: Move all this to actual functions
	let mut genes: Vec<Gene> = Vec::new();
	for i in 1..17 {
		let rect = Rect{ x: 0, y: 0, w: (i*13)%7 + 4, h: (i*5)%7 + 4};
		genes.push(Gene::new(rect, i - 1));
	}
	let mut rng = rand::thread_rng();
	let mut population = breeding::generate_initial_population(
		genes, 500, &mut rng);
	let mut last_fitness: f32 = 0.0;
	let mut counter = 0;
	for i in 0..3001 {
		population = breeding::breed(population, &mut rng);
		if i % 200 == 0 {
			population.sort_by(|a, b| a.partial_cmp(b).unwrap_or(Equal));
			population.reverse();
			println!("{:?}: {:?}", i, population[0]);
			if population[0].fitness > last_fitness {
				last_fitness = population[0].fitness;
				counter = 0;
			} else {
				println!("PURGE!");
				counter += 1;
				if counter == 4 {
					break;
				}
				breeding::purge(&mut population, &mut rng);
			}
		}
	}
	population.sort_by(|a, b| a.partial_cmp(b).unwrap_or(Equal));
	population.reverse();
	println!("{:?}: {:?}", population[0], population[0].genes[0]);
	let layout = population[0].as_layout();
	let matrix = layout.as_char_matrix();
	match output::save(matrix) {
		Err(reason) => panic!("{:?}", reason),
		Ok(_) => {}
	};
}