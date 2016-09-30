//use sfml::graphics::{RenderTexture};
extern crate rand;
extern crate dwarfilib;
use std::cmp::Ordering::Equal;
use dwarfilib::mapping::shapes::Rect;
use dwarfilib::genetics::genes::Gene;
use dwarfilib::genetics::breeding;

fn main() { //TODO: Move all this to actual functions
	let mut genes: Vec<Gene> = Vec::new();
	for i in 1..17 {
		let rect = Rect{ x: 0, y: 0, w: (i*13)%7 + 1, h: (i*5)%7 + 1};
		genes.push(Gene::new(rect, i));
	}
	let mut rng = rand::thread_rng();
	let mut population = breeding::generate_initial_population(
		genes, 300, &mut rng);
	let mut last_fitness: f32 = 0.0;
	for i in 0..5001 {
		population = breeding::breed(population, &mut rng);
		if i % 500 == 0 {
			population.sort_by(|a, b| a.partial_cmp(b).unwrap_or(Equal));
			population.reverse();
			println!("{:?}: {:?}", i, population[0]);
			if population[0].fitness > last_fitness {
				last_fitness = population[0].fitness;
			} else {
				println!("PURGE!");
				breeding::purge(&mut population, &mut rng);
			}
		}
	}
}