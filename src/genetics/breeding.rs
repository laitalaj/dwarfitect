//! This module contains functions that manipulate populations (vectors of
//! chromosomes). Functionality such as control of who mates who is found here.

use rand::Rng;
use std::cmp::Ordering;
use std::cmp::Ordering::Equal;
use std::fmt::{Debug, Formatter, Result};
use super::genes::{Gene, Target, Chromosome};
use collections::Vector;

/// Percentage of population that should be kept alive for the next round of
/// breeding
pub const KEEP_ALIVE_PERCENTAGE: f32 = 0.1;
/// Percentage of population to kill when purging a population
pub const PURGE_PERCENTAGE: f32 = 0.7;
/// How many generations of stagnation before purge
pub const PURGE_INTERVAL: usize = 100;

/// Candidate is a container for a chromosome with a determined probability
/// of selection for breeding
#[derive(PartialEq)]
pub struct Candidate<'a> {
    prob_range_end: f32,
    pub chromosome: &'a Chromosome,
}

impl<'a> Debug for Candidate<'a> {
  /// Debug output formatting for candidate
  fn fmt(&self, f: &mut Formatter) -> Result {
    write!(f, "Candidate with prob range end {}", self.prob_range_end)
  }
}

impl<'a> Candidate<'a> {
    /// Constructor for candidate
    fn new(prob_range_end: f32, chromosome: &'a Chromosome) -> Candidate {
        Candidate {
            prob_range_end: prob_range_end,
            chromosome: chromosome,
        }
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
pub fn search_candidate<'a>(candidates: &'a Vector<Candidate>, random_value: f32)
-> Option<&'a Candidate<'a>> {
    let mut smallest_match: Option<&Candidate> = None;
    let mut min = 0;
    let mut max = candidates.len() - 1;
    while min <= max {
        let candidate = &candidates[(min + max) / 2];
        if candidate.prob_range_end < random_value {
            min = (min + max) / 2 + 1;
        } else {
            smallest_match = Some(candidate);
            match max {
              0 => break, //Avoid infinite loop
              1 => max = 0, //If min is 0 and max is 1 (min+max)/2-1 is -1 -> can't do that!
              _ => max = (min + max) / 2 - 1
            };
        }
    }
//    if smallest_match == None {
//      println!("{}", random_value);
//      println!("{}, {}", min, max);
//      println!("{:?}", candidates);
//    }
    smallest_match
}

/// Generates an initial population with determined size
pub fn generate_initial_population<R: Rng>(genes: Vector<Gene>, 
	targets: Vector<Target>, size: usize, rng: &mut R) -> Vector<Chromosome> {
    let mut population: Vector<Chromosome> = Vector::new();
    for _ in 0..size {
        population.push(Chromosome::generate_initial(genes.clone(), 
        		targets.clone(), rng));
    }
    population
}
	
/// Returns the most fit chromosome in a population
pub fn most_fit(population: &Vector<Chromosome>) -> Option<&Chromosome> {
	let mut most_fit = None;
	let mut largest_fitness = 0.0;
	for i in 0..population.len() {
		if population[i].fitness > largest_fitness {
			most_fit = Some(&population[i]);
			largest_fitness = population[i].fitness;
		}
	}
	most_fit
}

/// Breeds a population by 1 step; generates and mutates children and returns
/// the next population.
pub fn breed<R: Rng>(population: Vector<Chromosome>, rng: &mut R)
-> Vector<Chromosome> {
    let mut total_fitness = 0.0;
    let mut work_population = population.to_vec();
    work_population.sort_by(|a, b| a.partial_cmp(b).unwrap_or(Equal));
    work_population.reverse();
    for i in 0..work_population.len() {
        total_fitness += work_population[i].fitness;
    }
    let mut candidates: Vector<Candidate> = Vector::new();
    let mut current_prob_range_end = 0.0;
    for i in 0..work_population.len() {
        if i == work_population.len() - 1 { // Make sure float inaccuracy doesn't destroy things
          current_prob_range_end = 1.0;
        } else {
          current_prob_range_end += work_population[i].fitness / total_fitness;
        }
        candidates
        .push(Candidate::new(current_prob_range_end, &work_population[i]));
    }
    let mut next_population: Vector<Chromosome> = Vector::new(); //TODO: Keeping the best of previous population
    let keep_alive = work_population.len() as f32 * KEEP_ALIVE_PERCENTAGE;
    let keep_alive_usize = keep_alive.round() as usize;
    for i in 0..keep_alive_usize {
      next_population.push(work_population[i].clone());
    }
    while next_population.len() < population.len() {
        let candidate1 = search_candidate(&candidates, rng.next_f32());
        // TODO: Handling None
        let candidate2 = search_candidate(&candidates, rng.next_f32());
        let actual_candidate1 = candidate1.unwrap();
        let actual_candidate2 = candidate2.unwrap();
        let chromosome1 = actual_candidate1.chromosome;
        let chromosome2 = actual_candidate2.chromosome;
        // TODO: Avoiding duplicates
        let (mut child1, mut child2) = chromosome1.mate(chromosome2, rng);
    child1.mutate(rng);
    child2.mutate(rng);
        next_population.push(child1);
        if next_population.len() < population.len() {
          next_population.push(child2);
        }
    }
    next_population
}

/// Replaces number of worst chromosomes equal to PURGE_PERCENTAGE with initial
/// chromosomes.
pub fn purge<R: Rng>(population: &mut Vector<Chromosome>, rng: &mut R) {
  population.sort_by(|a, b| a.partial_cmp(b).unwrap_or(Equal));
    population.reverse(); //TODO: Get rid of excess sorts
  let kill = population.len() as f32 * PURGE_PERCENTAGE;
    let kill_usize = kill.round() as usize;
    let genes = population[0].genes.clone();
    let targets = population[0].targets.clone();
    for i in kill_usize..population.len() {
      population[i] = Chromosome::generate_initial(genes.clone(), 
      	targets.clone(), rng);
    }
}

/// Breeds population for given number of generations
pub fn breed_for<R: Rng>(population: Vector<Chromosome>, generations: usize, 
	rng: &mut R) -> Vector<Chromosome> {
	let mut last_fitness = 0.0;
	let mut work_population = population.clone();
	let mut purge_imminent = false;
	for i in 0..generations {
		work_population = breed(work_population, rng);
		if i % PURGE_INTERVAL == 0 {
			let most_fit = most_fit(&work_population).unwrap();
			print!("Generation {}/{}, largest fitness: {}", i, generations,
			most_fit.fitness);
			if most_fit.fitness > last_fitness {
				last_fitness = most_fit.fitness;
			} else {
				purge_imminent = true;
			} 
			if !purge_imminent {
				println!("");
			}
		}
		if purge_imminent {
			println!(" -> Purging stale population");
			purge(&mut work_population, rng);
			purge_imminent = false;
		}
	}
	work_population
}
	
/// Creates a population of given size from genes and targets and breeds it for
/// given amount of generations.
/// # Panics
/// Panics if can't for some reason find a most fit chromosomes.
pub fn breeder<R: Rng>(genes: Vector<Gene>, targets: Vector<Target>, 
	pop_size: usize, generations: usize, rng: &mut R) -> Chromosome {
	let mut population = generate_initial_population(
		genes, targets, pop_size, rng
	);
	population = breed_for(population, generations, rng);
	match most_fit(&population) {
		None => panic!("Couldn't find most fit chromosome!"),
		Some(chromosome) => chromosome.clone()
	}
}

#[cfg(test)]
mod tests {

