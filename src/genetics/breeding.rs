use rand::Rng;
use super::genes::{Gene, Chromosome};

pub const KEEP_ALIVE_PERCENTAGE: f32 = 0.1;

struct Candidate {
	prob_range_end: f32,
	chromosome: &Chromosome
}

impl PartialOrd for Candidate {
	fn partial_cmp(&self, other: &Candidate) -> Option<Ordering> {
		Some(self.cmp(other))
	}
}

impl Ord for Candidate {
	fn cmp(&self, other: &Candidate) -> Ordering {
		self.prob_range_end.cmp(&other.prob_range_end)
	}
}

fn generate_initial_population<R: Rng>(genes: Vec<Gene>, size: u16, rng: R){
	let mut population: Vec<Chromosome> = Vec::new();
	for _ in 0..size {
		population.push(Chromosome::create_initial(genes, rng));
	}
	population
}

fn breed<R: Rng>(population: &Vec<Chromosome>, rng: R) -> Vec<Chromosome>{
	let mut total_fitness = 0.0;
	for c in population {
		total_fitness += c.fitness;
	}
	let mut candidates
}