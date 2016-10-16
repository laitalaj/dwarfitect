use std::cmp::Ordering::Equal;
use std::cell::RefCell;
use mapping::shapes::Rect;
use genetics::genes::Gene;
use genetics::breeding;
use collections::Vector;
use io::output;
use rand;

extern crate test;
use tests::benchmarks::test::Bencher;

#[bench]
fn breeding_benchmark(b: &mut Bencher) {
	let mut genes: Vector<Gene> = Vector::new();
	for i in 1..17 {
		let rect = Rect{ x: 0, y: 0, w: (i*13)%7 + 4, h: (i*5)%7 + 4};
		genes.push(Gene::new(rect, i - 1));
	}
	let mut rng = rand::thread_rng();
	let mut population = breeding::generate_initial_population(
		genes, 500, &mut rng);
	b.iter(|| {
		breeding::breed(population.clone(), &mut rng);
	});
}