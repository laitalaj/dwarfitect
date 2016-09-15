use std::vec::Vec;
use std::collections::HashMap;
use rand::Rng;
use rand::distributions::{IndependentSample, Range};
use mapping::shapes::Rect;

//TODO: Get rid of this hardcoding
pub const CROSSOVER_CHANCE: f32 = 0.7;
pub const MUTATION_CHANCE: f32 = 0.01;

/// Genes are rooms represented only by their bounding rectangle. A chromosome
/// is made of these.
#[derive(PartialEq, Eq, Copy, Clone, Debug)]
pub struct Gene {
  rect: Rect,
  gene_id: i16
}

/// Chromosomes are possible solutions. They handle the genetic operations.
pub struct Chromosome {
  genes: Vec<Gene>,
  fitness: f32
}

impl Gene {

  /// Rotates the gene (switches it's rectangles width with its height)
  /// Returns a new, rotated gene
  fn rotate(&self) -> Gene {
    let new_rect = self.rect.rotate();
    Gene{ rect: new_rect, gene_id: self.gene_id }
  }
  /// Rotates the gene in place (switches it's rectangles width with its
  /// height). Only works for mutable genes.
  fn rot_in_place(&mut self) {
    self.rect = self.rect.rotate();
  }
  fn get_x(&self) -> i16 {
    self.rect.x
  }
  fn get_y(&self) -> i16 {
    self.rect.y
  }
  fn get_w(&self) -> i16 {
    self.rect.w
  }
  fn get_h(&self) -> i16 {
    self.rect.h
  }
  /// Creates a new gene that's a copy of this one but with a differing
  /// position.
  fn set_pos(&self, new_x: i16, new_y: i16) -> Gene {
    let new_rect = Rect { x: new_x, y: new_y,
      w: self.rect.w, h: self.rect.h };
    Gene { rect: new_rect, gene_id: self.gene_id }
  }
  /// Sets the X position of the gene. Only works if the gene is mutable
  fn set_x(&mut self, x: i16) {
    self.rect.x = x;
  }
  /// Sets the Y position of the gene. Only works if the gene is mutable
  fn set_y(&mut self, y: i16) {
    self.rect.y = y;
  }

}

impl Chromosome {
	
	/// A constructor for the chromosome
	pub fn new(genes: Vec<Gene>) -> Chromosome{
		Chromosome{ genes: genes, fitness: 0.0 } //TODO: Fitness calculation, cleanliness
	}
	
	/// The mating function: Two children are created by swapping half of this 
	/// chromosomes genes with the partner chromosome's genes (aka. crossover)
	/// # Panics
	/// Panics if trying to mate two genes of different lengths!
	/// (Doing that is contrary to both natural evolution AND the word of God!)
	pub fn mate<R: Rng>(&self, partner: &Chromosome, rng: &mut R) 
	-> (Chromosome, Chromosome) {
		if self.genes.len() != partner.genes.len() {
			panic!("Tried to mate chromosomes with different lengths! 
			Shame on you!");
		}
		let mut my_childs_genes: Vec<Gene> = Vec::new();
		let mut partners_childs_genes: Vec<Gene> = Vec::new();
		for i in 0..self.genes.len() {
			if rng.next_f32() < CROSSOVER_CHANCE {
				partners_childs_genes.push(self.genes[i]);
				my_childs_genes.push(partner.genes[i]);
			} else {
				my_childs_genes.push(self.genes[i]);
				partners_childs_genes.push(partner.genes[i]);
			}
		}
		let my_child = Chromosome::new(my_childs_genes);
		let partners_child = Chromosome::new(partners_childs_genes);
		(my_child, partners_child)
	}
}



#[cfg(test)]
mod tests {

  use super::*;
  use mapping::shapes::Rect;
  use rand::Rng;
  
  struct TestRng {
  	numbers: Vec<f32>
  }
  
  impl Rng for TestRng {
  	fn next_u32(&mut self) -> u32 {
  		panic!("Not supported!");
  	}
  	fn next_f32(&mut self) -> f32 {
  		match self.numbers.pop() {
  			Some(n) => n,
  			None => panic!("Ran out of numbers!")
  		}
  	}
  }

  #[test]
  fn gene_rotates_correctly() {
    let rect1 = Rect { x: 2, y: 3, w: 5, h:7 };
    let gene1 = Gene { rect: rect1, gene_id: 0 };
    let gene2 = gene1.rotate();
    assert_eq!(7, gene2.get_w());
    assert_eq!(5, gene2.get_h());
    let rect2 = Rect { x: -2, y: -3, w: 1, h:9 };
    let mut gene3 = Gene { rect: rect2, gene_id: 0 };
    gene3.rot_in_place();
    assert_eq!(9, gene3.get_w());
    assert_eq!(1, gene3.get_h());
  }

  #[test]
  fn position_changing_ok() {
    let rect1 = Rect { x: 2, y: 3, w: 5, h:7 };
    let mut gene1 = Gene { rect: rect1, gene_id: 0 };
    gene1.set_x(9);
    gene1.set_y(10);
    assert_eq!(9, gene1.get_x());
    assert_eq!(10, gene1.get_y());
  }
  
  #[test]
  fn mating_works_correctly() {
  	let rect1 = Rect { x: 2, y: 3, w: 5, h: 7 };
    let gene1 = Gene { rect: rect1, gene_id: 0 };
    let rect2 = Rect { x: 1, y: 0, w: 3, h: 3 };
    let gene2 = Gene { rect: rect2, gene_id: 1};
    let mut gene3 = gene1;
    gene3.set_x(4);
    let mut gene4 = gene2;
    gene4.set_y(5);
    let genes1 = vec![gene1, gene2];
    let genes2 = vec![gene3, gene4];
    let chrom1 = Chromosome::new(genes1);
    let chrom2 = Chromosome::new(genes2);
    let crossover_delta = CROSSOVER_CHANCE * 0.1;
    let random_numbers = vec![CROSSOVER_CHANCE - crossover_delta,
    CROSSOVER_CHANCE + crossover_delta];
    let mut rng = TestRng{ numbers: random_numbers };
    let (child1, child2) = chrom1.mate(&chrom2, &mut rng);
    assert_eq!(gene1, child1.genes[0]);
    assert_eq!(gene4, child1.genes[1]);
    assert_eq!(gene3, child2.genes[0]);
    assert_eq!(gene2, child2.genes[1]);
  }
  
}
