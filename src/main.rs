//use sfml::graphics::{RenderTexture};
extern crate rand;
extern crate dwarfilib;
use std::cmp::Ordering::Equal;
use dwarfilib::mapping::shapes::Rect;
use dwarfilib::genetics::genes::Gene;
use dwarfilib::genetics::breeding;

fn main() { //TODO: Move all this to actual functions
	let mut genes: Vec<Gene> = Vec::new();
	for i in 1..15 {
		let rect = Rect{ x: 0, y: 0, w: (i*13)%7, h: (i*5)%7 };
		genes.push(Gene::new(rect, i));
	}
	let mut rng = rand::thread_rng();
	let mut population = breeding::generate_initial_population(
		genes, 200, &mut rng);
	for i in 0..1000 {
		if i % 100 == 0 {
			population.sort_by(|a, b| a.partial_cmp(b).unwrap_or(Equal));
			population.reverse();
			println!("{:?}: {:?}", i, population[0]);
		}
		population = breeding::breed(&population, &mut rng);
	}
}