use mapping::shapes::Rect;
use genetics::genes::Gene;
use genetics::breeding;
use collections::Vector;
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
	let population = breeding::generate_initial_population(
		genes, Vector::new(), 500, &mut rng);
	b.iter(|| {
		breeding::breed(population.clone(), &mut rng); //TODO: Find out how to do this without cloning
	});
}