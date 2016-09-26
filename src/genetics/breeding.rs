use rand::Rng;
use std::cmp::Ordering;
use std::cmp::Ordering::Equal;
use super::genes::{Gene, Chromosome};

/// Percentage of population that should be kept alive for the next round of
/// breeding
pub const KEEP_ALIVE_PERCENTAGE: f32 = 0.1;

/// Candidate is a container for a chromosome with a determined probability
/// of selection for breeding
#[derive(PartialEq, Debug)]
struct Candidate<'a> {
  prob_range_end: f32,
  pub chromosome: &'a Chromosome
}

impl<'a> Candidate<'a> {
	/// Constructor for candidate
  fn new(prob_range_end: f32, chromosome: &'a Chromosome) -> Candidate {
    Candidate{ prob_range_end: prob_range_end, chromosome: chromosome }
  }
}

impl<'a> PartialOrd for Candidate<'a> {
	/// Ordering for candidate, based on it's probability range end
  fn partial_cmp(&self, other: &Candidate) -> Option<Ordering> {
    self.prob_range_end.partial_cmp(&other.prob_range_end)
  }
}

/// Binary searches the candidate with the smallest probability range end that's
/// larger than random_value
fn search_candidate<'a>(candidates: &'a Vec<Candidate>, random_value: f32) 
-> Option<&'a Candidate<'a>> {
	let mut smallest_match: Option<&Candidate> = None;
	let mut min = 0;
	let mut max = candidates.len() - 1;
	while min + 1 < max {
		let candidate = &candidates[(min + max) / 2];
		if candidate.prob_range_end < random_value {
			min = (min + max) / 2;
		} else {
			smallest_match = Some(candidate);
			max = (min + max) / 2;
		}
	}
	smallest_match
}

/// Generates an initial population with determined size
pub fn generate_initial_population<R: Rng>(genes: Vec<Gene>, size: u16, 
	rng: &mut R) -> Vec<Chromosome> {
  let mut population: Vec<Chromosome> = Vec::new();
  for _ in 0..size {
    population.push(Chromosome::generate_initial(genes.to_vec(), rng));
  }
  population
}

/// Breeds a population by 1 step; generates and mutates children and returns
/// the next population.
pub fn breed<R: Rng>(population: &Vec<Chromosome>, rng: &mut R) -> Vec<Chromosome>{
  let mut total_fitness = 0.0;
  let mut work_population = population.to_vec();
  work_population.sort_by(|a, b| a.partial_cmp(b).unwrap_or(Equal));
  work_population.reverse();
  for i in 0..work_population.len() {
    total_fitness += work_population[i].fitness;
  }
  let mut candidates: Vec<Candidate> = Vec::new();
  let mut current_prob_range_end = 0.0;
  for i in 0..work_population.len() {
    current_prob_range_end += work_population[i].fitness / total_fitness;
    candidates.push(Candidate::new(current_prob_range_end, &work_population[i]));
  }
  let mut next_population: Vec<Chromosome> = Vec::new(); //TODO: Keeping the best of previous population
  while next_population.len() < population.len() {
  	let candidate1 = search_candidate(&candidates, rng.next_f32()).unwrap(); //TODO: Handling None
  	let candidate2 = search_candidate(&candidates, rng.next_f32()).unwrap(); //TODO: Avoiding duplicates
  	let (mut child1, mut child2) = candidate1.chromosome
  	.mate(candidate2.chromosome, rng);
  	child1.mutate(rng);
  	child2.mutate(rng);
  	next_population.push(child1);
  	next_population.push(child2);
  }
  next_population
}

#[cfg(test)]
mod tests {

    use super::*;
    use mapping::shapes::Rect;
    use genetics::genes::Gene;
    use rand;
    
    #[test]
    fn everything_doesnt_break() { //TODO: Actual tests
	    let rect1 = Rect {
            x: 2,
            y: 3,
            w: 5,
            h: 7,
        };
        let gene1 = Gene::new(rect1, 0);
        let rect2 = Rect {
            x: 1,
            y: 0,
            w: 3,
            h: 3,
        };
        let gene2 = Gene::new(rect2, 1);
        let rect3 = Rect {
            x: -2,
            y: 5,
            w: 5,
            h: 10,
        };
        let gene3 = Gene::new(rect3, 2);
        let rect4 = Rect {
            x: 0,
            y: -7,
            w: 12,
            h: 8,
        };
        let gene4 = Gene::new(rect4, 3);
	    let mut rng = rand::thread_rng();
    	let initial_pop = 
    	generate_initial_population(vec![gene1, gene2, gene3, gene4], 100, 
    		&mut rng);
    	assert_eq!(100, initial_pop.len());
    	let next_pop = breed(&initial_pop, &mut rng);
    	assert_eq!(100, next_pop.len());
    }

}