    use super::*;
    use mapping::shapes::Rect;
    use genetics::genes::{Gene, Chromosome};
    use collections::Vector;
    use rand;

    #[test]
    fn breed_returns_correctly_sized_population() {
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
        let mut genes = Vector::new();
        genes.push(gene1);
        genes.push(gene2);
        genes.push(gene3);
        genes.push(gene4);
        let initial_pop = generate_initial_population(
          genes, Vector::new(), 100, &mut rng
        );
        assert_eq!(100, initial_pop.len());
        let next_pop = breed(initial_pop, &mut rng);
        assert_eq!(100, next_pop.len());
    }

    #[test]
    fn search_candidate_finds_correct_candidate() {
      let rect = Rect { x:0, y:0, w:2, h:2 };
      let mut dummy_genes = Vector::new();
      dummy_genes.push(Gene::new(rect, 1));
      let mut chromosome = Chromosome::new(dummy_genes, Vector::new());
      let mut chromosomes = Vector::new();
      let mut candidates = Vector::new();
      for _ in 0..10 {
        let new_chromosome = chromosome.clone();
        chromosomes.push(new_chromosome);
        chromosome.genes.push(Gene::new(rect, 1));
      }
      let mut prob = 0.0;
      for i in 0..chromosomes.len() {
        prob += 1.0 / chromosomes.len() as f32;
        candidates.push(Candidate::new(prob, &chromosomes[i]));
      }
      let found_candidate = search_candidate(&candidates, 0.75).unwrap();
      assert_eq!(8, found_candidate.chromosome.genes.len());
    }

}
